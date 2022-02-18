pub trait IEnumSpObjectTokens_Impl: Sized {
    fn Next(&self, celt: u32, pelt: *mut ::core::option::Option<ISpObjectToken>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSpObjectTokens>;
    fn Item(&self, index: u32) -> ::windows::core::Result<ISpObjectToken>;
    fn GetCount(&self, pcount: *mut u32) -> ::windows::core::Result<()>;
}
impl IEnumSpObjectTokens_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>() -> IEnumSpObjectTokens_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSpObjectTokens as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpAudio_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl {
    fn SetState(&self, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::Result<()>;
    fn SetFormat(&self, rguidfmtid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::Result<()>;
    fn SetBufferInfo(&self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::Result<()>;
    fn GetBufferInfo(&self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::Result<()>;
    fn GetDefaultFormat(&self, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn EventHandle(&self) -> super::super::Foundation::HANDLE;
    fn GetVolumeLevel(&self, plevel: *mut u32) -> ::windows::core::Result<()>;
    fn SetVolumeLevel(&self, level: u32) -> ::windows::core::Result<()>;
    fn GetBufferNotifySize(&self, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetBufferNotifySize(&self, cbsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>() -> ISpAudio_Vtbl {
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&newstate), ::core::mem::transmute_copy(&ullreserved)).into()
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidfmtid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&rguidfmtid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn SetBufferInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferInfo(::core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetBufferInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferInfo(::core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetDefaultFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDefaultFormat(::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwaveformatex)).into()
        }
        unsafe extern "system" fn EventHandle<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EventHandle()
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVolumeLevel(::core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn SetVolumeLevel<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolumeLevel(::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn GetBufferNotifySize<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferNotifySize(::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferNotifySize(::core::mem::transmute_copy(&cbsize)).into()
        }
        Self {
            base: ISpStreamFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetState: SetState::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetBufferInfo: SetBufferInfo::<Identity, Impl, OFFSET>,
            GetBufferInfo: GetBufferInfo::<Identity, Impl, OFFSET>,
            GetDefaultFormat: GetDefaultFormat::<Identity, Impl, OFFSET>,
            EventHandle: EventHandle::<Identity, Impl, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, Impl, OFFSET>,
            SetVolumeLevel: SetVolumeLevel::<Identity, Impl, OFFSET>,
            GetBufferNotifySize: GetBufferNotifySize::<Identity, Impl, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpAudio as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID || iid == &<ISpStreamFormat as ::windows::core::Interface>::IID
    }
}
pub trait ISpContainerLexicon_Impl: Sized + ISpLexicon_Impl {
    fn AddLexicon(&self, paddlexicon: &::core::option::Option<ISpLexicon>, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ISpContainerLexicon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpContainerLexicon_Impl, const OFFSET: isize>() -> ISpContainerLexicon_Vtbl {
        unsafe extern "system" fn AddLexicon<Identity: ::windows::core::IUnknownImpl, Impl: ISpContainerLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddlexicon: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLexicon(::core::mem::transmute(&paddlexicon), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ISpLexicon_Vtbl::new::<Identity, Impl, OFFSET>(), AddLexicon: AddLexicon::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpContainerLexicon as ::windows::core::Interface>::IID || iid == &<ISpLexicon as ::windows::core::Interface>::IID
    }
}
pub trait ISpDataKey_Impl: Sized {
    fn SetData(&self, pszvaluename: &::windows::core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows::core::Result<()>;
    fn GetData(&self, pszvaluename: &::windows::core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::Result<()>;
    fn SetStringValue(&self, pszvaluename: &::windows::core::PCWSTR, pszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetStringValue(&self, pszvaluename: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetDWORD(&self, pszvaluename: &::windows::core::PCWSTR, dwvalue: u32) -> ::windows::core::Result<()>;
    fn GetDWORD(&self, pszvaluename: &::windows::core::PCWSTR, pdwvalue: *mut u32) -> ::windows::core::Result<()>;
    fn OpenKey(&self, pszsubkeyname: &::windows::core::PCWSTR) -> ::windows::core::Result<ISpDataKey>;
    fn CreateKey(&self, pszsubkey: &::windows::core::PCWSTR) -> ::windows::core::Result<ISpDataKey>;
    fn DeleteKey(&self, pszsubkey: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn DeleteValue(&self, pszvaluename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn EnumKeys(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn EnumValues(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ISpDataKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>() -> ISpDataKey_Vtbl {
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, pszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute(&pszvaluename), ::core::mem::transmute(&pszvalue)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute(&pszvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDWORD<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDWORD(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn GetDWORD<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, pdwvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDWORD(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&pdwvalue)).into()
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkeyname: ::windows::core::PCWSTR, ppsubkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenKey(::core::mem::transmute(&pszsubkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: ::windows::core::PCWSTR, ppsubkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateKey(::core::mem::transmute(&pszsubkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteKey(::core::mem::transmute(&pszsubkey)).into()
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteValue(::core::mem::transmute(&pszvaluename)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszsubkeyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumKeys(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszsubkeyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszvaluename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumValues(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszvaluename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetDWORD: SetDWORD::<Identity, Impl, OFFSET>,
            GetDWORD: GetDWORD::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CreateKey: CreateKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            EnumValues: EnumValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpDataKey as ::windows::core::Interface>::IID
    }
}
pub trait ISpDisplayAlternates_Impl: Sized {
    fn GetDisplayAlternates(&self, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn SetFullStopTrailSpace(&self, ultrailspace: u32) -> ::windows::core::Result<()>;
}
impl ISpDisplayAlternates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>() -> ISpDisplayAlternates_Vtbl {
        unsafe extern "system" fn GetDisplayAlternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayAlternates(::core::mem::transmute_copy(&pphrase), ::core::mem::transmute_copy(&crequestcount), ::core::mem::transmute_copy(&ppcomemphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn SetFullStopTrailSpace<Identity: ::windows::core::IUnknownImpl, Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultrailspace: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFullStopTrailSpace(::core::mem::transmute_copy(&ultrailspace)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDisplayAlternates: GetDisplayAlternates::<Identity, Impl, OFFSET>,
            SetFullStopTrailSpace: SetFullStopTrailSpace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpDisplayAlternates as ::windows::core::Interface>::IID
    }
}
pub trait ISpEnginePronunciation_Impl: Sized {
    fn Normalize(&self, pszword: &::windows::core::PCWSTR, pszleftcontext: &::windows::core::PCWSTR, pszrightcontext: &::windows::core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows::core::Result<()>;
    fn GetPronunciations(&self, pszword: &::windows::core::PCWSTR, pszleftcontext: &::windows::core::PCWSTR, pszrightcontext: &::windows::core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::Result<()>;
}
impl ISpEnginePronunciation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>() -> ISpEnginePronunciation_Vtbl {
        unsafe extern "system" fn Normalize<Identity: ::windows::core::IUnknownImpl, Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, pszleftcontext: ::windows::core::PCWSTR, pszrightcontext: ::windows::core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Normalize(::core::mem::transmute(&pszword), ::core::mem::transmute(&pszleftcontext), ::core::mem::transmute(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pnormalizationlist)).into()
        }
        unsafe extern "system" fn GetPronunciations<Identity: ::windows::core::IUnknownImpl, Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, pszleftcontext: ::windows::core::PCWSTR, pszrightcontext: ::windows::core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPronunciations(::core::mem::transmute(&pszword), ::core::mem::transmute(&pszleftcontext), ::core::mem::transmute(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&penginepronunciationlist)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEnginePronunciation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSink_Impl: Sized {
    fn AddEvents(&self, peventarray: *const SPEVENT, ulcount: u32) -> ::windows::core::Result<()>;
    fn GetEventInterest(&self, pulleventinterest: *mut u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSink_Impl, const OFFSET: isize>() -> ISpEventSink_Vtbl {
        unsafe extern "system" fn AddEvents<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventarray: *const SPEVENT, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddEvents(::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEventInterest(::core::mem::transmute_copy(&pulleventinterest)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddEvents: AddEvents::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource_Impl: Sized + ISpNotifySource_Impl {
    fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::Result<()>;
    fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::Result<()>;
    fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource_Impl, const OFFSET: isize>() -> ISpEventSource_Vtbl {
        unsafe extern "system" fn SetInterest<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterest(::core::mem::transmute_copy(&ulleventinterest), ::core::mem::transmute_copy(&ullqueuedinterest)).into()
        }
        unsafe extern "system" fn GetEvents<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEvents(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base: ISpNotifySource_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetInterest: SetInterest::<Identity, Impl, OFFSET>,
            GetEvents: GetEvents::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEventSource as ::windows::core::Interface>::IID || iid == &<ISpNotifySource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource2_Impl: Sized + ISpNotifySource_Impl + ISpEventSource_Impl {
    fn GetEventsEx(&self, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource2_Impl, const OFFSET: isize>() -> ISpEventSource2_Vtbl {
        unsafe extern "system" fn GetEventsEx<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEventsEx(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        Self { base: ISpEventSource_Vtbl::new::<Identity, Impl, OFFSET>(), GetEventsEx: GetEventsEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEventSource2 as ::windows::core::Interface>::IID || iid == &<ISpNotifySource as ::windows::core::Interface>::IID || iid == &<ISpEventSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpGrammarBuilder_Impl: Sized {
    fn ResetGrammar(&self, newlanguage: u16) -> ::windows::core::Result<()>;
    fn GetRule(&self, pszrulename: &::windows::core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>;
    fn ClearRule(&self, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>;
    fn CreateNewState(&self, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>;
    fn AddWordTransition(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: &::windows::core::PCWSTR, pszseparators: &::windows::core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()>;
    fn AddRuleTransition(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()>;
    fn AddResource(&self, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: &::windows::core::PCWSTR, pszresourcevalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Commit(&self, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpGrammarBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>() -> ISpGrammarBuilder_Vtbl {
        unsafe extern "system" fn ResetGrammar<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetGrammar(::core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn GetRule<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows::core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRule(::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&dwruleid), ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&fcreateifnotexist), ::core::mem::transmute_copy(&phinitialstate)).into()
        }
        unsafe extern "system" fn ClearRule<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRule(::core::mem::transmute_copy(&hstate)).into()
        }
        unsafe extern "system" fn CreateNewState<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNewState(::core::mem::transmute_copy(&hstate), ::core::mem::transmute_copy(&phstate)).into()
        }
        unsafe extern "system" fn AddWordTransition<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: ::windows::core::PCWSTR, pszseparators: ::windows::core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddWordTransition(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute(&psz), ::core::mem::transmute(&pszseparators), ::core::mem::transmute_copy(&ewordtype), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRuleTransition(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute_copy(&hrule), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddResource<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: ::windows::core::PCWSTR, pszresourcevalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddResource(::core::mem::transmute_copy(&hrulestate), ::core::mem::transmute(&pszresourcename), ::core::mem::transmute(&pszresourcevalue)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ResetGrammar: ResetGrammar::<Identity, Impl, OFFSET>,
            GetRule: GetRule::<Identity, Impl, OFFSET>,
            ClearRule: ClearRule::<Identity, Impl, OFFSET>,
            CreateNewState: CreateNewState::<Identity, Impl, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, Impl, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpGrammarBuilder as ::windows::core::Interface>::IID
    }
}
pub trait ISpGrammarBuilder2_Impl: Sized {
    fn AddTextSubset(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: &::windows::core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows::core::Result<()>;
    fn SetPhoneticAlphabet(&self, phoneticalphabet: PHONETICALPHABET) -> ::windows::core::Result<()>;
}
impl ISpGrammarBuilder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>() -> ISpGrammarBuilder2_Vtbl {
        unsafe extern "system" fn AddTextSubset<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: ::windows::core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTextSubset(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute(&psz), ::core::mem::transmute_copy(&ematchmode)).into()
        }
        unsafe extern "system" fn SetPhoneticAlphabet<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneticalphabet: PHONETICALPHABET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPhoneticAlphabet(::core::mem::transmute_copy(&phoneticalphabet)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddTextSubset: AddTextSubset::<Identity, Impl, OFFSET>,
            SetPhoneticAlphabet: SetPhoneticAlphabet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpGrammarBuilder2 as ::windows::core::Interface>::IID
    }
}
pub trait ISpLexicon_Impl: Sized {
    fn GetPronunciations(&self, pszword: &::windows::core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::Result<()>;
    fn AddPronunciation(&self, pszword: &::windows::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::Result<()>;
    fn RemovePronunciation(&self, pszword: &::windows::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::Result<()>;
    fn GetGeneration(&self, pdwgeneration: *mut u32) -> ::windows::core::Result<()>;
    fn GetGenerationChange(&self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
    fn GetWords(&self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
}
impl ISpLexicon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const OFFSET: isize>() -> ISpLexicon_Vtbl {
        unsafe extern "system" fn GetPronunciations<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPronunciations(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwordpronunciationlist)).into()
        }
        unsafe extern "system" fn AddPronunciation<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPronunciation(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePronunciation(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn GetGeneration<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGeneration(::core::mem::transmute_copy(&pdwgeneration)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGenerationChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWords(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, Impl, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, Impl, OFFSET>,
            GetGeneration: GetGeneration::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpLexicon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpMMSysAudio_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl + ISpAudio_Impl {
    fn GetDeviceId(&self, pudeviceid: *mut u32) -> ::windows::core::Result<()>;
    fn SetDeviceId(&self, udeviceid: u32) -> ::windows::core::Result<()>;
    fn GetMMHandle(&self, phandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetLineId(&self, pulineid: *mut u32) -> ::windows::core::Result<()>;
    fn SetLineId(&self, ulineid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpMMSysAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>() -> ISpMMSysAudio_Vtbl {
        unsafe extern "system" fn GetDeviceId<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudeviceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceId(::core::mem::transmute_copy(&pudeviceid)).into()
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udeviceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDeviceId(::core::mem::transmute_copy(&udeviceid)).into()
        }
        unsafe extern "system" fn GetMMHandle<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMMHandle(::core::mem::transmute_copy(&phandle)).into()
        }
        unsafe extern "system" fn GetLineId<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulineid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLineId(::core::mem::transmute_copy(&pulineid)).into()
        }
        unsafe extern "system" fn SetLineId<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulineid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLineId(::core::mem::transmute_copy(&ulineid)).into()
        }
        Self {
            base: ISpAudio_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            GetMMHandle: GetMMHandle::<Identity, Impl, OFFSET>,
            GetLineId: GetLineId::<Identity, Impl, OFFSET>,
            SetLineId: SetLineId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpMMSysAudio as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID || iid == &<ISpStreamFormat as ::windows::core::Interface>::IID || iid == &<ISpAudio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyCallback_Impl: Sized {
    fn NotifyCallback(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyCallback_Impl, const OFFSET: isize>() -> ISpNotifyCallback_Vtbl {
        unsafe extern "system" fn NotifyCallback<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyCallback(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self { NotifyCallback: NotifyCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifyCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISpNotifySink_Impl: Sized {
    fn Notify(&self) -> ::windows::core::Result<()>;
}
impl ISpNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySink_Impl, const OFFSET: isize>() -> ISpNotifySink_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifySource_Impl: Sized {
    fn SetNotifySink(&self, pnotifysink: &::core::option::Option<ISpNotifySink>) -> ::windows::core::Result<()>;
    fn SetNotifyWindowMessage(&self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetNotifyCallbackFunction(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetNotifyCallbackInterface(&self, pspcallback: &::core::option::Option<ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetNotifyWin32Event(&self) -> ::windows::core::Result<()>;
    fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifySource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>() -> ISpNotifySource_Vtbl {
        unsafe extern "system" fn SetNotifySink<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifySink(::core::mem::transmute(&pnotifysink)).into()
        }
        unsafe extern "system" fn SetNotifyWindowMessage<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyWindowMessage(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackFunction<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyCallbackFunction(::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackInterface<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyCallbackInterface(::core::mem::transmute(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyWin32Event<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyWin32Event().into()
        }
        unsafe extern "system" fn WaitForNotifyEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForNotifyEvent(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetNotifyEventHandle<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNotifyEventHandle()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetNotifySink: SetNotifySink::<Identity, Impl, OFFSET>,
            SetNotifyWindowMessage: SetNotifyWindowMessage::<Identity, Impl, OFFSET>,
            SetNotifyCallbackFunction: SetNotifyCallbackFunction::<Identity, Impl, OFFSET>,
            SetNotifyCallbackInterface: SetNotifyCallbackInterface::<Identity, Impl, OFFSET>,
            SetNotifyWin32Event: SetNotifyWin32Event::<Identity, Impl, OFFSET>,
            WaitForNotifyEvent: WaitForNotifyEvent::<Identity, Impl, OFFSET>,
            GetNotifyEventHandle: GetNotifyEventHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifySource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyTranslator_Impl: Sized + ISpNotifySink_Impl {
    fn InitWindowMessage(&self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InitCallback(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InitSpNotifyCallback(&self, pspcallback: &::core::option::Option<ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InitWin32Event(&self, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Wait(&self, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn GetEventHandle(&self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyTranslator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>() -> ISpNotifyTranslator_Vtbl {
        unsafe extern "system" fn InitWindowMessage<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitWindowMessage(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitCallback<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitCallback(::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitSpNotifyCallback<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitSpNotifyCallback(::core::mem::transmute(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitWin32Event<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitWin32Event(::core::mem::transmute_copy(&hevent), ::core::mem::transmute_copy(&fclosehandleonrelease)).into()
        }
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Wait(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetEventHandle<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEventHandle()
        }
        Self {
            base: ISpNotifySink_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitWindowMessage: InitWindowMessage::<Identity, Impl, OFFSET>,
            InitCallback: InitCallback::<Identity, Impl, OFFSET>,
            InitSpNotifyCallback: InitSpNotifyCallback::<Identity, Impl, OFFSET>,
            InitWin32Event: InitWin32Event::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            GetEventHandle: GetEventHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifyTranslator as ::windows::core::Interface>::IID || iid == &<ISpNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectToken_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&self, pszcategoryid: &::windows::core::PCWSTR, psztokenid: &::windows::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetCategory(&self) -> ::windows::core::Result<ISpObjectTokenCategory>;
    fn CreateInstance(&self, punkouter: &::core::option::Option<::windows::core::IUnknown>, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetStorageFileName(&self, clsidcaller: *const ::windows::core::GUID, pszvaluename: &::windows::core::PCWSTR, pszfilenamespecifier: &::windows::core::PCWSTR, nfolder: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn RemoveStorageFileName(&self, clsidcaller: *const ::windows::core::GUID, pszkeyname: &::windows::core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Remove(&self, pclsidcaller: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn IsUISupported(&self, psztypeofui: &::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: &::core::option::Option<::windows::core::IUnknown>, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &::windows::core::PCWSTR, psztypeofui: &::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MatchesAttributes(&self, pszattributes: &::windows::core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectToken_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>() -> ISpObjectToken_Vtbl {
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows::core::PCWSTR, psztokenid: ::windows::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute(&pszcategoryid), ::core::mem::transmute(&psztokenid), ::core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemtokenid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptokencategory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pptokencategory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateInstance(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows::core::GUID, pszvaluename: ::windows::core::PCWSTR, pszfilenamespecifier: ::windows::core::PCWSTR, nfolder: u32, ppszfilepath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStorageFileName(::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute(&pszvaluename), ::core::mem::transmute(&pszfilenamespecifier), ::core::mem::transmute_copy(&nfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszfilepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows::core::GUID, pszkeyname: ::windows::core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStorageFileName(::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute(&pszkeyname), ::core::mem::transmute_copy(&fdeletefile)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidcaller: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&pclsidcaller)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUISupported(::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute(&punkobject), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute(&punkobject)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributes: ::windows::core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MatchesAttributes(::core::mem::transmute(&pszattributes), ::core::mem::transmute_copy(&pfmatches)).into()
        }
        Self {
            base: ISpDataKey_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, Impl, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectToken as ::windows::core::Interface>::IID || iid == &<ISpDataKey as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenCategory_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&self, pszcategoryid: &::windows::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDataKey(&self, spdkl: SPDATAKEYLOCATION) -> ::windows::core::Result<ISpDataKey>;
    fn EnumTokens(&self, pzsreqattribs: &::windows::core::PCWSTR, pszoptattribs: &::windows::core::PCWSTR) -> ::windows::core::Result<IEnumSpObjectTokens>;
    fn SetDefaultTokenId(&self, psztokenid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetDefaultTokenId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>() -> ISpObjectTokenCategory_Vtbl {
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute(&pszcategoryid), ::core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategoryid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemcategoryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataKey(::core::mem::transmute_copy(&spdkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdatakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTokens<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pzsreqattribs: ::windows::core::PCWSTR, pszoptattribs: ::windows::core::PCWSTR, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumTokens(::core::mem::transmute(&pzsreqattribs), ::core::mem::transmute(&pszoptattribs)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTokenId<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztokenid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultTokenId(::core::mem::transmute(&psztokenid)).into()
        }
        unsafe extern "system" fn GetDefaultTokenId<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultTokenId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemtokenid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpDataKey_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetDataKey: GetDataKey::<Identity, Impl, OFFSET>,
            EnumTokens: EnumTokens::<Identity, Impl, OFFSET>,
            SetDefaultTokenId: SetDefaultTokenId::<Identity, Impl, OFFSET>,
            GetDefaultTokenId: GetDefaultTokenId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectTokenCategory as ::windows::core::Interface>::IID || iid == &<ISpDataKey as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenInit_Impl: Sized + ISpDataKey_Impl + ISpObjectToken_Impl {
    fn InitFromDataKey(&self, pszcategoryid: &::windows::core::PCWSTR, psztokenid: &::windows::core::PCWSTR, pdatakey: &::core::option::Option<ISpDataKey>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectTokenInit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenInit_Impl, const OFFSET: isize>() -> ISpObjectTokenInit_Vtbl {
        unsafe extern "system" fn InitFromDataKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows::core::PCWSTR, psztokenid: ::windows::core::PCWSTR, pdatakey: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitFromDataKey(::core::mem::transmute(&pszcategoryid), ::core::mem::transmute(&psztokenid), ::core::mem::transmute(&pdatakey)).into()
        }
        Self { base: ISpObjectToken_Vtbl::new::<Identity, Impl, OFFSET>(), InitFromDataKey: InitFromDataKey::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectTokenInit as ::windows::core::Interface>::IID || iid == &<ISpDataKey as ::windows::core::Interface>::IID || iid == &<ISpObjectToken as ::windows::core::Interface>::IID
    }
}
pub trait ISpObjectWithToken_Impl: Sized {
    fn SetObjectToken(&self, ptoken: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn GetObjectToken(&self) -> ::windows::core::Result<ISpObjectToken>;
}
impl ISpObjectWithToken_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectWithToken_Impl, const OFFSET: isize>() -> ISpObjectWithToken_Vtbl {
        unsafe extern "system" fn SetObjectToken<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectToken(::core::mem::transmute(&ptoken)).into()
        }
        unsafe extern "system" fn GetObjectToken<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetObjectToken: SetObjectToken::<Identity, Impl, OFFSET>,
            GetObjectToken: GetObjectToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectWithToken as ::windows::core::Interface>::IID
    }
}
pub trait ISpPhoneConverter_Impl: Sized + ISpObjectWithToken_Impl {
    fn PhoneToId(&self, pszphone: &::windows::core::PCWSTR) -> ::windows::core::Result<u16>;
    fn IdToPhone(&self, pid: *const u16, pszphone: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ISpPhoneConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneConverter_Impl, const OFFSET: isize>() -> ISpPhoneConverter_Vtbl {
        unsafe extern "system" fn PhoneToId<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszphone: ::windows::core::PCWSTR, pid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhoneToId(::core::mem::transmute(&pszphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *const u16, pszphone: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IdToPhone(::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pszphone)).into()
        }
        Self {
            base: ISpObjectWithToken_Vtbl::new::<Identity, Impl, OFFSET>(),
            PhoneToId: PhoneToId::<Identity, Impl, OFFSET>,
            IdToPhone: IdToPhone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhoneConverter as ::windows::core::Interface>::IID || iid == &<ISpObjectWithToken as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetConverter_Impl: Sized {
    fn GetLangId(&self) -> ::windows::core::Result<u16>;
    fn SetLangId(&self, langid: u16) -> ::windows::core::Result<()>;
    fn SAPI2UPS(&self, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows::core::Result<()>;
    fn UPS2SAPI(&self, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows::core::Result<()>;
    fn GetMaxConvertLength(&self, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpPhoneticAlphabetConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>() -> ISpPhoneticAlphabetConverter_Vtbl {
        unsafe extern "system" fn GetLangId<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLangId() {
                ::core::result::Result::Ok(ok__) => {
                    *plangid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLangId<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLangId(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn SAPI2UPS<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SAPI2UPS(::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn UPS2SAPI<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UPS2SAPI(::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn GetMaxConvertLength<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxConvertLength(::core::mem::transmute_copy(&csrclength), ::core::mem::transmute_copy(&bsapi2ups)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcmaxdestlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
            SetLangId: SetLangId::<Identity, Impl, OFFSET>,
            SAPI2UPS: SAPI2UPS::<Identity, Impl, OFFSET>,
            UPS2SAPI: UPS2SAPI::<Identity, Impl, OFFSET>,
            GetMaxConvertLength: GetMaxConvertLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetSelection_Impl: Sized {
    fn IsAlphabetUPS(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAlphabetToUPS(&self, fforceups: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpPhoneticAlphabetSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>() -> ISpPhoneticAlphabetSelection_Vtbl {
        unsafe extern "system" fn IsAlphabetUPS<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisups: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAlphabetUPS() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphabetToUPS<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceups: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphabetToUPS(::core::mem::transmute_copy(&fforceups)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsAlphabetUPS: IsAlphabetUPS::<Identity, Impl, OFFSET>,
            SetAlphabetToUPS: SetAlphabetToUPS::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetSelection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpPhrase_Impl: Sized {
    fn GetPhrase(&self) -> ::windows::core::Result<*mut SPPHRASE>;
    fn GetSerializedPhrase(&self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE>;
    fn GetText(&self, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows::core::Result<()>;
    fn Discard(&self, dwvaluetypes: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpPhrase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase_Impl, const OFFSET: isize>() -> ISpPhrase_Vtbl {
        unsafe extern "system" fn GetPhrase<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPPHRASE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedPhrase<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSerializedPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&ulstart), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&fusetextreplacements), ::core::mem::transmute_copy(&ppszcomemtext), ::core::mem::transmute_copy(&pbdisplayattributes)).into()
        }
        unsafe extern "system" fn Discard<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwvaluetypes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Discard(::core::mem::transmute_copy(&dwvaluetypes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPhrase: GetPhrase::<Identity, Impl, OFFSET>,
            GetSerializedPhrase: GetSerializedPhrase::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Discard: Discard::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhrase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpPhrase2_Impl: Sized + ISpPhrase_Impl {
    fn GetXMLResult(&self, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<()>;
    fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::Result<()>;
    fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpPhrase2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase2_Impl, const OFFSET: isize>() -> ISpPhrase2_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        unsafe extern "system" fn GetAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpPhrase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
            GetAudio: GetAudio::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhrase2 as ::windows::core::Interface>::IID || iid == &<ISpPhrase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpPhraseAlt_Impl: Sized + ISpPhrase_Impl {
    fn GetAltInfo(&self, ppparent: *mut ::core::option::Option<ISpPhrase>, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows::core::Result<()>;
    fn Commit(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpPhraseAlt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhraseAlt_Impl, const OFFSET: isize>() -> ISpPhraseAlt_Vtbl {
        unsafe extern "system" fn GetAltInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAltInfo(::core::mem::transmute_copy(&ppparent), ::core::mem::transmute_copy(&pulstartelementinparent), ::core::mem::transmute_copy(&pcelementsinparent), ::core::mem::transmute_copy(&pcelementsinalt)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: ISpPhrase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAltInfo: GetAltInfo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhraseAlt as ::windows::core::Interface>::IID || iid == &<ISpPhrase as ::windows::core::Interface>::IID
    }
}
pub trait ISpProperties_Impl: Sized {
    fn SetPropertyNum(&self, pname: &::windows::core::PCWSTR, lvalue: i32) -> ::windows::core::Result<()>;
    fn GetPropertyNum(&self, pname: &::windows::core::PCWSTR, plvalue: *mut i32) -> ::windows::core::Result<()>;
    fn SetPropertyString(&self, pname: &::windows::core::PCWSTR, pvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetPropertyString(&self, pname: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ISpProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpProperties_Impl, const OFFSET: isize>() -> ISpProperties_Vtbl {
        unsafe extern "system" fn SetPropertyNum<Identity: ::windows::core::IUnknownImpl, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPropertyNum(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn GetPropertyNum<Identity: ::windows::core::IUnknownImpl, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyNum(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&plvalue)).into()
        }
        unsafe extern "system" fn SetPropertyString<Identity: ::windows::core::IUnknownImpl, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, pvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPropertyString(::core::mem::transmute(&pname), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetPropertyString<Identity: ::windows::core::IUnknownImpl, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, ppcomemvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyString(::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetPropertyNum: SetPropertyNum::<Identity, Impl, OFFSET>,
            GetPropertyNum: GetPropertyNum::<Identity, Impl, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, Impl, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub trait ISpRecoContext_Impl: Sized + ISpNotifySource_Impl + ISpEventSource_Impl {
    fn GetRecognizer(&self) -> ::windows::core::Result<ISpRecognizer>;
    fn CreateGrammar(&self, ullgrammarid: u64) -> ::windows::core::Result<ISpRecoGrammar>;
    fn GetStatus(&self, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows::core::Result<()>;
    fn GetMaxAlternates(&self, pcalternates: *mut u32) -> ::windows::core::Result<()>;
    fn SetMaxAlternates(&self, calternates: u32) -> ::windows::core::Result<()>;
    fn SetAudioOptions(&self, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetAudioOptions(&self, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn DeserializeResult(&self, pserializedresult: *const SPSERIALIZEDRESULT) -> ::windows::core::Result<ISpRecoResult>;
    fn Bookmark(&self, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetAdaptationData(&self, padaptationdata: &::windows::core::PCWSTR, cch: u32) -> ::windows::core::Result<()>;
    fn Pause(&self, dwreserved: u32) -> ::windows::core::Result<()>;
    fn Resume(&self, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetVoice(&self, pvoice: &::core::option::Option<ISpVoice>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetVoice(&self) -> ::windows::core::Result<ISpVoice>;
    fn SetVoicePurgeEvent(&self, ulleventinterest: u64) -> ::windows::core::Result<()>;
    fn GetVoicePurgeEvent(&self, pulleventinterest: *mut u64) -> ::windows::core::Result<()>;
    fn SetContextState(&self, econtextstate: SPCONTEXTSTATE) -> ::windows::core::Result<()>;
    fn GetContextState(&self, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl ISpRecoContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>() -> ISpRecoContext_Vtbl {
        unsafe extern "system" fn GetRecognizer<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGrammar<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullgrammarid: u64, ppgrammar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGrammar(::core::mem::transmute_copy(&ullgrammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgrammar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetMaxAlternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalternates: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMaxAlternates(::core::mem::transmute_copy(&pcalternates)).into()
        }
        unsafe extern "system" fn SetMaxAlternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, calternates: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxAlternates(::core::mem::transmute_copy(&calternates)).into()
        }
        unsafe extern "system" fn SetAudioOptions<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAudioOptions(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetAudioOptions<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAudioOptions(::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn DeserializeResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeserializeResult(::core::mem::transmute_copy(&pserializedresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Bookmark(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&ullstreamposition), ::core::mem::transmute_copy(&lparamevent)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAdaptationData(::core::mem::transmute(&padaptationdata), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetVoice<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoice: ::windows::core::RawPtr, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVoice(::core::mem::transmute(&pvoice), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetVoice<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVoice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvoice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVoicePurgeEvent(::core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn GetVoicePurgeEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVoicePurgeEvent(::core::mem::transmute_copy(&pulleventinterest)).into()
        }
        unsafe extern "system" fn SetContextState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContextState(::core::mem::transmute_copy(&econtextstate)).into()
        }
        unsafe extern "system" fn GetContextState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContextState(::core::mem::transmute_copy(&pecontextstate)).into()
        }
        Self {
            base: ISpEventSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRecognizer: GetRecognizer::<Identity, Impl, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetMaxAlternates: GetMaxAlternates::<Identity, Impl, OFFSET>,
            SetMaxAlternates: SetMaxAlternates::<Identity, Impl, OFFSET>,
            SetAudioOptions: SetAudioOptions::<Identity, Impl, OFFSET>,
            GetAudioOptions: GetAudioOptions::<Identity, Impl, OFFSET>,
            DeserializeResult: DeserializeResult::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetVoice: SetVoice::<Identity, Impl, OFFSET>,
            GetVoice: GetVoice::<Identity, Impl, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            GetVoicePurgeEvent: GetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            SetContextState: SetContextState::<Identity, Impl, OFFSET>,
            GetContextState: GetContextState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoContext as ::windows::core::Interface>::IID || iid == &<ISpNotifySource as ::windows::core::Interface>::IID || iid == &<ISpEventSource as ::windows::core::Interface>::IID
    }
}
pub trait ISpRecoContext2_Impl: Sized {
    fn SetGrammarOptions(&self, egrammaroptions: u32) -> ::windows::core::Result<()>;
    fn GetGrammarOptions(&self, pegrammaroptions: *mut u32) -> ::windows::core::Result<()>;
    fn SetAdaptationData2(&self, padaptationdata: &::windows::core::PCWSTR, cch: u32, ptopicname: &::windows::core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows::core::Result<()>;
}
impl ISpRecoContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext2_Impl, const OFFSET: isize>() -> ISpRecoContext2_Vtbl {
        unsafe extern "system" fn SetGrammarOptions<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammaroptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGrammarOptions(::core::mem::transmute_copy(&egrammaroptions)).into()
        }
        unsafe extern "system" fn GetGrammarOptions<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammaroptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGrammarOptions(::core::mem::transmute_copy(&pegrammaroptions)).into()
        }
        unsafe extern "system" fn SetAdaptationData2<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: ::windows::core::PCWSTR, cch: u32, ptopicname: ::windows::core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAdaptationData2(::core::mem::transmute(&padaptationdata), ::core::mem::transmute_copy(&cch), ::core::mem::transmute(&ptopicname), ::core::mem::transmute_copy(&eadaptationsettings), ::core::mem::transmute_copy(&erelevance)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetGrammarOptions: SetGrammarOptions::<Identity, Impl, OFFSET>,
            GetGrammarOptions: GetGrammarOptions::<Identity, Impl, OFFSET>,
            SetAdaptationData2: SetAdaptationData2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpRecoGrammar_Impl: Sized + ISpGrammarBuilder_Impl {
    fn GetGrammarId(&self, pullgrammarid: *mut u64) -> ::windows::core::Result<()>;
    fn GetRecoContext(&self) -> ::windows::core::Result<ISpRecoContext>;
    fn LoadCmdFromFile(&self, pszfilename: &::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromObject(&self, rcid: *const ::windows::core::GUID, pszgrammarname: &::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromResource(&self, hmodule: super::super::Foundation::HINSTANCE, pszresourcename: &::windows::core::PCWSTR, pszresourcetype: &::windows::core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromMemory(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromProprietaryGrammar(&self, rguidparam: *const ::windows::core::GUID, pszstringparam: &::windows::core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn SetRuleState(&self, pszname: &::windows::core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::Result<()>;
    fn SetRuleIdState(&self, ulruleid: u32, newstate: SPRULESTATE) -> ::windows::core::Result<()>;
    fn LoadDictation(&self, psztopicname: &::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn UnloadDictation(&self) -> ::windows::core::Result<()>;
    fn SetDictationState(&self, newstate: SPRULESTATE) -> ::windows::core::Result<()>;
    fn SetWordSequenceData(&self, ptext: &::windows::core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::Result<()>;
    fn SetTextSelection(&self, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::Result<()>;
    fn IsPronounceable(&self, pszword: &::windows::core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows::core::Result<()>;
    fn SetGrammarState(&self, egrammarstate: SPGRAMMARSTATE) -> ::windows::core::Result<()>;
    fn SaveCmd(&self, pstream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetGrammarState(&self, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpRecoGrammar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>() -> ISpRecoGrammar_Vtbl {
        unsafe extern "system" fn GetGrammarId<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullgrammarid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGrammarId(::core::mem::transmute_copy(&pullgrammarid)).into()
        }
        unsafe extern "system" fn GetRecoContext<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecoctxt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecoctxt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromFile<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCmdFromFile(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromObject<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcid: *const ::windows::core::GUID, pszgrammarname: ::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCmdFromObject(::core::mem::transmute_copy(&rcid), ::core::mem::transmute(&pszgrammarname), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromResource<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HINSTANCE, pszresourcename: ::windows::core::PCWSTR, pszresourcetype: ::windows::core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCmdFromResource(::core::mem::transmute_copy(&hmodule), ::core::mem::transmute(&pszresourcename), ::core::mem::transmute(&pszresourcetype), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCmdFromMemory(::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromProprietaryGrammar<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidparam: *const ::windows::core::GUID, pszstringparam: ::windows::core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCmdFromProprietaryGrammar(::core::mem::transmute_copy(&rguidparam), ::core::mem::transmute(&pszstringparam), ::core::mem::transmute_copy(&pvdataprarm), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetRuleState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRuleState(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&preserved), ::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetRuleIdState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulruleid: u32, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRuleIdState(::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn LoadDictation<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztopicname: ::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadDictation(::core::mem::transmute(&psztopicname), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn UnloadDictation<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnloadDictation().into()
        }
        unsafe extern "system" fn SetDictationState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDictationState(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: ::windows::core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWordSequenceData(::core::mem::transmute(&ptext), ::core::mem::transmute_copy(&cchtext), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn SetTextSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextSelection(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn IsPronounceable<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPronounceable(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&pwordpronounceable)).into()
        }
        unsafe extern "system" fn SetGrammarState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGrammarState(::core::mem::transmute_copy(&egrammarstate)).into()
        }
        unsafe extern "system" fn SaveCmd<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ppszcomemerrortext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveCmd(::core::mem::transmute(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemerrortext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrammarState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGrammarState(::core::mem::transmute_copy(&pegrammarstate)).into()
        }
        Self {
            base: ISpGrammarBuilder_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGrammarId: GetGrammarId::<Identity, Impl, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, Impl, OFFSET>,
            LoadCmdFromFile: LoadCmdFromFile::<Identity, Impl, OFFSET>,
            LoadCmdFromObject: LoadCmdFromObject::<Identity, Impl, OFFSET>,
            LoadCmdFromResource: LoadCmdFromResource::<Identity, Impl, OFFSET>,
            LoadCmdFromMemory: LoadCmdFromMemory::<Identity, Impl, OFFSET>,
            LoadCmdFromProprietaryGrammar: LoadCmdFromProprietaryGrammar::<Identity, Impl, OFFSET>,
            SetRuleState: SetRuleState::<Identity, Impl, OFFSET>,
            SetRuleIdState: SetRuleIdState::<Identity, Impl, OFFSET>,
            LoadDictation: LoadDictation::<Identity, Impl, OFFSET>,
            UnloadDictation: UnloadDictation::<Identity, Impl, OFFSET>,
            SetDictationState: SetDictationState::<Identity, Impl, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, Impl, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, Impl, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, Impl, OFFSET>,
            SetGrammarState: SetGrammarState::<Identity, Impl, OFFSET>,
            SaveCmd: SaveCmd::<Identity, Impl, OFFSET>,
            GetGrammarState: GetGrammarState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoGrammar as ::windows::core::Interface>::IID || iid == &<ISpGrammarBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub trait ISpRecoGrammar2_Impl: Sized {
    fn GetRules(&self, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows::core::Result<()>;
    fn LoadCmdFromFile2(&self, pszfilename: &::windows::core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: &::windows::core::PCWSTR, pszbaseuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn LoadCmdFromMemory2(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: &::windows::core::PCWSTR, pszbaseuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetRulePriority(&self, pszrulename: &::windows::core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows::core::Result<()>;
    fn SetRuleWeight(&self, pszrulename: &::windows::core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows::core::Result<()>;
    fn SetDictationWeight(&self, flweight: f32) -> ::windows::core::Result<()>;
    fn SetGrammarLoader(&self, ploader: &::core::option::Option<ISpeechResourceLoader>) -> ::windows::core::Result<()>;
    fn SetSMLSecurityManager(&self, psmlsecuritymanager: &::core::option::Option<super::super::System::Com::Urlmon::IInternetSecurityManager>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_Urlmon")]
impl ISpRecoGrammar2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>() -> ISpRecoGrammar2_Vtbl {
        unsafe extern "system" fn GetRules<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRules(::core::mem::transmute_copy(&ppcomemrules), ::core::mem::transmute_copy(&punumrules)).into()
        }
        unsafe extern "system" fn LoadCmdFromFile2<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: ::windows::core::PCWSTR, pszbaseuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCmdFromFile2(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&pszsharinguri), ::core::mem::transmute(&pszbaseuri)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory2<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: ::windows::core::PCWSTR, pszbaseuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCmdFromMemory2(::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&pszsharinguri), ::core::mem::transmute(&pszbaseuri)).into()
        }
        unsafe extern "system" fn SetRulePriority<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows::core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRulePriority(::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&nrulepriority)).into()
        }
        unsafe extern "system" fn SetRuleWeight<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows::core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRuleWeight(::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetDictationWeight<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flweight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDictationWeight(::core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetGrammarLoader<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGrammarLoader(::core::mem::transmute(&ploader)).into()
        }
        unsafe extern "system" fn SetSMLSecurityManager<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmlsecuritymanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSMLSecurityManager(::core::mem::transmute(&psmlsecuritymanager)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRules: GetRules::<Identity, Impl, OFFSET>,
            LoadCmdFromFile2: LoadCmdFromFile2::<Identity, Impl, OFFSET>,
            LoadCmdFromMemory2: LoadCmdFromMemory2::<Identity, Impl, OFFSET>,
            SetRulePriority: SetRulePriority::<Identity, Impl, OFFSET>,
            SetRuleWeight: SetRuleWeight::<Identity, Impl, OFFSET>,
            SetDictationWeight: SetDictationWeight::<Identity, Impl, OFFSET>,
            SetGrammarLoader: SetGrammarLoader::<Identity, Impl, OFFSET>,
            SetSMLSecurityManager: SetSMLSecurityManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoGrammar2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpRecoResult_Impl: Sized + ISpPhrase_Impl {
    fn GetResultTimes(&self, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::Result<()>;
    fn GetAlternates(&self, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut ::core::option::Option<ISpPhraseAlt>, pcphrasesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat>;
    fn SpeakAudio(&self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::Result<()>;
    fn ScaleAudio(&self, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetRecoContext(&self) -> ::windows::core::Result<ISpRecoContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>() -> ISpRecoResult_Vtbl {
        unsafe extern "system" fn GetResultTimes<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResultTimes(::core::mem::transmute_copy(&ptimes)).into()
        }
        unsafe extern "system" fn GetAlternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut ::windows::core::RawPtr, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAlternates(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&ulrequestcount), ::core::mem::transmute_copy(&ppphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn GetAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SpeakAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&ppcomemserializedresult)).into()
        }
        unsafe extern "system" fn ScaleAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleAudio(::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetRecoContext<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpPhrase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetResultTimes: GetResultTimes::<Identity, Impl, OFFSET>,
            GetAlternates: GetAlternates::<Identity, Impl, OFFSET>,
            GetAudio: GetAudio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            ScaleAudio: ScaleAudio::<Identity, Impl, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoResult as ::windows::core::Interface>::IID || iid == &<ISpPhrase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpRecoResult2_Impl: Sized + ISpPhrase_Impl + ISpRecoResult_Impl {
    fn CommitAlternate(&self, pphrasealt: &::core::option::Option<ISpPhraseAlt>) -> ::windows::core::Result<ISpRecoResult>;
    fn CommitText(&self, ulstartelement: u32, celements: u32, pszcorrecteddata: &::windows::core::PCWSTR, ecommitflags: u32) -> ::windows::core::Result<()>;
    fn SetTextFeedback(&self, pszfeedback: &::windows::core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpRecoResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult2_Impl, const OFFSET: isize>() -> ISpRecoResult2_Vtbl {
        unsafe extern "system" fn CommitAlternate<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrasealt: ::windows::core::RawPtr, ppnewresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CommitAlternate(::core::mem::transmute(&pphrasealt)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitText<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, pszcorrecteddata: ::windows::core::PCWSTR, ecommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitText(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute(&pszcorrecteddata), ::core::mem::transmute_copy(&ecommitflags)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeedback: ::windows::core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextFeedback(::core::mem::transmute(&pszfeedback), ::core::mem::transmute_copy(&fsuccessful)).into()
        }
        Self {
            base: ISpRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommitAlternate: CommitAlternate::<Identity, Impl, OFFSET>,
            CommitText: CommitText::<Identity, Impl, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoResult2 as ::windows::core::Interface>::IID || iid == &<ISpPhrase as ::windows::core::Interface>::IID || iid == &<ISpRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpRecognizer_Impl: Sized + ISpProperties_Impl {
    fn SetRecognizer(&self, precognizer: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn GetRecognizer(&self) -> ::windows::core::Result<ISpObjectToken>;
    fn SetInput(&self, punkinput: &::core::option::Option<::windows::core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInputObjectToken(&self) -> ::windows::core::Result<ISpObjectToken>;
    fn GetInputStream(&self) -> ::windows::core::Result<ISpStreamFormat>;
    fn CreateRecoContext(&self) -> ::windows::core::Result<ISpRecoContext>;
    fn GetRecoProfile(&self) -> ::windows::core::Result<ISpObjectToken>;
    fn SetRecoProfile(&self, ptoken: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn IsSharedInstance(&self) -> ::windows::core::Result<()>;
    fn GetRecoState(&self, pstate: *mut SPRECOSTATE) -> ::windows::core::Result<()>;
    fn SetRecoState(&self, newstate: SPRECOSTATE) -> ::windows::core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows::core::Result<()>;
    fn GetFormat(&self, waveformattype: SPWAVEFORMATTYPE, pformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn IsUISupported(&self, psztypeofui: &::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &::windows::core::PCWSTR, psztypeofui: &::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::Result<()>;
    fn EmulateRecognition(&self, pphrase: &::core::option::Option<ISpPhrase>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>() -> ISpRecognizer_Vtbl {
        unsafe extern "system" fn SetRecognizer<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecognizer(::core::mem::transmute(&precognizer)).into()
        }
        unsafe extern "system" fn GetRecognizer<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInput<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkinput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInput(::core::mem::transmute(&punkinput), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetInputObjectToken<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewctxt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewctxt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoProfile<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoProfile<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecoProfile(::core::mem::transmute(&ptoken)).into()
        }
        unsafe extern "system" fn IsSharedInstance<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSharedInstance().into()
        }
        unsafe extern "system" fn GetRecoState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SPRECOSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRecoState(::core::mem::transmute_copy(&pstate)).into()
        }
        unsafe extern "system" fn SetRecoState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRECOSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecoState(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waveformattype: SPWAVEFORMATTYPE, pformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&waveformattype), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUISupported(::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into()
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EmulateRecognition(::core::mem::transmute(&pphrase)).into()
        }
        Self {
            base: ISpProperties_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRecognizer: SetRecognizer::<Identity, Impl, OFFSET>,
            GetRecognizer: GetRecognizer::<Identity, Impl, OFFSET>,
            SetInput: SetInput::<Identity, Impl, OFFSET>,
            GetInputObjectToken: GetInputObjectToken::<Identity, Impl, OFFSET>,
            GetInputStream: GetInputStream::<Identity, Impl, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, Impl, OFFSET>,
            GetRecoProfile: GetRecoProfile::<Identity, Impl, OFFSET>,
            SetRecoProfile: SetRecoProfile::<Identity, Impl, OFFSET>,
            IsSharedInstance: IsSharedInstance::<Identity, Impl, OFFSET>,
            GetRecoState: GetRecoState::<Identity, Impl, OFFSET>,
            SetRecoState: SetRecoState::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecognizer as ::windows::core::Interface>::IID || iid == &<ISpProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpRecognizer2_Impl: Sized {
    fn EmulateRecognitionEx(&self, pphrase: &::core::option::Option<ISpPhrase>, dwcompareflags: u32) -> ::windows::core::Result<()>;
    fn SetTrainingState(&self, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ResetAcousticModelAdaptation(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpRecognizer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer2_Impl, const OFFSET: isize>() -> ISpRecognizer2_Vtbl {
        unsafe extern "system" fn EmulateRecognitionEx<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: ::windows::core::RawPtr, dwcompareflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EmulateRecognitionEx(::core::mem::transmute(&pphrase), ::core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn SetTrainingState<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrainingState(::core::mem::transmute_copy(&fdoingtraining), ::core::mem::transmute_copy(&fadaptfromtrainingdata)).into()
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetAcousticModelAdaptation().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EmulateRecognitionEx: EmulateRecognitionEx::<Identity, Impl, OFFSET>,
            SetTrainingState: SetTrainingState::<Identity, Impl, OFFSET>,
            ResetAcousticModelAdaptation: ResetAcousticModelAdaptation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecognizer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait ISpRegDataKey_Impl: Sized + ISpDataKey_Impl {
    fn SetKey(&self, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ISpRegDataKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRegDataKey_Impl, const OFFSET: isize>() -> ISpRegDataKey_Vtbl {
        unsafe extern "system" fn SetKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpRegDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKey(::core::mem::transmute_copy(&hkey), ::core::mem::transmute_copy(&freadonly)).into()
        }
        Self { base: ISpDataKey_Vtbl::new::<Identity, Impl, OFFSET>(), SetKey: SetKey::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRegDataKey as ::windows::core::Interface>::IID || iid == &<ISpDataKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpResourceManager_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn SetObject(&self, guidserviceid: *const ::windows::core::GUID, punkobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetObject(&self, guidserviceid: *const ::windows::core::GUID, objectclsid: *const ::windows::core::GUID, objectiid: *const ::windows::core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpResourceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpResourceManager_Impl, const OFFSET: isize>() -> ISpResourceManager_Vtbl {
        unsafe extern "system" fn SetObject<Identity: ::windows::core::IUnknownImpl, Impl: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows::core::GUID, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObject(::core::mem::transmute_copy(&guidserviceid), ::core::mem::transmute(&punkobject)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows::core::GUID, objectclsid: *const ::windows::core::GUID, objectiid: *const ::windows::core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&guidserviceid), ::core::mem::transmute_copy(&objectclsid), ::core::mem::transmute_copy(&objectiid), ::core::mem::transmute_copy(&freleasewhenlastexternalrefreleased), ::core::mem::transmute_copy(&ppobject)).into()
        }
        Self {
            base: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetObject: SetObject::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpResourceManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IServiceProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISpSerializeState_Impl: Sized {
    fn GetSerializedState(&self, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetSerializedState(&self, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl ISpSerializeState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpSerializeState_Impl, const OFFSET: isize>() -> ISpSerializeState_Vtbl {
        unsafe extern "system" fn GetSerializedState<Identity: ::windows::core::IUnknownImpl, Impl: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSerializedState(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pulsize), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetSerializedState<Identity: ::windows::core::IUnknownImpl, Impl: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSerializedState(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSerializedState: GetSerializedState::<Identity, Impl, OFFSET>,
            SetSerializedState: SetSerializedState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpSerializeState as ::windows::core::Interface>::IID
    }
}
pub trait ISpShortcut_Impl: Sized {
    fn AddShortcut(&self, pszdisplay: &::windows::core::PCWSTR, langid: u16, pszspoken: &::windows::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::Result<()>;
    fn RemoveShortcut(&self, pszdisplay: &::windows::core::PCWSTR, langid: u16, pszspoken: &::windows::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::Result<()>;
    fn GetShortcuts(&self, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()>;
    fn GetGeneration(&self) -> ::windows::core::Result<u32>;
    fn GetWordsFromGenerationChange(&self, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
    fn GetWords(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
    fn GetShortcutsForGeneration(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()>;
    fn GetGenerationChange(&self, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()>;
}
impl ISpShortcut_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>() -> ISpShortcut_Vtbl {
        unsafe extern "system" fn AddShortcut<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: ::windows::core::PCWSTR, langid: u16, pszspoken: ::windows::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddShortcut(::core::mem::transmute(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute(&pszspoken), ::core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn RemoveShortcut<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: ::windows::core::PCWSTR, langid: u16, pszspoken: ::windows::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveShortcut(::core::mem::transmute(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute(&pszspoken), ::core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn GetShortcuts<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetShortcuts(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGeneration<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGeneration() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwgeneration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWordsFromGenerationChange<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWordsFromGenerationChange(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWords(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetShortcutsForGeneration<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetShortcutsForGeneration(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGenerationChange(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddShortcut: AddShortcut::<Identity, Impl, OFFSET>,
            RemoveShortcut: RemoveShortcut::<Identity, Impl, OFFSET>,
            GetShortcuts: GetShortcuts::<Identity, Impl, OFFSET>,
            GetGeneration: GetGeneration::<Identity, Impl, OFFSET>,
            GetWordsFromGenerationChange: GetWordsFromGenerationChange::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            GetShortcutsForGeneration: GetShortcutsForGeneration::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpShortcut as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpStream_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl {
    fn SetBaseStream(&self, pstream: &::core::option::Option<super::super::System::Com::IStream>, rguidformat: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetBaseStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn BindToFile(&self, pszfilename: &::windows::core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStream_Impl, const OFFSET: isize>() -> ISpStream_Vtbl {
        unsafe extern "system" fn SetBaseStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, rguidformat: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBaseStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&rguidformat), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetBaseStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFile<Identity: ::windows::core::IUnknownImpl, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindToFile(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&emode), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ISpStreamFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBaseStream: SetBaseStream::<Identity, Impl, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, Impl, OFFSET>,
            BindToFile: BindToFile::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID || iid == &<ISpStreamFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpStreamFormat_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn GetFormat(&self, pguidformatid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut super::Audio::WAVEFORMATEX>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpStreamFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormat_Impl, const OFFSET: isize>() -> ISpStreamFormat_Vtbl {
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidformatid: *const ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&pguidformatid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemwaveformatex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(), GetFormat: GetFormat::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpStreamFormat as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpStreamFormatConverter_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl {
    fn SetBaseStream(&self, pstream: &::core::option::Option<ISpStreamFormat>, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBaseStream(&self) -> ::windows::core::Result<ISpStreamFormat>;
    fn SetFormat(&self, rguidformatidofconvertedstream: *const ::windows::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn ResetSeekPosition(&self) -> ::windows::core::Result<()>;
    fn ScaleConvertedToBaseOffset(&self, ulloffsetconvertedstream: u64) -> ::windows::core::Result<u64>;
    fn ScaleBaseToConvertedOffset(&self, ulloffsetbasestream: u64) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpStreamFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>() -> ISpStreamFormatConverter_Vtbl {
        unsafe extern "system" fn SetBaseStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBaseStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&fsetformattobasestreamformat), ::core::mem::transmute_copy(&fwritetobasestream)).into()
        }
        unsafe extern "system" fn GetBaseStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidformatidofconvertedstream: *const ::windows::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&rguidformatidofconvertedstream), ::core::mem::transmute_copy(&pwaveformatexofconvertedstream)).into()
        }
        unsafe extern "system" fn ResetSeekPosition<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetSeekPosition().into()
        }
        unsafe extern "system" fn ScaleConvertedToBaseOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScaleConvertedToBaseOffset(::core::mem::transmute_copy(&ulloffsetconvertedstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulloffsetbasestream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleBaseToConvertedOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScaleBaseToConvertedOffset(::core::mem::transmute_copy(&ulloffsetbasestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulloffsetconvertedstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpStreamFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBaseStream: SetBaseStream::<Identity, Impl, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ResetSeekPosition: ResetSeekPosition::<Identity, Impl, OFFSET>,
            ScaleConvertedToBaseOffset: ScaleConvertedToBaseOffset::<Identity, Impl, OFFSET>,
            ScaleBaseToConvertedOffset: ScaleBaseToConvertedOffset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpStreamFormatConverter as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID || iid == &<ISpStreamFormat as ::windows::core::Interface>::IID
    }
}
pub trait ISpTranscript_Impl: Sized {
    fn GetTranscript(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn AppendTranscript(&self, psztranscript: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ISpTranscript_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpTranscript_Impl, const OFFSET: isize>() -> ISpTranscript_Vtbl {
        unsafe extern "system" fn GetTranscript<Identity: ::windows::core::IUnknownImpl, Impl: ISpTranscript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztranscript: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTranscript() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztranscript = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTranscript<Identity: ::windows::core::IUnknownImpl, Impl: ISpTranscript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztranscript: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AppendTranscript(::core::mem::transmute(&psztranscript)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTranscript: GetTranscript::<Identity, Impl, OFFSET>,
            AppendTranscript: AppendTranscript::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpTranscript as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpVoice_Impl: Sized + ISpNotifySource_Impl + ISpEventSource_Impl {
    fn SetOutput(&self, punkoutput: &::core::option::Option<::windows::core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetOutputObjectToken(&self) -> ::windows::core::Result<ISpObjectToken>;
    fn GetOutputStream(&self) -> ::windows::core::Result<ISpStreamFormat>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn SetVoice(&self, ptoken: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn GetVoice(&self) -> ::windows::core::Result<ISpObjectToken>;
    fn Speak(&self, pwcs: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<u32>;
    fn SpeakStream(&self, pstream: &::core::option::Option<super::super::System::Com::IStream>, dwflags: u32) -> ::windows::core::Result<u32>;
    fn GetStatus(&self, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn Skip(&self, pitemtype: &::windows::core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows::core::Result<()>;
    fn SetPriority(&self, epriority: SPVPRIORITY) -> ::windows::core::Result<()>;
    fn GetPriority(&self, pepriority: *mut SPVPRIORITY) -> ::windows::core::Result<()>;
    fn SetAlertBoundary(&self, eboundary: SPEVENTENUM) -> ::windows::core::Result<()>;
    fn GetAlertBoundary(&self, peboundary: *mut SPEVENTENUM) -> ::windows::core::Result<()>;
    fn SetRate(&self, rateadjust: i32) -> ::windows::core::Result<()>;
    fn GetRate(&self, prateadjust: *mut i32) -> ::windows::core::Result<()>;
    fn SetVolume(&self, usvolume: u16) -> ::windows::core::Result<()>;
    fn GetVolume(&self, pusvolume: *mut u16) -> ::windows::core::Result<()>;
    fn WaitUntilDone(&self, mstimeout: u32) -> ::windows::core::Result<()>;
    fn SetSyncSpeakTimeout(&self, mstimeout: u32) -> ::windows::core::Result<()>;
    fn GetSyncSpeakTimeout(&self, pmstimeout: *mut u32) -> ::windows::core::Result<()>;
    fn SpeakCompleteEvent(&self) -> super::super::Foundation::HANDLE;
    fn IsUISupported(&self, psztypeofui: &::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &::windows::core::PCWSTR, psztypeofui: &::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>() -> ISpVoice_Vtbl {
        unsafe extern "system" fn SetOutput<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkoutput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutput(::core::mem::transmute(&punkoutput), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetOutputObjectToken<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjecttoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjecttoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn SetVoice<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVoice(::core::mem::transmute(&ptoken)).into()
        }
        unsafe extern "system" fn GetVoice<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVoice() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcs: ::windows::core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Speak(::core::mem::transmute(&pwcs), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulstreamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpeakStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulstreamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppszlastbookmark)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: ::windows::core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute(&pitemtype), ::core::mem::transmute_copy(&lnumitems), ::core::mem::transmute_copy(&pulnumskipped)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epriority: SPVPRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&epriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepriority: *mut SPVPRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPriority(::core::mem::transmute_copy(&pepriority)).into()
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eboundary: SPEVENTENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlertBoundary(::core::mem::transmute_copy(&eboundary)).into()
        }
        unsafe extern "system" fn GetAlertBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peboundary: *mut SPEVENTENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAlertBoundary(::core::mem::transmute_copy(&peboundary)).into()
        }
        unsafe extern "system" fn SetRate<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateadjust: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRate(::core::mem::transmute_copy(&rateadjust)).into()
        }
        unsafe extern "system" fn GetRate<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prateadjust: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRate(::core::mem::transmute_copy(&prateadjust)).into()
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usvolume: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&usvolume)).into()
        }
        unsafe extern "system" fn GetVolume<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusvolume: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVolume(::core::mem::transmute_copy(&pusvolume)).into()
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitUntilDone(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SetSyncSpeakTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSyncSpeakTimeout(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn GetSyncSpeakTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmstimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSyncSpeakTimeout(::core::mem::transmute_copy(&pmstimeout)).into()
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SpeakCompleteEvent()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUISupported(::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into()
        }
        Self {
            base: ISpEventSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetOutputObjectToken: GetOutputObjectToken::<Identity, Impl, OFFSET>,
            GetOutputStream: GetOutputStream::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetVoice: SetVoice::<Identity, Impl, OFFSET>,
            GetVoice: GetVoice::<Identity, Impl, OFFSET>,
            Speak: Speak::<Identity, Impl, OFFSET>,
            SpeakStream: SpeakStream::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, Impl, OFFSET>,
            GetAlertBoundary: GetAlertBoundary::<Identity, Impl, OFFSET>,
            SetRate: SetRate::<Identity, Impl, OFFSET>,
            GetRate: GetRate::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            GetVolume: GetVolume::<Identity, Impl, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, Impl, OFFSET>,
            SetSyncSpeakTimeout: SetSyncSpeakTimeout::<Identity, Impl, OFFSET>,
            GetSyncSpeakTimeout: GetSyncSpeakTimeout::<Identity, Impl, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpVoice as ::windows::core::Interface>::IID || iid == &<ISpNotifySource as ::windows::core::Interface>::IID || iid == &<ISpEventSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpXMLRecoResult_Impl: Sized + ISpPhrase_Impl + ISpRecoResult_Impl {
    fn GetXMLResult(&self, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<()>;
    fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpXMLRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>() -> ISpXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        Self {
            base: ISpRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpXMLRecoResult as ::windows::core::Interface>::IID || iid == &<ISpPhrase as ::windows::core::Interface>::IID || iid == &<ISpRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudio_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn Status(&self) -> ::windows::core::Result<ISpeechAudioStatus>;
    fn BufferInfo(&self) -> ::windows::core::Result<ISpeechAudioBufferInfo>;
    fn DefaultFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn Volume(&self) -> ::windows::core::Result<i32>;
    fn SetVolume(&self, volume: i32) -> ::windows::core::Result<()>;
    fn BufferNotifySize(&self) -> ::windows::core::Result<i32>;
    fn SetBufferNotifySize(&self, buffernotifysize: i32) -> ::windows::core::Result<()>;
    fn EventHandle(&self) -> ::windows::core::Result<i32>;
    fn SetState(&self, state: SpeechAudioState) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>() -> ISpeechAudio_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *bufferinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *streamformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *volume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn BufferNotifySize<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferNotifySize() {
                ::core::result::Result::Ok(ok__) => {
                    *buffernotifysize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferNotifySize(::core::mem::transmute_copy(&buffernotifysize)).into()
        }
        unsafe extern "system" fn EventHandle<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *eventhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechAudioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            BufferInfo: BufferInfo::<Identity, Impl, OFFSET>,
            DefaultFormat: DefaultFormat::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            BufferNotifySize: BufferNotifySize::<Identity, Impl, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, Impl, OFFSET>,
            EventHandle: EventHandle::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudio as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISpeechBaseStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudioBufferInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MinNotification(&self) -> ::windows::core::Result<i32>;
    fn SetMinNotification(&self, minnotification: i32) -> ::windows::core::Result<()>;
    fn BufferSize(&self) -> ::windows::core::Result<i32>;
    fn SetBufferSize(&self, buffersize: i32) -> ::windows::core::Result<()>;
    fn EventBias(&self) -> ::windows::core::Result<i32>;
    fn SetEventBias(&self, eventbias: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudioBufferInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>() -> ISpeechAudioBufferInfo_Vtbl {
        unsafe extern "system" fn MinNotification<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *minnotification = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinNotification<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinNotification(::core::mem::transmute_copy(&minnotification)).into()
        }
        unsafe extern "system" fn BufferSize<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *buffersize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferSize(::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn EventBias<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventBias() {
                ::core::result::Result::Ok(ok__) => {
                    *eventbias = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventBias<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventBias(::core::mem::transmute_copy(&eventbias)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MinNotification: MinNotification::<Identity, Impl, OFFSET>,
            SetMinNotification: SetMinNotification::<Identity, Impl, OFFSET>,
            BufferSize: BufferSize::<Identity, Impl, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, Impl, OFFSET>,
            EventBias: EventBias::<Identity, Impl, OFFSET>,
            SetEventBias: SetEventBias::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudioBufferInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudioFormat_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows::core::Result<SpeechAudioFormatType>;
    fn SetType(&self, audioformat: SpeechAudioFormatType) -> ::windows::core::Result<()>;
    fn Guid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetGuid(&self, guid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetWaveFormatEx(&self) -> ::windows::core::Result<ISpeechWaveFormatEx>;
    fn SetWaveFormatEx(&self, speechwaveformatex: &::core::option::Option<ISpeechWaveFormatEx>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudioFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>() -> ISpeechAudioFormat_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut SpeechAudioFormatType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *audioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: SpeechAudioFormatType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&audioformat)).into()
        }
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *guid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGuid(::core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn GetWaveFormatEx<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWaveFormatEx() {
                ::core::result::Result::Ok(ok__) => {
                    *speechwaveformatex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaveFormatEx<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWaveFormatEx(::core::mem::transmute(&speechwaveformatex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            GetWaveFormatEx: GetWaveFormatEx::<Identity, Impl, OFFSET>,
            SetWaveFormatEx: SetWaveFormatEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudioFormat as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudioStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FreeBufferSpace(&self) -> ::windows::core::Result<i32>;
    fn NonBlockingIO(&self) -> ::windows::core::Result<i32>;
    fn State(&self) -> ::windows::core::Result<SpeechAudioState>;
    fn CurrentSeekPosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CurrentDevicePosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudioStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>() -> ISpeechAudioStatus_Vtbl {
        unsafe extern "system" fn FreeBufferSpace<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freebufferspace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FreeBufferSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *freebufferspace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonBlockingIO<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonblockingio: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NonBlockingIO() {
                ::core::result::Result::Ok(ok__) => {
                    *nonblockingio = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechAudioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSeekPosition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentseekposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentSeekPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *currentseekposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDevicePosition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdeviceposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDevicePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *currentdeviceposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FreeBufferSpace: FreeBufferSpace::<Identity, Impl, OFFSET>,
            NonBlockingIO: NonBlockingIO::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            CurrentSeekPosition: CurrentSeekPosition::<Identity, Impl, OFFSET>,
            CurrentDevicePosition: CurrentDevicePosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudioStatus as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechBaseStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Format(&self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn putref_Format(&self, audioformat: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn Read(&self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()>;
    fn Write(&self, buffer: &super::super::System::Com::VARIANT) -> ::windows::core::Result<i32>;
    fn Seek(&self, position: &super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechBaseStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>() -> ISpeechBaseStream_Vtbl {
        unsafe extern "system" fn Format<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *audioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Format<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Format(::core::mem::transmute(&audioformat)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&numberofbytes), ::core::mem::transmute_copy(&bytesread)).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, byteswritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Write(::core::mem::transmute(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *byteswritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, origin: SpeechStreamSeekPositionType, newposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Seek(::core::mem::transmute(&position), ::core::mem::transmute_copy(&origin)) {
                ::core::result::Result::Ok(ok__) => {
                    *newposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Format: Format::<Identity, Impl, OFFSET>,
            putref_Format: putref_Format::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechBaseStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechCustomStream_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn BaseStream(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_BaseStream(&self, punkstream: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechCustomStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCustomStream_Impl, const OFFSET: isize>() -> ISpeechCustomStream_Vtbl {
        unsafe extern "system" fn BaseStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_BaseStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_BaseStream(::core::mem::transmute(&punkstream)).into()
        }
        Self {
            base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            BaseStream: BaseStream::<Identity, Impl, OFFSET>,
            putref_BaseStream: putref_BaseStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechCustomStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISpeechBaseStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechDataKey_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetBinaryValue(&self, valuename: &super::super::Foundation::BSTR, value: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetBinaryValue(&self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetStringValue(&self, valuename: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetStringValue(&self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLongValue(&self, valuename: &super::super::Foundation::BSTR, value: i32) -> ::windows::core::Result<()>;
    fn GetLongValue(&self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn OpenKey(&self, subkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechDataKey>;
    fn CreateKey(&self, subkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechDataKey>;
    fn DeleteKey(&self, subkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteValue(&self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumKeys(&self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumValues(&self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechDataKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>() -> ISpeechDataKey_Vtbl {
        unsafe extern "system" fn SetBinaryValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBinaryValue(::core::mem::transmute(&valuename), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetBinaryValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBinaryValue(::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute(&valuename), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLongValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLongValue(::core::mem::transmute(&valuename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLongValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLongValue(::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenKey(::core::mem::transmute(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *subkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateKey(::core::mem::transmute(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *subkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteKey(::core::mem::transmute(&subkeyname)).into()
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteValue(::core::mem::transmute(&valuename)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, subkeyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumKeys(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *subkeyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, valuename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumValues(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *valuename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBinaryValue: SetBinaryValue::<Identity, Impl, OFFSET>,
            GetBinaryValue: GetBinaryValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetLongValue: SetLongValue::<Identity, Impl, OFFSET>,
            GetLongValue: GetLongValue::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CreateKey: CreateKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            EnumValues: EnumValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechDataKey as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechFileStream_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn Open(&self, filename: &super::super::Foundation::BSTR, filemode: SpeechStreamFileMode, doevents: i16) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechFileStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechFileStream_Impl, const OFFSET: isize>() -> ISpeechFileStream_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemode: SpeechStreamFileMode, doevents: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&filemode), ::core::mem::transmute_copy(&doevents)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self { base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(), Open: Open::<Identity, Impl, OFFSET>, Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechFileStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISpeechBaseStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Attributes(&self) -> ::windows::core::Result<SpeechRuleAttributes>;
    fn InitialState(&self) -> ::windows::core::Result<ISpeechGrammarRuleState>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn AddResource(&self, resourcename: &super::super::Foundation::BSTR, resourcevalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddState(&self) -> ::windows::core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>() -> ISpeechGrammarRule_Vtbl {
        unsafe extern "system" fn Attributes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut SpeechRuleAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddResource<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcevalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddResource(::core::mem::transmute(&resourcename), ::core::mem::transmute(&resourcevalue)).into()
        }
        unsafe extern "system" fn AddState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            InitialState: InitialState::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            AddState: AddState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRule as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRuleState_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Rule(&self) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Transitions(&self) -> ::windows::core::Result<ISpeechGrammarRuleStateTransitions>;
    fn AddWordTransition(&self, deststate: &::core::option::Option<ISpeechGrammarRuleState>, words: &super::super::Foundation::BSTR, separators: &super::super::Foundation::BSTR, r#type: SpeechGrammarWordType, propertyname: &super::super::Foundation::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>;
    fn AddRuleTransition(&self, destinationstate: &::core::option::Option<ISpeechGrammarRuleState>, rule: &::core::option::Option<ISpeechGrammarRule>, propertyname: &super::super::Foundation::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>;
    fn AddSpecialTransition(&self, destinationstate: &::core::option::Option<ISpeechGrammarRuleState>, r#type: SpeechSpecialTransitionType, propertyname: &super::super::Foundation::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRuleState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleState_Vtbl {
        unsafe extern "system" fn Rule<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rule() {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transitions<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Transitions() {
                ::core::result::Result::Ok(ok__) => {
                    *transitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWordTransition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deststate: ::windows::core::RawPtr, words: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, separators: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: SpeechGrammarWordType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddWordTransition(::core::mem::transmute(&deststate), ::core::mem::transmute(&words), ::core::mem::transmute(&separators), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: ::windows::core::RawPtr, rule: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRuleTransition(::core::mem::transmute(&destinationstate), ::core::mem::transmute(&rule), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddSpecialTransition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: ::windows::core::RawPtr, r#type: SpeechSpecialTransitionType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSpecialTransition(::core::mem::transmute(&destinationstate), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Rule: Rule::<Identity, Impl, OFFSET>,
            Transitions: Transitions::<Identity, Impl, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, Impl, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, Impl, OFFSET>,
            AddSpecialTransition: AddSpecialTransition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleState as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRuleStateTransition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows::core::Result<SpeechGrammarRuleStateTransitionType>;
    fn Text(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Rule(&self) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Weight(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PropertyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PropertyId(&self) -> ::windows::core::Result<i32>;
    fn PropertyValue(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NextState(&self) -> ::windows::core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRuleStateTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransition_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut SpeechGrammarRuleStateTransitionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rule() {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *weight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyName<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyName() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyValue() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextState() {
                ::core::result::Result::Ok(ok__) => {
                    *nextstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            Rule: Rule::<Identity, Impl, OFFSET>,
            Weight: Weight::<Identity, Impl, OFFSET>,
            PropertyName: PropertyName::<Identity, Impl, OFFSET>,
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            NextState: NextState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransition as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRuleStateTransitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechGrammarRuleStateTransition>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRuleStateTransitions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransitions_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransitions as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn FindRule(&self, rulenameorid: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Dynamic(&self) -> ::windows::core::Result<i16>;
    fn Add(&self, rulename: &super::super::Foundation::BSTR, attributes: SpeechRuleAttributes, ruleid: i32) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn CommitAndSave(&self, errortext: *mut super::super::Foundation::BSTR, savestream: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>() -> ISpeechGrammarRules_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindRule<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulenameorid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindRule(::core::mem::transmute(&rulenameorid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dynamic<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Dynamic() {
                ::core::result::Result::Ok(ok__) => {
                    *dynamic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute(&rulename), ::core::mem::transmute_copy(&attributes), ::core::mem::transmute_copy(&ruleid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn CommitAndSave<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortext: *mut super::super::Foundation::BSTR, savestream: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitAndSave(::core::mem::transmute_copy(&errortext), ::core::mem::transmute_copy(&savestream)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            FindRule: FindRule::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Dynamic: Dynamic::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            CommitAndSave: CommitAndSave::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRules as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexicon_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GenerationId(&self) -> ::windows::core::Result<i32>;
    fn GetWords(&self, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows::core::Result<()>;
    fn AddPronunciation(&self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddPronunciationByPhoneIds(&self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RemovePronunciation(&self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemovePronunciationByPhoneIds(&self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetPronunciations(&self, bstrword: &super::super::Foundation::BSTR, langid: i32, typeflags: SpeechLexiconType) -> ::windows::core::Result<ISpeechLexiconPronunciations>;
    fn GetGenerationChange(&self, generationid: *mut i32, ppwords: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexicon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>() -> ISpeechLexicon_Vtbl {
        unsafe extern "system" fn GenerationId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerationId() {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWords(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&words)).into()
        }
        unsafe extern "system" fn AddPronunciation<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPronunciation(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn AddPronunciationByPhoneIds<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPronunciationByPhoneIds(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePronunciation(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciationByPhoneIds<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePronunciationByPhoneIds(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn GetPronunciations<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPronunciations(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&typeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppronunciations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32, ppwords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGenerationChange(::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&ppwords)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GenerationId: GenerationId::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, Impl, OFFSET>,
            AddPronunciationByPhoneIds: AddPronunciationByPhoneIds::<Identity, Impl, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, Impl, OFFSET>,
            RemovePronunciationByPhoneIds: RemovePronunciationByPhoneIds::<Identity, Impl, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexicon as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconPronunciation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows::core::Result<SpeechLexiconType>;
    fn LangId(&self) -> ::windows::core::Result<i32>;
    fn PartOfSpeech(&self) -> ::windows::core::Result<SpeechPartOfSpeech>;
    fn PhoneIds(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Symbolic(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconPronunciation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>() -> ISpeechLexiconPronunciation_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicontype: *mut SpeechLexiconType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *lexicontype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LangId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LangId() {
                ::core::result::Result::Ok(ok__) => {
                    *langid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartOfSpeech<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partofspeech: *mut SpeechPartOfSpeech) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PartOfSpeech() {
                ::core::result::Result::Ok(ok__) => {
                    *partofspeech = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneIds<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhoneIds() {
                ::core::result::Result::Ok(ok__) => {
                    *phoneids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Symbolic<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbolic: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Symbolic() {
                ::core::result::Result::Ok(ok__) => {
                    *symbolic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            LangId: LangId::<Identity, Impl, OFFSET>,
            PartOfSpeech: PartOfSpeech::<Identity, Impl, OFFSET>,
            PhoneIds: PhoneIds::<Identity, Impl, OFFSET>,
            Symbolic: Symbolic::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconPronunciations_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechLexiconPronunciation>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconPronunciations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>() -> ISpeechLexiconPronunciations_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pronunciation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pronunciation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciations as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconWord_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LangId(&self) -> ::windows::core::Result<i32>;
    fn Type(&self) -> ::windows::core::Result<SpeechWordType>;
    fn Word(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Pronunciations(&self) -> ::windows::core::Result<ISpeechLexiconPronunciations>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconWord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>() -> ISpeechLexiconWord_Vtbl {
        unsafe extern "system" fn LangId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LangId() {
                ::core::result::Result::Ok(ok__) => {
                    *langid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordtype: *mut SpeechWordType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *wordtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Word<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Word() {
                ::core::result::Result::Ok(ok__) => {
                    *word = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciations<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Pronunciations() {
                ::core::result::Result::Ok(ok__) => {
                    *pronunciations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LangId: LangId::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Word: Word::<Identity, Impl, OFFSET>,
            Pronunciations: Pronunciations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconWord as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconWords_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechLexiconWord>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconWords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>() -> ISpeechLexiconWords_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, word: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *word = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconWords as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechMMSysAudio_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl + ISpeechAudio_Impl {
    fn DeviceId(&self) -> ::windows::core::Result<i32>;
    fn SetDeviceId(&self, deviceid: i32) -> ::windows::core::Result<()>;
    fn LineId(&self) -> ::windows::core::Result<i32>;
    fn SetLineId(&self, lineid: i32) -> ::windows::core::Result<()>;
    fn MMHandle(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechMMSysAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>() -> ISpeechMMSysAudio_Vtbl {
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDeviceId(::core::mem::transmute_copy(&deviceid)).into()
        }
        unsafe extern "system" fn LineId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineId() {
                ::core::result::Result::Ok(ok__) => {
                    *lineid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLineId(::core::mem::transmute_copy(&lineid)).into()
        }
        unsafe extern "system" fn MMHandle<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MMHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *handle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpeechAudio_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            LineId: LineId::<Identity, Impl, OFFSET>,
            SetLineId: SetLineId::<Identity, Impl, OFFSET>,
            MMHandle: MMHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechMMSysAudio as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISpeechBaseStream as ::windows::core::Interface>::IID || iid == &<ISpeechAudio as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechMemoryStream_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn SetData(&self, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechMemoryStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>() -> ISpeechMemoryStream_Vtbl {
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetData() {
                ::core::result::Result::Ok(ok__) => {
                    *pdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechMemoryStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISpeechBaseStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechObjectToken_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DataKey(&self) -> ::windows::core::Result<ISpeechDataKey>;
    fn Category(&self) -> ::windows::core::Result<ISpeechObjectTokenCategory>;
    fn GetDescription(&self, locale: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetId(&self, id: &super::super::Foundation::BSTR, categoryid: &super::super::Foundation::BSTR, createifnotexist: i16) -> ::windows::core::Result<()>;
    fn GetAttribute(&self, attributename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateInstance(&self, punkouter: &::core::option::Option<::windows::core::IUnknown>, clscontext: SpeechTokenContext) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Remove(&self, objectstorageclsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetStorageFileName(&self, objectstorageclsid: &super::super::Foundation::BSTR, keyname: &super::super::Foundation::BSTR, filename: &super::super::Foundation::BSTR, folder: SpeechTokenShellFolder) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RemoveStorageFileName(&self, objectstorageclsid: &super::super::Foundation::BSTR, keyname: &super::super::Foundation::BSTR, deletefilea: i16) -> ::windows::core::Result<()>;
    fn IsUISupported(&self, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT, object: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<i16>;
    fn DisplayUI(&self, hwnd: i32, title: &super::super::Foundation::BSTR, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT, object: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MatchesAttributes(&self, attributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechObjectToken_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>() -> ISpeechObjectToken_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *objectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataKey() {
                ::core::result::Result::Ok(ok__) => {
                    *datakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *category = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: i32, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, categoryid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute(&id), ::core::mem::transmute(&categoryid), ::core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributevalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttribute(::core::mem::transmute(&attributename)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributevalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&clscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&objectstorageclsid)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStorageFileName(::core::mem::transmute(&objectstorageclsid), ::core::mem::transmute(&keyname), ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&folder)) {
                ::core::result::Result::Ok(ok__) => {
                    *filepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, deletefilea: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStorageFileName(::core::mem::transmute(&objectstorageclsid), ::core::mem::transmute(&keyname), ::core::mem::transmute_copy(&deletefilea)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUISupported(::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata), ::core::mem::transmute(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, matches: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MatchesAttributes(::core::mem::transmute(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *matches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            DataKey: DataKey::<Identity, Impl, OFFSET>,
            Category: Category::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, Impl, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechObjectToken as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechObjectTokenCategory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDefault(&self, tokenid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Default(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetId(&self, id: &super::super::Foundation::BSTR, createifnotexist: i16) -> ::windows::core::Result<()>;
    fn GetDataKey(&self, location: SpeechDataKeyLocation) -> ::windows::core::Result<ISpeechDataKey>;
    fn EnumerateTokens(&self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>() -> ISpeechObjectTokenCategory_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefault(::core::mem::transmute(&tokenid)).into()
        }
        unsafe extern "system" fn Default<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Default() {
                ::core::result::Result::Ok(ok__) => {
                    *tokenid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute(&id), ::core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetDataKey<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: SpeechDataKeyLocation, datakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataKey(::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *datakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTokens<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateTokens(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *tokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            SetDefault: SetDefault::<Identity, Impl, OFFSET>,
            Default: Default::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetDataKey: GetDataKey::<Identity, Impl, OFFSET>,
            EnumerateTokens: EnumerateTokens::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechObjectTokenCategory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechObjectTokens_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechObjectToken>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechObjectTokens_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>() -> ISpeechObjectTokens_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, token: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *token = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechObjectTokens as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhoneConverter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&self) -> ::windows::core::Result<i32>;
    fn SetLanguageId(&self, languageid: i32) -> ::windows::core::Result<()>;
    fn PhoneToId(&self, phonemes: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn IdToPhone(&self, idarray: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhoneConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>() -> ISpeechPhoneConverter_Vtbl {
        unsafe extern "system" fn LanguageId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LanguageId() {
                ::core::result::Result::Ok(ok__) => {
                    *languageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLanguageId(::core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn PhoneToId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonemes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, idarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhoneToId(::core::mem::transmute(&phonemes)) {
                ::core::result::Result::Ok(ok__) => {
                    *idarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idarray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, phonemes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IdToPhone(::core::mem::transmute(&idarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *phonemes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LanguageId: LanguageId::<Identity, Impl, OFFSET>,
            SetLanguageId: SetLanguageId::<Identity, Impl, OFFSET>,
            PhoneToId: PhoneToId::<Identity, Impl, OFFSET>,
            IdToPhone: IdToPhone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhoneConverter as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseAlternate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoResult(&self) -> ::windows::core::Result<ISpeechRecoResult>;
    fn StartElementInResult(&self) -> ::windows::core::Result<i32>;
    fn NumberOfElementsInResult(&self) -> ::windows::core::Result<i32>;
    fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo>;
    fn Commit(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseAlternate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>() -> ISpeechPhraseAlternate_Vtbl {
        unsafe extern "system" fn RecoResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecoResult() {
                ::core::result::Result::Ok(ok__) => {
                    *recoresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartElementInResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartElementInResult() {
                ::core::result::Result::Ok(ok__) => {
                    *startelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElementsInResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfElementsInResult() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RecoResult: RecoResult::<Identity, Impl, OFFSET>,
            StartElementInResult: StartElementInResult::<Identity, Impl, OFFSET>,
            NumberOfElementsInResult: NumberOfElementsInResult::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternate as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseAlternates_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseAlternate>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseAlternates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>() -> ISpeechPhraseAlternates_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, phrasealternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrasealternate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternates as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseElement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioTimeOffset(&self) -> ::windows::core::Result<i32>;
    fn AudioSizeTime(&self) -> ::windows::core::Result<i32>;
    fn AudioStreamOffset(&self) -> ::windows::core::Result<i32>;
    fn AudioSizeBytes(&self) -> ::windows::core::Result<i32>;
    fn RetainedStreamOffset(&self) -> ::windows::core::Result<i32>;
    fn RetainedSizeBytes(&self) -> ::windows::core::Result<i32>;
    fn DisplayText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LexicalForm(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Pronunciation(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DisplayAttributes(&self) -> ::windows::core::Result<SpeechDisplayAttributes>;
    fn RequiredConfidence(&self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn ActualConfidence(&self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>() -> ISpeechPhraseElement_Vtbl {
        unsafe extern "system" fn AudioTimeOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiotimeoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioTimeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *audiotimeoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioSizeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *audiosizetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioStreamOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *audiostreamoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *audiosizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedStreamOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedstreamoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetainedStreamOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *retainedstreamoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetainedSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *retainedsizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayText<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *displaytext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LexicalForm<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicalform: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LexicalForm() {
                ::core::result::Result::Ok(ok__) => {
                    *lexicalform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciation<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciation: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Pronunciation() {
                ::core::result::Result::Ok(ok__) => {
                    *pronunciation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAttributes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *displayattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiredConfidence<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequiredConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *requiredconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualConfidence<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActualConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *actualconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *engineconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AudioTimeOffset: AudioTimeOffset::<Identity, Impl, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, Impl, OFFSET>,
            AudioStreamOffset: AudioStreamOffset::<Identity, Impl, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, Impl, OFFSET>,
            RetainedStreamOffset: RetainedStreamOffset::<Identity, Impl, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, Impl, OFFSET>,
            DisplayText: DisplayText::<Identity, Impl, OFFSET>,
            LexicalForm: LexicalForm::<Identity, Impl, OFFSET>,
            Pronunciation: Pronunciation::<Identity, Impl, OFFSET>,
            DisplayAttributes: DisplayAttributes::<Identity, Impl, OFFSET>,
            RequiredConfidence: RequiredConfidence::<Identity, Impl, OFFSET>,
            ActualConfidence: ActualConfidence::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseElement as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseElements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseElement>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseElements_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>() -> ISpeechPhraseElements_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseElements as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&self) -> ::windows::core::Result<i32>;
    fn GrammarId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AudioStreamPosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AudioSizeBytes(&self) -> ::windows::core::Result<i32>;
    fn RetainedSizeBytes(&self) -> ::windows::core::Result<i32>;
    fn AudioSizeTime(&self) -> ::windows::core::Result<i32>;
    fn Rule(&self) -> ::windows::core::Result<ISpeechPhraseRule>;
    fn Properties(&self) -> ::windows::core::Result<ISpeechPhraseProperties>;
    fn Elements(&self) -> ::windows::core::Result<ISpeechPhraseElements>;
    fn Replacements(&self) -> ::windows::core::Result<ISpeechPhraseReplacements>;
    fn EngineId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnginePrivateData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetText(&self, startelement: i32, elements: i32, usereplacements: i16) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDisplayAttributes(&self, startelement: i32, elements: i32, usereplacements: i16) -> ::windows::core::Result<SpeechDisplayAttributes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>() -> ISpeechPhraseInfo_Vtbl {
        unsafe extern "system" fn LanguageId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LanguageId() {
                ::core::result::Result::Ok(ok__) => {
                    *languageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrammarId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GrammarId() {
                ::core::result::Result::Ok(ok__) => {
                    *grammarid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *starttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamPosition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioStreamPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *audiostreamposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiosizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *paudiosizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetainedSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *retainedsizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioSizeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *audiosizetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rule() {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *properties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Elements<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Elements() {
                ::core::result::Result::Ok(ok__) => {
                    *elements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacements<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replacements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Replacements() {
                ::core::result::Result::Ok(ok__) => {
                    *replacements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineidguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EngineId() {
                ::core::result::Result::Ok(ok__) => {
                    *engineidguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnginePrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privatedata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnginePrivateData() {
                ::core::result::Result::Ok(ok__) => {
                    *privatedata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: i16, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: i16, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDisplayAttributes(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    *displayattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LanguageId: LanguageId::<Identity, Impl, OFFSET>,
            GrammarId: GrammarId::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            AudioStreamPosition: AudioStreamPosition::<Identity, Impl, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, Impl, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, Impl, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, Impl, OFFSET>,
            Rule: Rule::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Elements: Elements::<Identity, Impl, OFFSET>,
            Replacements: Replacements::<Identity, Impl, OFFSET>,
            EngineId: EngineId::<Identity, Impl, OFFSET>,
            EnginePrivateData: EnginePrivateData::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetDisplayAttributes: GetDisplayAttributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseInfoBuilder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RestorePhraseFromMemory(&self, phraseinmemory: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechPhraseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseInfoBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfoBuilder_Impl, const OFFSET: isize>() -> ISpeechPhraseInfoBuilder_Vtbl {
        unsafe extern "system" fn RestorePhraseFromMemory<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfoBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinmemory: *const super::super::System::Com::VARIANT, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RestorePhraseFromMemory(::core::mem::transmute_copy(&phraseinmemory)) {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RestorePhraseFromMemory: RestorePhraseFromMemory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseInfoBuilder as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseProperty>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>() -> ISpeechPhraseProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *property = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseProperties as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn FirstElement(&self) -> ::windows::core::Result<i32>;
    fn NumberOfElements(&self) -> ::windows::core::Result<i32>;
    fn EngineConfidence(&self) -> ::windows::core::Result<f32>;
    fn Confidence(&self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn Parent(&self) -> ::windows::core::Result<ISpeechPhraseProperty>;
    fn Children(&self) -> ::windows::core::Result<ISpeechPhraseProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>() -> ISpeechPhraseProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    *firstelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *confidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *confidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *parentproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseProperty as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseReplacement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DisplayAttributes(&self) -> ::windows::core::Result<SpeechDisplayAttributes>;
    fn Text(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FirstElement(&self) -> ::windows::core::Result<i32>;
    fn NumberOfElements(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseReplacement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>() -> ISpeechPhraseReplacement_Vtbl {
        unsafe extern "system" fn DisplayAttributes<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *displayattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    *firstelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayAttributes: DisplayAttributes::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacement as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseReplacements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseReplacement>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseReplacements_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>() -> ISpeechPhraseReplacements_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, reps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *reps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacements as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn FirstElement(&self) -> ::windows::core::Result<i32>;
    fn NumberOfElements(&self) -> ::windows::core::Result<i32>;
    fn Parent(&self) -> ::windows::core::Result<ISpeechPhraseRule>;
    fn Children(&self) -> ::windows::core::Result<ISpeechPhraseRules>;
    fn Confidence(&self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>() -> ISpeechPhraseRule_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    *firstelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *actualconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *engineconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseRule as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseRule>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>() -> ISpeechPhraseRules_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseRules as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Recognizer(&self) -> ::windows::core::Result<ISpeechRecognizer>;
    fn AudioInputInterferenceStatus(&self) -> ::windows::core::Result<SpeechInterference>;
    fn RequestedUIType(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn putref_Voice(&self, voice: &::core::option::Option<ISpeechVoice>) -> ::windows::core::Result<()>;
    fn Voice(&self) -> ::windows::core::Result<ISpeechVoice>;
    fn SetAllowVoiceFormatMatchingOnNextSet(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowVoiceFormatMatchingOnNextSet(&self) -> ::windows::core::Result<i16>;
    fn SetVoicePurgeEvent(&self, eventinterest: SpeechRecoEvents) -> ::windows::core::Result<()>;
    fn VoicePurgeEvent(&self) -> ::windows::core::Result<SpeechRecoEvents>;
    fn SetEventInterests(&self, eventinterest: SpeechRecoEvents) -> ::windows::core::Result<()>;
    fn EventInterests(&self) -> ::windows::core::Result<SpeechRecoEvents>;
    fn SetCmdMaxAlternates(&self, maxalternates: i32) -> ::windows::core::Result<()>;
    fn CmdMaxAlternates(&self) -> ::windows::core::Result<i32>;
    fn SetState(&self, state: SpeechRecoContextState) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<SpeechRecoContextState>;
    fn SetRetainedAudio(&self, option: SpeechRetainedAudioOptions) -> ::windows::core::Result<()>;
    fn RetainedAudio(&self) -> ::windows::core::Result<SpeechRetainedAudioOptions>;
    fn putref_RetainedAudioFormat(&self, format: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn RetainedAudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn CreateGrammar(&self, grammarid: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechRecoGrammar>;
    fn CreateResultFromMemory(&self, resultblock: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechRecoResult>;
    fn Bookmark(&self, options: SpeechBookmarkOptions, streampos: &super::super::System::Com::VARIANT, bookmarkid: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetAdaptationData(&self, adaptationstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>() -> ISpeechRecoContext_Vtbl {
        unsafe extern "system" fn Recognizer<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *recognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioInputInterferenceStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interference: *mut SpeechInterference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioInputInterferenceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *interference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUIType<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uitype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedUIType() {
                ::core::result::Result::Ok(ok__) => {
                    *uitype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Voice(::core::mem::transmute(&voice)).into()
        }
        unsafe extern "system" fn Voice<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Voice() {
                ::core::result::Result::Ok(ok__) => {
                    *voice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowVoiceFormatMatchingOnNextSet<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowVoiceFormatMatchingOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowVoiceFormatMatchingOnNextSet<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowVoiceFormatMatchingOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    *pallow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVoicePurgeEvent(::core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn VoicePurgeEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VoicePurgeEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *eventinterest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventInterests(::core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn EventInterests<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventInterests() {
                ::core::result::Result::Ok(ok__) => {
                    *eventinterest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCmdMaxAlternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCmdMaxAlternates(::core::mem::transmute_copy(&maxalternates)).into()
        }
        unsafe extern "system" fn CmdMaxAlternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CmdMaxAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    *maxalternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecoContextState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecoContextState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetainedAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SpeechRetainedAudioOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRetainedAudio(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn RetainedAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: *mut SpeechRetainedAudioOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetainedAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *option = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RetainedAudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_RetainedAudioFormat(::core::mem::transmute(&format)).into()
        }
        unsafe extern "system" fn RetainedAudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetainedAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn CreateGrammar<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, grammar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGrammar(::core::mem::transmute(&grammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    *grammar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultFromMemory<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *const super::super::System::Com::VARIANT, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateResultFromMemory(::core::mem::transmute_copy(&resultblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SpeechBookmarkOptions, streampos: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bookmarkid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Bookmark(::core::mem::transmute_copy(&options), ::core::mem::transmute(&streampos), ::core::mem::transmute(&bookmarkid)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adaptationstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAdaptationData(::core::mem::transmute(&adaptationstring)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            AudioInputInterferenceStatus: AudioInputInterferenceStatus::<Identity, Impl, OFFSET>,
            RequestedUIType: RequestedUIType::<Identity, Impl, OFFSET>,
            putref_Voice: putref_Voice::<Identity, Impl, OFFSET>,
            Voice: Voice::<Identity, Impl, OFFSET>,
            SetAllowVoiceFormatMatchingOnNextSet: SetAllowVoiceFormatMatchingOnNextSet::<Identity, Impl, OFFSET>,
            AllowVoiceFormatMatchingOnNextSet: AllowVoiceFormatMatchingOnNextSet::<Identity, Impl, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            VoicePurgeEvent: VoicePurgeEvent::<Identity, Impl, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, Impl, OFFSET>,
            EventInterests: EventInterests::<Identity, Impl, OFFSET>,
            SetCmdMaxAlternates: SetCmdMaxAlternates::<Identity, Impl, OFFSET>,
            CmdMaxAlternates: CmdMaxAlternates::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetRetainedAudio: SetRetainedAudio::<Identity, Impl, OFFSET>,
            RetainedAudio: RetainedAudio::<Identity, Impl, OFFSET>,
            putref_RetainedAudioFormat: putref_RetainedAudioFormat::<Identity, Impl, OFFSET>,
            RetainedAudioFormat: RetainedAudioFormat::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, Impl, OFFSET>,
            CreateResultFromMemory: CreateResultFromMemory::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoContext as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoGrammar_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn SetState(&self, state: SpeechGrammarState) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<SpeechGrammarState>;
    fn Rules(&self) -> ::windows::core::Result<ISpeechGrammarRules>;
    fn Reset(&self, newlanguage: i32) -> ::windows::core::Result<()>;
    fn CmdLoadFromFile(&self, filename: &super::super::Foundation::BSTR, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromObject(&self, classid: &super::super::Foundation::BSTR, grammarname: &super::super::Foundation::BSTR, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromResource(&self, hmodule: i32, resourcename: &super::super::System::Com::VARIANT, resourcetype: &super::super::System::Com::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromMemory(&self, grammardata: &super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromProprietaryGrammar(&self, proprietaryguid: &super::super::Foundation::BSTR, proprietarystring: &super::super::Foundation::BSTR, proprietarydata: &super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdSetRuleState(&self, name: &super::super::Foundation::BSTR, state: SpeechRuleState) -> ::windows::core::Result<()>;
    fn CmdSetRuleIdState(&self, ruleid: i32, state: SpeechRuleState) -> ::windows::core::Result<()>;
    fn DictationLoad(&self, topicname: &super::super::Foundation::BSTR, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn DictationUnload(&self) -> ::windows::core::Result<()>;
    fn DictationSetState(&self, state: SpeechRuleState) -> ::windows::core::Result<()>;
    fn SetWordSequenceData(&self, text: &super::super::Foundation::BSTR, textlength: i32, info: &::core::option::Option<ISpeechTextSelectionInformation>) -> ::windows::core::Result<()>;
    fn SetTextSelection(&self, info: &::core::option::Option<ISpeechTextSelectionInformation>) -> ::windows::core::Result<()>;
    fn IsPronounceable(&self, word: &super::super::Foundation::BSTR) -> ::windows::core::Result<SpeechWordPronounceable>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoGrammar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>() -> ISpeechRecoGrammar_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecoContext<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *recocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechGrammarState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechGrammarState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *rules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn CmdLoadFromFile<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CmdLoadFromFile(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromObject<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grammarname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CmdLoadFromObject(::core::mem::transmute(&classid), ::core::mem::transmute(&grammarname), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromResource<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: i32, resourcename: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, resourcetype: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, languageid: i32, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CmdLoadFromResource(::core::mem::transmute_copy(&hmodule), ::core::mem::transmute(&resourcename), ::core::mem::transmute(&resourcetype), ::core::mem::transmute_copy(&languageid), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromMemory<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CmdLoadFromMemory(::core::mem::transmute(&grammardata), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromProprietaryGrammar<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proprietaryguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarydata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CmdLoadFromProprietaryGrammar(::core::mem::transmute(&proprietaryguid), ::core::mem::transmute(&proprietarystring), ::core::mem::transmute(&proprietarydata), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdSetRuleState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CmdSetRuleState(::core::mem::transmute(&name), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CmdSetRuleIdState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleid: i32, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CmdSetRuleIdState(::core::mem::transmute_copy(&ruleid), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn DictationLoad<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topicname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DictationLoad(::core::mem::transmute(&topicname), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn DictationUnload<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DictationUnload().into()
        }
        unsafe extern "system" fn DictationSetState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DictationSetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, textlength: i32, info: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWordSequenceData(::core::mem::transmute(&text), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute(&info)).into()
        }
        unsafe extern "system" fn SetTextSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, info: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextSelection(::core::mem::transmute(&info)).into()
        }
        unsafe extern "system" fn IsPronounceable<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPronounceable(::core::mem::transmute(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    *wordpronounceable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            CmdLoadFromFile: CmdLoadFromFile::<Identity, Impl, OFFSET>,
            CmdLoadFromObject: CmdLoadFromObject::<Identity, Impl, OFFSET>,
            CmdLoadFromResource: CmdLoadFromResource::<Identity, Impl, OFFSET>,
            CmdLoadFromMemory: CmdLoadFromMemory::<Identity, Impl, OFFSET>,
            CmdLoadFromProprietaryGrammar: CmdLoadFromProprietaryGrammar::<Identity, Impl, OFFSET>,
            CmdSetRuleState: CmdSetRuleState::<Identity, Impl, OFFSET>,
            CmdSetRuleIdState: CmdSetRuleIdState::<Identity, Impl, OFFSET>,
            DictationLoad: DictationLoad::<Identity, Impl, OFFSET>,
            DictationUnload: DictationUnload::<Identity, Impl, OFFSET>,
            DictationSetState: DictationSetState::<Identity, Impl, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, Impl, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, Impl, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoGrammar as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn Times(&self) -> ::windows::core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&self, format: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn AudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo>;
    fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates>;
    fn Audio(&self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>() -> ISpeechRecoResult_Vtbl {
        unsafe extern "system" fn RecoContext<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *recocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Times() {
                ::core::result::Result::Ok(ok__) => {
                    *times = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AudioFormat(::core::mem::transmute(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Alternates(::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *alternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Audio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpeakAudio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *resultblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardResultInfo(::core::mem::transmute_copy(&valuetypes)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            Times: Times::<Identity, Impl, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, Impl, OFFSET>,
            AudioFormat: AudioFormat::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Alternates: Alternates::<Identity, Impl, OFFSET>,
            Audio: Audio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResult as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResult2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechRecoResult_Impl {
    fn SetTextFeedback(&self, feedback: &super::super::Foundation::BSTR, wassuccessful: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult2_Impl, const OFFSET: isize>() -> ISpeechRecoResult2_Vtbl {
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextFeedback(::core::mem::transmute(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self { base: ISpeechRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(), SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResult2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISpeechRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResultDispatch_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn Times(&self) -> ::windows::core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&self, format: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn AudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo>;
    fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates>;
    fn Audio(&self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()>;
    fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut ::windows::core::HRESULT, iserror: *mut i16) -> ::windows::core::Result<()>;
    fn SetTextFeedback(&self, feedback: &super::super::Foundation::BSTR, wassuccessful: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResultDispatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>() -> ISpeechRecoResultDispatch_Vtbl {
        unsafe extern "system" fn RecoContext<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *recocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Times() {
                ::core::result::Result::Ok(ok__) => {
                    *times = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AudioFormat(::core::mem::transmute(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Alternates(::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *alternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Audio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpeakAudio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *resultblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardResultInfo(::core::mem::transmute_copy(&valuetypes)).into()
        }
        unsafe extern "system" fn GetXMLResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXMLResult(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut ::windows::core::HRESULT, iserror: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextFeedback(::core::mem::transmute(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            Times: Times::<Identity, Impl, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, Impl, OFFSET>,
            AudioFormat: AudioFormat::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Alternates: Alternates::<Identity, Impl, OFFSET>,
            Audio: Audio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, Impl, OFFSET>,
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResultDispatch as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResultTimes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StreamTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Length(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn TickCount(&self) -> ::windows::core::Result<i32>;
    fn OffsetFromStart(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResultTimes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>() -> ISpeechRecoResultTimes_Vtbl {
        unsafe extern "system" fn StreamTime<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StreamTime() {
                ::core::result::Result::Ok(ok__) => {
                    *time = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *tickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromStart<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OffsetFromStart() {
                ::core::result::Result::Ok(ok__) => {
                    *offsetfromstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StreamTime: StreamTime::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            OffsetFromStart: OffsetFromStart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResultTimes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecognizer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn putref_Recognizer(&self, recognizer: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn Recognizer(&self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn SetAllowAudioInputFormatChangesOnNextSet(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowAudioInputFormatChangesOnNextSet(&self) -> ::windows::core::Result<i16>;
    fn putref_AudioInput(&self, audioinput: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn AudioInput(&self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn putref_AudioInputStream(&self, audioinputstream: &::core::option::Option<ISpeechBaseStream>) -> ::windows::core::Result<()>;
    fn AudioInputStream(&self) -> ::windows::core::Result<ISpeechBaseStream>;
    fn IsShared(&self) -> ::windows::core::Result<i16>;
    fn SetState(&self, state: SpeechRecognizerState) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<SpeechRecognizerState>;
    fn Status(&self) -> ::windows::core::Result<ISpeechRecognizerStatus>;
    fn putref_Profile(&self, profile: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn Profile(&self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn EmulateRecognition(&self, textelements: &super::super::System::Com::VARIANT, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows::core::Result<()>;
    fn CreateRecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn GetFormat(&self, r#type: SpeechFormatType) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn SetPropertyNumber(&self, name: &super::super::Foundation::BSTR, value: i32) -> ::windows::core::Result<i16>;
    fn GetPropertyNumber(&self, name: &super::super::Foundation::BSTR, value: *mut i32, supported: *mut i16) -> ::windows::core::Result<()>;
    fn SetPropertyString(&self, name: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetPropertyString(&self, name: &super::super::Foundation::BSTR, value: *mut super::super::Foundation::BSTR, supported: *mut i16) -> ::windows::core::Result<()>;
    fn IsUISupported(&self, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn DisplayUI(&self, hwndparent: i32, title: &super::super::Foundation::BSTR, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetRecognizers(&self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn GetAudioInputs(&self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn GetProfiles(&self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>() -> ISpeechRecognizer_Vtbl {
        unsafe extern "system" fn putref_Recognizer<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Recognizer(::core::mem::transmute(&recognizer)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *recognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAudioInputFormatChangesOnNextSet<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowAudioInputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioInputFormatChangesOnNextSet<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowAudioInputFormatChangesOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInput<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AudioInput(::core::mem::transmute(&audioinput)).into()
        }
        unsafe extern "system" fn AudioInput<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioInput() {
                ::core::result::Result::Ok(ok__) => {
                    *audioinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInputStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AudioInputStream(::core::mem::transmute(&audioinputstream)).into()
        }
        unsafe extern "system" fn AudioInputStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *audioinputstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShared<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shared: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsShared() {
                ::core::result::Result::Ok(ok__) => {
                    *shared = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Profile<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Profile(::core::mem::transmute(&profile)).into()
        }
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *profile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textelements: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EmulateRecognition(::core::mem::transmute(&textelements), ::core::mem::transmute_copy(&elementdisplayattributes), ::core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *newcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SpeechFormatType, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetPropertyNumber(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyNumber(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn SetPropertyString<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetPropertyString(::core::mem::transmute(&name), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyString<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyString(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUISupported(::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)).into()
        }
        unsafe extern "system" fn GetRecognizers<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecognizers(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioInputs<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAudioInputs(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfiles<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProfiles(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            putref_Recognizer: putref_Recognizer::<Identity, Impl, OFFSET>,
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            SetAllowAudioInputFormatChangesOnNextSet: SetAllowAudioInputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            AllowAudioInputFormatChangesOnNextSet: AllowAudioInputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            putref_AudioInput: putref_AudioInput::<Identity, Impl, OFFSET>,
            AudioInput: AudioInput::<Identity, Impl, OFFSET>,
            putref_AudioInputStream: putref_AudioInputStream::<Identity, Impl, OFFSET>,
            AudioInputStream: AudioInputStream::<Identity, Impl, OFFSET>,
            IsShared: IsShared::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            putref_Profile: putref_Profile::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, Impl, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            SetPropertyNumber: SetPropertyNumber::<Identity, Impl, OFFSET>,
            GetPropertyNumber: GetPropertyNumber::<Identity, Impl, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, Impl, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            GetRecognizers: GetRecognizers::<Identity, Impl, OFFSET>,
            GetAudioInputs: GetAudioInputs::<Identity, Impl, OFFSET>,
            GetProfiles: GetProfiles::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecognizer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecognizerStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioStatus(&self) -> ::windows::core::Result<ISpeechAudioStatus>;
    fn CurrentStreamPosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CurrentStreamNumber(&self) -> ::windows::core::Result<i32>;
    fn NumberOfActiveRules(&self) -> ::windows::core::Result<i32>;
    fn ClsidEngine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SupportedLanguages(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecognizerStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>() -> ISpeechRecognizerStatus_Vtbl {
        unsafe extern "system" fn AudioStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *audiostatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamPosition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcurrentstreampos: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentStreamPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pcurrentstreampos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfActiveRules<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofactiverules: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfActiveRules() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofactiverules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClsidEngine<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidengine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClsidEngine() {
                ::core::result::Result::Ok(ok__) => {
                    *clsidengine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedLanguages<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedlanguages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedlanguages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AudioStatus: AudioStatus::<Identity, Impl, OFFSET>,
            CurrentStreamPosition: CurrentStreamPosition::<Identity, Impl, OFFSET>,
            CurrentStreamNumber: CurrentStreamNumber::<Identity, Impl, OFFSET>,
            NumberOfActiveRules: NumberOfActiveRules::<Identity, Impl, OFFSET>,
            ClsidEngine: ClsidEngine::<Identity, Impl, OFFSET>,
            SupportedLanguages: SupportedLanguages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecognizerStatus as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechResourceLoader_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LoadResource(&self, bstrresourceuri: &super::super::Foundation::BSTR, falwaysreload: i16, pstream: *mut ::core::option::Option<::windows::core::IUnknown>, pbstrmimetype: *mut super::super::Foundation::BSTR, pfmodified: *mut i16, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetLocalCopy(&self, bstrresourceuri: &super::super::Foundation::BSTR, pbstrlocalpath: *mut super::super::Foundation::BSTR, pbstrmimetype: *mut super::super::Foundation::BSTR, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReleaseLocalCopy(&self, pbstrlocalpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechResourceLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>() -> ISpeechResourceLoader_Vtbl {
        unsafe extern "system" fn LoadResource<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, falwaysreload: i16, pstream: *mut *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR, pfmodified: *mut i16, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadResource(::core::mem::transmute(&bstrresourceuri), ::core::mem::transmute_copy(&falwaysreload), ::core::mem::transmute_copy(&pstream), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pfmodified), ::core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn GetLocalCopy<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlocalpath: *mut super::super::Foundation::BSTR, pbstrmimetype: *mut super::super::Foundation::BSTR, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLocalCopy(::core::mem::transmute(&bstrresourceuri), ::core::mem::transmute_copy(&pbstrlocalpath), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn ReleaseLocalCopy<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlocalpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseLocalCopy(::core::mem::transmute(&pbstrlocalpath)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LoadResource: LoadResource::<Identity, Impl, OFFSET>,
            GetLocalCopy: GetLocalCopy::<Identity, Impl, OFFSET>,
            ReleaseLocalCopy: ReleaseLocalCopy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechResourceLoader as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechTextSelectionInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetActiveOffset(&self, activeoffset: i32) -> ::windows::core::Result<()>;
    fn ActiveOffset(&self) -> ::windows::core::Result<i32>;
    fn SetActiveLength(&self, activelength: i32) -> ::windows::core::Result<()>;
    fn ActiveLength(&self) -> ::windows::core::Result<i32>;
    fn SetSelectionOffset(&self, selectionoffset: i32) -> ::windows::core::Result<()>;
    fn SelectionOffset(&self) -> ::windows::core::Result<i32>;
    fn SetSelectionLength(&self, selectionlength: i32) -> ::windows::core::Result<()>;
    fn SelectionLength(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechTextSelectionInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>() -> ISpeechTextSelectionInformation_Vtbl {
        unsafe extern "system" fn SetActiveOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveOffset(::core::mem::transmute_copy(&activeoffset)).into()
        }
        unsafe extern "system" fn ActiveOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActiveOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *activeoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveLength<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveLength(::core::mem::transmute_copy(&activelength)).into()
        }
        unsafe extern "system" fn ActiveLength<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActiveLength() {
                ::core::result::Result::Ok(ok__) => {
                    *activelength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelectionOffset(::core::mem::transmute_copy(&selectionoffset)).into()
        }
        unsafe extern "system" fn SelectionOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectionOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *selectionoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionLength<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelectionLength(::core::mem::transmute_copy(&selectionlength)).into()
        }
        unsafe extern "system" fn SelectionLength<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectionLength() {
                ::core::result::Result::Ok(ok__) => {
                    *selectionlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetActiveOffset: SetActiveOffset::<Identity, Impl, OFFSET>,
            ActiveOffset: ActiveOffset::<Identity, Impl, OFFSET>,
            SetActiveLength: SetActiveLength::<Identity, Impl, OFFSET>,
            ActiveLength: ActiveLength::<Identity, Impl, OFFSET>,
            SetSelectionOffset: SetSelectionOffset::<Identity, Impl, OFFSET>,
            SelectionOffset: SelectionOffset::<Identity, Impl, OFFSET>,
            SetSelectionLength: SetSelectionLength::<Identity, Impl, OFFSET>,
            SelectionLength: SelectionLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechTextSelectionInformation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechVoice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&self) -> ::windows::core::Result<ISpeechVoiceStatus>;
    fn Voice(&self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn putref_Voice(&self, voice: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn AudioOutput(&self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn putref_AudioOutput(&self, audiooutput: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn AudioOutputStream(&self) -> ::windows::core::Result<ISpeechBaseStream>;
    fn putref_AudioOutputStream(&self, audiooutputstream: &::core::option::Option<ISpeechBaseStream>) -> ::windows::core::Result<()>;
    fn Rate(&self) -> ::windows::core::Result<i32>;
    fn SetRate(&self, rate: i32) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<i32>;
    fn SetVolume(&self, volume: i32) -> ::windows::core::Result<()>;
    fn SetAllowAudioOutputFormatChangesOnNextSet(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowAudioOutputFormatChangesOnNextSet(&self) -> ::windows::core::Result<i16>;
    fn EventInterests(&self) -> ::windows::core::Result<SpeechVoiceEvents>;
    fn SetEventInterests(&self, eventinterestflags: SpeechVoiceEvents) -> ::windows::core::Result<()>;
    fn SetPriority(&self, priority: SpeechVoicePriority) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<SpeechVoicePriority>;
    fn SetAlertBoundary(&self, boundary: SpeechVoiceEvents) -> ::windows::core::Result<()>;
    fn AlertBoundary(&self) -> ::windows::core::Result<SpeechVoiceEvents>;
    fn SetSynchronousSpeakTimeout(&self, mstimeout: i32) -> ::windows::core::Result<()>;
    fn SynchronousSpeakTimeout(&self) -> ::windows::core::Result<i32>;
    fn Speak(&self, text: &super::super::Foundation::BSTR, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn SpeakStream(&self, stream: &::core::option::Option<ISpeechBaseStream>, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, r#type: &super::super::Foundation::BSTR, numitems: i32) -> ::windows::core::Result<i32>;
    fn GetVoices(&self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn GetAudioOutputs(&self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn WaitUntilDone(&self, mstimeout: i32) -> ::windows::core::Result<i16>;
    fn SpeakCompleteEvent(&self) -> ::windows::core::Result<i32>;
    fn IsUISupported(&self, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn DisplayUI(&self, hwndparent: i32, title: &super::super::Foundation::BSTR, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>() -> ISpeechVoice_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Voice<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Voice() {
                ::core::result::Result::Ok(ok__) => {
                    *voice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Voice(::core::mem::transmute(&voice)).into()
        }
        unsafe extern "system" fn AudioOutput<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *audiooutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutput<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AudioOutput(::core::mem::transmute(&audiooutput)).into()
        }
        unsafe extern "system" fn AudioOutputStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *audiooutputstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutputStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AudioOutputStream(::core::mem::transmute(&audiooutputstream)).into()
        }
        unsafe extern "system" fn Rate<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rate() {
                ::core::result::Result::Ok(ok__) => {
                    *rate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRate<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRate(::core::mem::transmute_copy(&rate)).into()
        }
        unsafe extern "system" fn Volume<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *volume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn SetAllowAudioOutputFormatChangesOnNextSet<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowAudioOutputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioOutputFormatChangesOnNextSet<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowAudioOutputFormatChangesOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterests<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventInterests() {
                ::core::result::Result::Ok(ok__) => {
                    *eventinterestflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventInterests(::core::mem::transmute_copy(&eventinterestflags)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: SpeechVoicePriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: *mut SpeechVoicePriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *priority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlertBoundary(::core::mem::transmute_copy(&boundary)).into()
        }
        unsafe extern "system" fn AlertBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AlertBoundary() {
                ::core::result::Result::Ok(ok__) => {
                    *boundary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynchronousSpeakTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSynchronousSpeakTimeout(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SynchronousSpeakTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SynchronousSpeakTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *mstimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Speak(::core::mem::transmute(&text), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpeakStream(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numitems: i32, numskipped: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Skip(::core::mem::transmute(&r#type), ::core::mem::transmute_copy(&numitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *numskipped = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoices<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVoices(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioOutputs<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAudioOutputs(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32, done: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WaitUntilDone(::core::mem::transmute_copy(&mstimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *done = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpeakCompleteEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *handle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUISupported(::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Voice: Voice::<Identity, Impl, OFFSET>,
            putref_Voice: putref_Voice::<Identity, Impl, OFFSET>,
            AudioOutput: AudioOutput::<Identity, Impl, OFFSET>,
            putref_AudioOutput: putref_AudioOutput::<Identity, Impl, OFFSET>,
            AudioOutputStream: AudioOutputStream::<Identity, Impl, OFFSET>,
            putref_AudioOutputStream: putref_AudioOutputStream::<Identity, Impl, OFFSET>,
            Rate: Rate::<Identity, Impl, OFFSET>,
            SetRate: SetRate::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            SetAllowAudioOutputFormatChangesOnNextSet: SetAllowAudioOutputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            AllowAudioOutputFormatChangesOnNextSet: AllowAudioOutputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            EventInterests: EventInterests::<Identity, Impl, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, Impl, OFFSET>,
            AlertBoundary: AlertBoundary::<Identity, Impl, OFFSET>,
            SetSynchronousSpeakTimeout: SetSynchronousSpeakTimeout::<Identity, Impl, OFFSET>,
            SynchronousSpeakTimeout: SynchronousSpeakTimeout::<Identity, Impl, OFFSET>,
            Speak: Speak::<Identity, Impl, OFFSET>,
            SpeakStream: SpeakStream::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            GetVoices: GetVoices::<Identity, Impl, OFFSET>,
            GetAudioOutputs: GetAudioOutputs::<Identity, Impl, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, Impl, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechVoice as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechVoiceStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentStreamNumber(&self) -> ::windows::core::Result<i32>;
    fn LastStreamNumberQueued(&self) -> ::windows::core::Result<i32>;
    fn LastHResult(&self) -> ::windows::core::Result<i32>;
    fn RunningState(&self) -> ::windows::core::Result<SpeechRunState>;
    fn InputWordPosition(&self) -> ::windows::core::Result<i32>;
    fn InputWordLength(&self) -> ::windows::core::Result<i32>;
    fn InputSentencePosition(&self) -> ::windows::core::Result<i32>;
    fn InputSentenceLength(&self) -> ::windows::core::Result<i32>;
    fn LastBookmark(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LastBookmarkId(&self) -> ::windows::core::Result<i32>;
    fn PhonemeId(&self) -> ::windows::core::Result<i16>;
    fn VisemeId(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechVoiceStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>() -> ISpeechVoiceStatus_Vtbl {
        unsafe extern "system" fn CurrentStreamNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastStreamNumberQueued<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastStreamNumberQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastHResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastHResult() {
                ::core::result::Result::Ok(ok__) => {
                    *hresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningState<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRunState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RunningState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordPosition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InputWordPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *position = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordLength<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InputWordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentencePosition<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InputSentencePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *position = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentenceLength<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InputSentenceLength() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmark<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastBookmark() {
                ::core::result::Result::Ok(ok__) => {
                    *bookmark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmarkId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmarkid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastBookmarkId() {
                ::core::result::Result::Ok(ok__) => {
                    *bookmarkid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhonemeId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhonemeId() {
                ::core::result::Result::Ok(ok__) => {
                    *phoneid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisemeId<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visemeid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VisemeId() {
                ::core::result::Result::Ok(ok__) => {
                    *visemeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentStreamNumber: CurrentStreamNumber::<Identity, Impl, OFFSET>,
            LastStreamNumberQueued: LastStreamNumberQueued::<Identity, Impl, OFFSET>,
            LastHResult: LastHResult::<Identity, Impl, OFFSET>,
            RunningState: RunningState::<Identity, Impl, OFFSET>,
            InputWordPosition: InputWordPosition::<Identity, Impl, OFFSET>,
            InputWordLength: InputWordLength::<Identity, Impl, OFFSET>,
            InputSentencePosition: InputSentencePosition::<Identity, Impl, OFFSET>,
            InputSentenceLength: InputSentenceLength::<Identity, Impl, OFFSET>,
            LastBookmark: LastBookmark::<Identity, Impl, OFFSET>,
            LastBookmarkId: LastBookmarkId::<Identity, Impl, OFFSET>,
            PhonemeId: PhonemeId::<Identity, Impl, OFFSET>,
            VisemeId: VisemeId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechVoiceStatus as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechWaveFormatEx_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FormatTag(&self) -> ::windows::core::Result<i16>;
    fn SetFormatTag(&self, formattag: i16) -> ::windows::core::Result<()>;
    fn Channels(&self) -> ::windows::core::Result<i16>;
    fn SetChannels(&self, channels: i16) -> ::windows::core::Result<()>;
    fn SamplesPerSec(&self) -> ::windows::core::Result<i32>;
    fn SetSamplesPerSec(&self, samplespersec: i32) -> ::windows::core::Result<()>;
    fn AvgBytesPerSec(&self) -> ::windows::core::Result<i32>;
    fn SetAvgBytesPerSec(&self, avgbytespersec: i32) -> ::windows::core::Result<()>;
    fn BlockAlign(&self) -> ::windows::core::Result<i16>;
    fn SetBlockAlign(&self, blockalign: i16) -> ::windows::core::Result<()>;
    fn BitsPerSample(&self) -> ::windows::core::Result<i16>;
    fn SetBitsPerSample(&self, bitspersample: i16) -> ::windows::core::Result<()>;
    fn ExtraData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExtraData(&self, extradata: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechWaveFormatEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>() -> ISpeechWaveFormatEx_Vtbl {
        unsafe extern "system" fn FormatTag<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatTag() {
                ::core::result::Result::Ok(ok__) => {
                    *formattag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormatTag(::core::mem::transmute_copy(&formattag)).into()
        }
        unsafe extern "system" fn Channels<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Channels() {
                ::core::result::Result::Ok(ok__) => {
                    *channels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChannels(::core::mem::transmute_copy(&channels)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SamplesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *samplespersec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSamplesPerSec(::core::mem::transmute_copy(&samplespersec)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AvgBytesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *avgbytespersec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAvgBytesPerSec(::core::mem::transmute_copy(&avgbytespersec)).into()
        }
        unsafe extern "system" fn BlockAlign<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BlockAlign() {
                ::core::result::Result::Ok(ok__) => {
                    *blockalign = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlockAlign(::core::mem::transmute_copy(&blockalign)).into()
        }
        unsafe extern "system" fn BitsPerSample<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    *bitspersample = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBitsPerSample(::core::mem::transmute_copy(&bitspersample)).into()
        }
        unsafe extern "system" fn ExtraData<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtraData() {
                ::core::result::Result::Ok(ok__) => {
                    *extradata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtraData<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtraData(::core::mem::transmute(&extradata)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FormatTag: FormatTag::<Identity, Impl, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, Impl, OFFSET>,
            Channels: Channels::<Identity, Impl, OFFSET>,
            SetChannels: SetChannels::<Identity, Impl, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, Impl, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, Impl, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, Impl, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, Impl, OFFSET>,
            BlockAlign: BlockAlign::<Identity, Impl, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, Impl, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, Impl, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, Impl, OFFSET>,
            ExtraData: ExtraData::<Identity, Impl, OFFSET>,
            SetExtraData: SetExtraData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechWaveFormatEx as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechXMLRecoResult_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechRecoResult_Impl {
    fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut i32, iserror: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechXMLRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>() -> ISpeechXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXMLResult(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut i32, iserror: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into()
        }
        Self {
            base: ISpeechRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechXMLRecoResult as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISpeechRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ISpeechRecoContextEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ISpeechRecoContextEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISpeechRecoContextEvents_Impl, const OFFSET: isize>() -> _ISpeechRecoContextEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ISpeechRecoContextEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ISpeechVoiceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ISpeechVoiceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISpeechVoiceEvents_Impl, const OFFSET: isize>() -> _ISpeechVoiceEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ISpeechVoiceEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
