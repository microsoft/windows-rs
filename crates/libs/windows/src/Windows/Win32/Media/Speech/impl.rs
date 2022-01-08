pub trait IEnumSpObjectTokensImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn Item();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumSpObjectTokens {
    const NAME: &'static str = "Windows.Win32.Media.Speech.IEnumSpObjectTokens";
}
impl IEnumSpObjectTokensVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokensImpl, const OFFSET: isize>() -> IEnumSpObjectTokensVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSpObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&pelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSpObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSpObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumSpObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Item<Impl: IEnumSpObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pptoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumSpObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(pcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumSpObjectTokens>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, Item::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpAudioImpl: Sized + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn SetState();
    fn SetFormat();
    fn GetStatus();
    fn SetBufferInfo();
    fn GetBufferInfo();
    fn GetDefaultFormat();
    fn EventHandle();
    fn GetVolumeLevel();
    fn SetVolumeLevel();
    fn GetBufferNotifySize();
    fn SetBufferNotifySize();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpAudio {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpAudio";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpAudioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudioImpl, const OFFSET: isize>() -> ISpAudioVtbl {
        unsafe extern "system" fn SetState<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(newstate, ullreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidfmtid: &::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormat(&*(&rguidfmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pwaveformatex as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(&*(&pstatus as *const <SPAUDIOSTATUS as ::windows::core::Abi>::Abi as *const <SPAUDIOSTATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferInfo<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferInfo(&*(&pbuffinfo as *const <SPAUDIOBUFFERINFO as ::windows::core::Abi>::Abi as *const <SPAUDIOBUFFERINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferInfo<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferInfo(&*(&pbuffinfo as *const <SPAUDIOBUFFERINFO as ::windows::core::Abi>::Abi as *const <SPAUDIOBUFFERINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFormat<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultFormat(&*(&pformatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppcomemwaveformatex as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventHandle<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeLevel<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeLevel(plevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeLevel<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolumeLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferNotifySize<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferNotifySize(pcbsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Impl: ISpAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferNotifySize(cbsize) {
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
            ::windows::core::GetRuntimeClassName::<ISpAudio>,
            ::windows::core::GetTrustLevel,
            SetState::<Impl, OFFSET>,
            SetFormat::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            SetBufferInfo::<Impl, OFFSET>,
            GetBufferInfo::<Impl, OFFSET>,
            GetDefaultFormat::<Impl, OFFSET>,
            EventHandle::<Impl, OFFSET>,
            GetVolumeLevel::<Impl, OFFSET>,
            SetVolumeLevel::<Impl, OFFSET>,
            GetBufferNotifySize::<Impl, OFFSET>,
            SetBufferNotifySize::<Impl, OFFSET>,
        )
    }
}
pub trait ISpContainerLexiconImpl: Sized + ISpLexiconImpl {
    fn AddLexicon();
}
impl ::windows::core::RuntimeName for ISpContainerLexicon {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpContainerLexicon";
}
impl ISpContainerLexiconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpContainerLexiconImpl, const OFFSET: isize>() -> ISpContainerLexiconVtbl {
        unsafe extern "system" fn AddLexicon<Impl: ISpContainerLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddlexicon: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddLexicon(&*(&paddlexicon as *const <ISpLexicon as ::windows::core::Abi>::Abi as *const <ISpLexicon as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpContainerLexicon>, ::windows::core::GetTrustLevel, AddLexicon::<Impl, OFFSET>)
    }
}
pub trait ISpDataKeyImpl: Sized {
    fn SetData();
    fn GetData();
    fn SetStringValue();
    fn GetStringValue();
    fn SetDWORD();
    fn GetDWORD();
    fn OpenKey();
    fn CreateKey();
    fn DeleteKey();
    fn DeleteValue();
    fn EnumKeys();
    fn EnumValues();
}
impl ::windows::core::RuntimeName for ISpDataKey {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpDataKey";
}
impl ISpDataKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKeyImpl, const OFFSET: isize>() -> ISpDataKeyVtbl {
        unsafe extern "system" fn SetData<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, cbdata: u32, pdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cbdata, pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcbdata, pdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStringValue(&*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(&*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDWORD<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDWORD(&*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDWORD<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, pdwvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDWORD(&*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pdwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenKey<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkeyname: super::super::Foundation::PWSTR, ppsubkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenKey(&*(&pszsubkeyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsubkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: super::super::Foundation::PWSTR, ppsubkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKey(&*(&pszsubkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsubkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteKey(&*(&pszsubkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteValue<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteValue(&*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumKeys<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszsubkeyname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumKeys(index, ::core::mem::transmute_copy(&ppszsubkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Impl: ISpDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszvaluename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumValues(index, ::core::mem::transmute_copy(&ppszvaluename)) {
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
            ::windows::core::GetRuntimeClassName::<ISpDataKey>,
            ::windows::core::GetTrustLevel,
            SetData::<Impl, OFFSET>,
            GetData::<Impl, OFFSET>,
            SetStringValue::<Impl, OFFSET>,
            GetStringValue::<Impl, OFFSET>,
            SetDWORD::<Impl, OFFSET>,
            GetDWORD::<Impl, OFFSET>,
            OpenKey::<Impl, OFFSET>,
            CreateKey::<Impl, OFFSET>,
            DeleteKey::<Impl, OFFSET>,
            DeleteValue::<Impl, OFFSET>,
            EnumKeys::<Impl, OFFSET>,
            EnumValues::<Impl, OFFSET>,
        )
    }
}
pub trait ISpDisplayAlternatesImpl: Sized {
    fn GetDisplayAlternates();
    fn SetFullStopTrailSpace();
}
impl ::windows::core::RuntimeName for ISpDisplayAlternates {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpDisplayAlternates";
}
impl ISpDisplayAlternatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpDisplayAlternatesImpl, const OFFSET: isize>() -> ISpDisplayAlternatesVtbl {
        unsafe extern "system" fn GetDisplayAlternates<Impl: ISpDisplayAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayAlternates(&*(&pphrase as *const <SPDISPLAYPHRASE as ::windows::core::Abi>::Abi as *const <SPDISPLAYPHRASE as ::windows::core::DefaultType>::DefaultType), crequestcount, &*(&ppcomemphrases as *const <SPDISPLAYPHRASE as ::windows::core::Abi>::Abi as *const <SPDISPLAYPHRASE as ::windows::core::DefaultType>::DefaultType), pcphrasesreturned) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullStopTrailSpace<Impl: ISpDisplayAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultrailspace: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFullStopTrailSpace(ultrailspace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpDisplayAlternates>, ::windows::core::GetTrustLevel, GetDisplayAlternates::<Impl, OFFSET>, SetFullStopTrailSpace::<Impl, OFFSET>)
    }
}
pub trait ISpEnginePronunciationImpl: Sized {
    fn Normalize();
    fn GetPronunciations();
}
impl ::windows::core::RuntimeName for ISpEnginePronunciation {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpEnginePronunciation";
}
impl ISpEnginePronunciationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEnginePronunciationImpl, const OFFSET: isize>() -> ISpEnginePronunciationVtbl {
        unsafe extern "system" fn Normalize<Impl: ISpEnginePronunciationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, pszleftcontext: super::super::Foundation::PWSTR, pszrightcontext: super::super::Foundation::PWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Normalize(
                &*(&pszword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszleftcontext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszrightcontext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&pnormalizationlist as *const <SPNORMALIZATIONLIST as ::windows::core::Abi>::Abi as *const <SPNORMALIZATIONLIST as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPronunciations<Impl: ISpEnginePronunciationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, pszleftcontext: super::super::Foundation::PWSTR, pszrightcontext: super::super::Foundation::PWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPronunciations(
                &*(&pszword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszleftcontext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszrightcontext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&penginepronunciationlist as *const <SPWORDPRONUNCIATIONLIST as ::windows::core::Abi>::Abi as *const <SPWORDPRONUNCIATIONLIST as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpEnginePronunciation>, ::windows::core::GetTrustLevel, Normalize::<Impl, OFFSET>, GetPronunciations::<Impl, OFFSET>)
    }
}
pub trait ISpEventSinkImpl: Sized {
    fn AddEvents();
    fn GetEventInterest();
}
impl ::windows::core::RuntimeName for ISpEventSink {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpEventSink";
}
impl ISpEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSinkImpl, const OFFSET: isize>() -> ISpEventSinkVtbl {
        unsafe extern "system" fn AddEvents<Impl: ISpEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventarray: *const SPEVENT, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEvents(&*(&peventarray as *const <SPEVENT as ::windows::core::Abi>::Abi as *const <SPEVENT as ::windows::core::DefaultType>::DefaultType), ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventInterest<Impl: ISpEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventInterest(pulleventinterest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpEventSink>, ::windows::core::GetTrustLevel, AddEvents::<Impl, OFFSET>, GetEventInterest::<Impl, OFFSET>)
    }
}
pub trait ISpEventSourceImpl: Sized + ISpNotifySourceImpl {
    fn SetInterest();
    fn GetEvents();
    fn GetInfo();
}
impl ::windows::core::RuntimeName for ISpEventSource {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpEventSource";
}
impl ISpEventSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSourceImpl, const OFFSET: isize>() -> ISpEventSourceVtbl {
        unsafe extern "system" fn SetInterest<Impl: ISpEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInterest(ulleventinterest, ullqueuedinterest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEvents<Impl: ISpEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvents(ulcount, &*(&peventarray as *const <SPEVENT as ::windows::core::Abi>::Abi as *const <SPEVENT as ::windows::core::DefaultType>::DefaultType), pulfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Impl: ISpEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo(&*(&pinfo as *const <SPEVENTSOURCEINFO as ::windows::core::Abi>::Abi as *const <SPEVENTSOURCEINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpEventSource>, ::windows::core::GetTrustLevel, SetInterest::<Impl, OFFSET>, GetEvents::<Impl, OFFSET>, GetInfo::<Impl, OFFSET>)
    }
}
pub trait ISpEventSource2Impl: Sized + ISpEventSourceImpl + ISpNotifySourceImpl {
    fn GetEventsEx();
}
impl ::windows::core::RuntimeName for ISpEventSource2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpEventSource2";
}
impl ISpEventSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource2Impl, const OFFSET: isize>() -> ISpEventSource2Vtbl {
        unsafe extern "system" fn GetEventsEx<Impl: ISpEventSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventsEx(ulcount, &*(&peventarray as *const <SPEVENTEX as ::windows::core::Abi>::Abi as *const <SPEVENTEX as ::windows::core::DefaultType>::DefaultType), pulfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpEventSource2>, ::windows::core::GetTrustLevel, GetEventsEx::<Impl, OFFSET>)
    }
}
pub trait ISpGrammarBuilderImpl: Sized {
    fn ResetGrammar();
    fn GetRule();
    fn ClearRule();
    fn CreateNewState();
    fn AddWordTransition();
    fn AddRuleTransition();
    fn AddResource();
    fn Commit();
}
impl ::windows::core::RuntimeName for ISpGrammarBuilder {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpGrammarBuilder";
}
impl ISpGrammarBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilderImpl, const OFFSET: isize>() -> ISpGrammarBuilderVtbl {
        unsafe extern "system" fn ResetGrammar<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetGrammar(newlanguage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRule<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: super::super::Foundation::PWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRule(
                &*(&pszrulename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwruleid,
                dwattributes,
                &*(&fcreateifnotexist as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&phinitialstate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRule<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearRule(&*(&hstate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewState<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewState(&*(&hstate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType), &*(&phstate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWordTransition<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: super::super::Foundation::PWSTR, pszseparators: super::super::Foundation::PWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWordTransition(
                &*(&hfromstate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType),
                &*(&htostate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType),
                &*(&psz as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszseparators as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ewordtype,
                weight,
                &*(&ppropinfo as *const <SPPROPERTYINFO as ::windows::core::Abi>::Abi as *const <SPPROPERTYINFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRuleTransition<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRuleTransition(
                &*(&hfromstate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType),
                &*(&htostate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType),
                &*(&hrule as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType),
                weight,
                &*(&ppropinfo as *const <SPPROPERTYINFO as ::windows::core::Abi>::Abi as *const <SPPROPERTYINFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddResource<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: super::super::Foundation::PWSTR, pszresourcevalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddResource(
                &*(&hrulestate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType),
                &*(&pszresourcename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszresourcevalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ISpGrammarBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit(dwreserved) {
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
            ::windows::core::GetRuntimeClassName::<ISpGrammarBuilder>,
            ::windows::core::GetTrustLevel,
            ResetGrammar::<Impl, OFFSET>,
            GetRule::<Impl, OFFSET>,
            ClearRule::<Impl, OFFSET>,
            CreateNewState::<Impl, OFFSET>,
            AddWordTransition::<Impl, OFFSET>,
            AddRuleTransition::<Impl, OFFSET>,
            AddResource::<Impl, OFFSET>,
            Commit::<Impl, OFFSET>,
        )
    }
}
pub trait ISpGrammarBuilder2Impl: Sized {
    fn AddTextSubset();
    fn SetPhoneticAlphabet();
}
impl ::windows::core::RuntimeName for ISpGrammarBuilder2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpGrammarBuilder2";
}
impl ISpGrammarBuilder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder2Impl, const OFFSET: isize>() -> ISpGrammarBuilder2Vtbl {
        unsafe extern "system" fn AddTextSubset<Impl: ISpGrammarBuilder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: super::super::Foundation::PWSTR, ematchmode: SPMATCHINGMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTextSubset(&*(&hfromstate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType), &*(&htostate as *const <SPSTATEHANDLE__ as ::windows::core::Abi>::Abi as *const <SPSTATEHANDLE__ as ::windows::core::DefaultType>::DefaultType), &*(&psz as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ematchmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneticAlphabet<Impl: ISpGrammarBuilder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneticalphabet: PHONETICALPHABET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPhoneticAlphabet(phoneticalphabet) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpGrammarBuilder2>, ::windows::core::GetTrustLevel, AddTextSubset::<Impl, OFFSET>, SetPhoneticAlphabet::<Impl, OFFSET>)
    }
}
pub trait ISpLexiconImpl: Sized {
    fn GetPronunciations();
    fn AddPronunciation();
    fn RemovePronunciation();
    fn GetGeneration();
    fn GetGenerationChange();
    fn GetWords();
}
impl ::windows::core::RuntimeName for ISpLexicon {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpLexicon";
}
impl ISpLexiconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexiconImpl, const OFFSET: isize>() -> ISpLexiconVtbl {
        unsafe extern "system" fn GetPronunciations<Impl: ISpLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPronunciations(&*(&pszword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), langid, dwflags, &*(&pwordpronunciationlist as *const <SPWORDPRONUNCIATIONLIST as ::windows::core::Abi>::Abi as *const <SPWORDPRONUNCIATIONLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPronunciation<Impl: ISpLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPronunciation(&*(&pszword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), langid, epartofspeech, pszpronunciation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePronunciation<Impl: ISpLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePronunciation(&*(&pszword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), langid, epartofspeech, pszpronunciation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeneration<Impl: ISpLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneration(pdwgeneration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationChange<Impl: ISpLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenerationChange(dwflags, pdwgeneration, &*(&pwordlist as *const <SPWORDLIST as ::windows::core::Abi>::Abi as *const <SPWORDLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Impl: ISpLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWords(dwflags, pdwgeneration, pdwcookie, &*(&pwordlist as *const <SPWORDLIST as ::windows::core::Abi>::Abi as *const <SPWORDLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpLexicon>, ::windows::core::GetTrustLevel, GetPronunciations::<Impl, OFFSET>, AddPronunciation::<Impl, OFFSET>, RemovePronunciation::<Impl, OFFSET>, GetGeneration::<Impl, OFFSET>, GetGenerationChange::<Impl, OFFSET>, GetWords::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpMMSysAudioImpl: Sized + ISpAudioImpl + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn GetDeviceId();
    fn SetDeviceId();
    fn GetMMHandle();
    fn GetLineId();
    fn SetLineId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpMMSysAudio {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpMMSysAudio";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpMMSysAudioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudioImpl, const OFFSET: isize>() -> ISpMMSysAudioVtbl {
        unsafe extern "system" fn GetDeviceId<Impl: ISpMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudeviceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceId(pudeviceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Impl: ISpMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udeviceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeviceId(udeviceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMMHandle<Impl: ISpMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMMHandle(&*(&phandle as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineId<Impl: ISpMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulineid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineId(pulineid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineId<Impl: ISpMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulineid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineId(ulineid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpMMSysAudio>, ::windows::core::GetTrustLevel, GetDeviceId::<Impl, OFFSET>, SetDeviceId::<Impl, OFFSET>, GetMMHandle::<Impl, OFFSET>, GetLineId::<Impl, OFFSET>, SetLineId::<Impl, OFFSET>)
    }
}
pub trait ISpNotifyCallbackImpl: Sized {
    fn NotifyCallback();
}
impl ::windows::core::RuntimeName for ISpNotifyCallback {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpNotifyCallback";
}
impl ISpNotifyCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyCallbackImpl, const OFFSET: isize>() -> ISpNotifyCallbackVtbl {
        unsafe extern "system" fn NotifyCallback<Impl: ISpNotifyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyCallback(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpNotifyCallback>, ::windows::core::GetTrustLevel, NotifyCallback::<Impl, OFFSET>)
    }
}
pub trait ISpNotifySinkImpl: Sized {
    fn Notify();
}
impl ::windows::core::RuntimeName for ISpNotifySink {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpNotifySink";
}
impl ISpNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySinkImpl, const OFFSET: isize>() -> ISpNotifySinkVtbl {
        unsafe extern "system" fn Notify<Impl: ISpNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpNotifySink>, ::windows::core::GetTrustLevel, Notify::<Impl, OFFSET>)
    }
}
pub trait ISpNotifySourceImpl: Sized {
    fn SetNotifySink();
    fn SetNotifyWindowMessage();
    fn SetNotifyCallbackFunction();
    fn SetNotifyCallbackInterface();
    fn SetNotifyWin32Event();
    fn WaitForNotifyEvent();
    fn GetNotifyEventHandle();
}
impl ::windows::core::RuntimeName for ISpNotifySource {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpNotifySource";
}
impl ISpNotifySourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySourceImpl, const OFFSET: isize>() -> ISpNotifySourceVtbl {
        unsafe extern "system" fn SetNotifySink<Impl: ISpNotifySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotifySink(&*(&pnotifysink as *const <ISpNotifySink as ::windows::core::Abi>::Abi as *const <ISpNotifySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyWindowMessage<Impl: ISpNotifySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotifyWindowMessage(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyCallbackFunction<Impl: ISpNotifySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotifyCallbackFunction(
                &*(&pfncallback as *const <SPNOTIFYCALLBACK as ::windows::core::Abi>::Abi as *const <SPNOTIFYCALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyCallbackInterface<Impl: ISpNotifySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotifyCallbackInterface(
                &*(&pspcallback as *const <ISpNotifyCallback as ::windows::core::Abi>::Abi as *const <ISpNotifyCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyWin32Event<Impl: ISpNotifySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotifyWin32Event() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForNotifyEvent<Impl: ISpNotifySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForNotifyEvent(dwmilliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotifyEventHandle<Impl: ISpNotifySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotifyEventHandle() {
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
            ::windows::core::GetRuntimeClassName::<ISpNotifySource>,
            ::windows::core::GetTrustLevel,
            SetNotifySink::<Impl, OFFSET>,
            SetNotifyWindowMessage::<Impl, OFFSET>,
            SetNotifyCallbackFunction::<Impl, OFFSET>,
            SetNotifyCallbackInterface::<Impl, OFFSET>,
            SetNotifyWin32Event::<Impl, OFFSET>,
            WaitForNotifyEvent::<Impl, OFFSET>,
            GetNotifyEventHandle::<Impl, OFFSET>,
        )
    }
}
pub trait ISpNotifyTranslatorImpl: Sized + ISpNotifySinkImpl {
    fn InitWindowMessage();
    fn InitCallback();
    fn InitSpNotifyCallback();
    fn InitWin32Event();
    fn Wait();
    fn GetEventHandle();
}
impl ::windows::core::RuntimeName for ISpNotifyTranslator {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpNotifyTranslator";
}
impl ISpNotifyTranslatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslatorImpl, const OFFSET: isize>() -> ISpNotifyTranslatorVtbl {
        unsafe extern "system" fn InitWindowMessage<Impl: ISpNotifyTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitWindowMessage(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitCallback<Impl: ISpNotifyTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitCallback(
                &*(&pfncallback as *const <SPNOTIFYCALLBACK as ::windows::core::Abi>::Abi as *const <SPNOTIFYCALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitSpNotifyCallback<Impl: ISpNotifyTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitSpNotifyCallback(
                &*(&pspcallback as *const <ISpNotifyCallback as ::windows::core::Abi>::Abi as *const <ISpNotifyCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitWin32Event<Impl: ISpNotifyTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitWin32Event(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&fclosehandleonrelease as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wait<Impl: ISpNotifyTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wait(dwmilliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventHandle<Impl: ISpNotifyTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpNotifyTranslator>, ::windows::core::GetTrustLevel, InitWindowMessage::<Impl, OFFSET>, InitCallback::<Impl, OFFSET>, InitSpNotifyCallback::<Impl, OFFSET>, InitWin32Event::<Impl, OFFSET>, Wait::<Impl, OFFSET>, GetEventHandle::<Impl, OFFSET>)
    }
}
pub trait ISpObjectTokenImpl: Sized + ISpDataKeyImpl {
    fn SetId();
    fn GetId();
    fn GetCategory();
    fn CreateInstance();
    fn GetStorageFileName();
    fn RemoveStorageFileName();
    fn Remove();
    fn IsUISupported();
    fn DisplayUI();
    fn MatchesAttributes();
}
impl ::windows::core::RuntimeName for ISpObjectToken {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpObjectToken";
}
impl ISpObjectTokenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenImpl, const OFFSET: isize>() -> ISpObjectTokenVtbl {
        unsafe extern "system" fn SetId<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: super::super::Foundation::PWSTR, psztokenid: super::super::Foundation::PWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetId(
                &*(&pszcategoryid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psztokenid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fcreateifnotexist as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&ppszcomemtokenid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptokencategory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory(::core::mem::transmute_copy(&pptokencategory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: &::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwclscontext,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&ppvobject as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageFileName<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: &::windows::core::GUID, pszvaluename: super::super::Foundation::PWSTR, pszfilenamespecifier: super::super::Foundation::PWSTR, nfolder: u32, ppszfilepath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFileName(
                &*(&clsidcaller as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszfilenamespecifier as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                nfolder,
                ::core::mem::transmute_copy(&ppszfilepath),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: &::windows::core::GUID, pszkeyname: super::super::Foundation::PWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStorageFileName(
                &*(&clsidcaller as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszkeyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fdeletefile as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidcaller: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&pclsidcaller as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(
                &*(&psztypeofui as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvextradata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbextradata,
                &*(&punkobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pfsupported as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUI(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&psztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psztypeofui as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvextradata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbextradata,
                &*(&punkobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesAttributes<Impl: ISpObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributes: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesAttributes(&*(&pszattributes as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pfmatches as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISpObjectToken>,
            ::windows::core::GetTrustLevel,
            SetId::<Impl, OFFSET>,
            GetId::<Impl, OFFSET>,
            GetCategory::<Impl, OFFSET>,
            CreateInstance::<Impl, OFFSET>,
            GetStorageFileName::<Impl, OFFSET>,
            RemoveStorageFileName::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            IsUISupported::<Impl, OFFSET>,
            DisplayUI::<Impl, OFFSET>,
            MatchesAttributes::<Impl, OFFSET>,
        )
    }
}
pub trait ISpObjectTokenCategoryImpl: Sized + ISpDataKeyImpl {
    fn SetId();
    fn GetId();
    fn GetDataKey();
    fn EnumTokens();
    fn SetDefaultTokenId();
    fn GetDefaultTokenId();
}
impl ::windows::core::RuntimeName for ISpObjectTokenCategory {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpObjectTokenCategory";
}
impl ISpObjectTokenCategoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategoryImpl, const OFFSET: isize>() -> ISpObjectTokenCategoryVtbl {
        unsafe extern "system" fn SetId<Impl: ISpObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: super::super::Foundation::PWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetId(&*(&pszcategoryid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&fcreateifnotexist as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: ISpObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategoryid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&ppszcomemcategoryid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataKey<Impl: ISpObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataKey(spdkl, ::core::mem::transmute_copy(&ppdatakey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTokens<Impl: ISpObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pzsreqattribs: super::super::Foundation::PWSTR, pszoptattribs: super::super::Foundation::PWSTR, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTokens(&*(&pzsreqattribs as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoptattribs as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTokenId<Impl: ISpObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztokenid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultTokenId(&*(&psztokenid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultTokenId<Impl: ISpObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultTokenId(::core::mem::transmute_copy(&ppszcomemtokenid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpObjectTokenCategory>, ::windows::core::GetTrustLevel, SetId::<Impl, OFFSET>, GetId::<Impl, OFFSET>, GetDataKey::<Impl, OFFSET>, EnumTokens::<Impl, OFFSET>, SetDefaultTokenId::<Impl, OFFSET>, GetDefaultTokenId::<Impl, OFFSET>)
    }
}
pub trait ISpObjectTokenInitImpl: Sized + ISpObjectTokenImpl + ISpDataKeyImpl {
    fn InitFromDataKey();
}
impl ::windows::core::RuntimeName for ISpObjectTokenInit {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpObjectTokenInit";
}
impl ISpObjectTokenInitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenInitImpl, const OFFSET: isize>() -> ISpObjectTokenInitVtbl {
        unsafe extern "system" fn InitFromDataKey<Impl: ISpObjectTokenInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: super::super::Foundation::PWSTR, psztokenid: super::super::Foundation::PWSTR, pdatakey: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitFromDataKey(
                &*(&pszcategoryid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psztokenid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdatakey as *const <ISpDataKey as ::windows::core::Abi>::Abi as *const <ISpDataKey as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpObjectTokenInit>, ::windows::core::GetTrustLevel, InitFromDataKey::<Impl, OFFSET>)
    }
}
pub trait ISpObjectWithTokenImpl: Sized {
    fn SetObjectToken();
    fn GetObjectToken();
}
impl ::windows::core::RuntimeName for ISpObjectWithToken {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpObjectWithToken";
}
impl ISpObjectWithTokenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectWithTokenImpl, const OFFSET: isize>() -> ISpObjectWithTokenVtbl {
        unsafe extern "system" fn SetObjectToken<Impl: ISpObjectWithTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectToken(&*(&ptoken as *const <ISpObjectToken as ::windows::core::Abi>::Abi as *const <ISpObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectToken<Impl: ISpObjectWithTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectToken(::core::mem::transmute_copy(&pptoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpObjectWithToken>, ::windows::core::GetTrustLevel, SetObjectToken::<Impl, OFFSET>, GetObjectToken::<Impl, OFFSET>)
    }
}
pub trait ISpPhoneConverterImpl: Sized + ISpObjectWithTokenImpl {
    fn PhoneToId();
    fn IdToPhone();
}
impl ::windows::core::RuntimeName for ISpPhoneConverter {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpPhoneConverter";
}
impl ISpPhoneConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneConverterImpl, const OFFSET: isize>() -> ISpPhoneConverterVtbl {
        unsafe extern "system" fn PhoneToId<Impl: ISpPhoneConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszphone: super::super::Foundation::PWSTR, pid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneToId(&*(&pszphone as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Impl: ISpPhoneConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *const u16, pszphone: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdToPhone(pid, ::core::mem::transmute_copy(&pszphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpPhoneConverter>, ::windows::core::GetTrustLevel, PhoneToId::<Impl, OFFSET>, IdToPhone::<Impl, OFFSET>)
    }
}
pub trait ISpPhoneticAlphabetConverterImpl: Sized {
    fn GetLangId();
    fn SetLangId();
    fn SAPI2UPS();
    fn UPS2SAPI();
    fn GetMaxConvertLength();
}
impl ::windows::core::RuntimeName for ISpPhoneticAlphabetConverter {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpPhoneticAlphabetConverter";
}
impl ISpPhoneticAlphabetConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverterImpl, const OFFSET: isize>() -> ISpPhoneticAlphabetConverterVtbl {
        unsafe extern "system" fn GetLangId<Impl: ISpPhoneticAlphabetConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLangId(::core::mem::transmute_copy(&plangid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLangId<Impl: ISpPhoneticAlphabetConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLangId(langid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SAPI2UPS<Impl: ISpPhoneticAlphabetConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SAPI2UPS(pszsapiid, ::core::mem::transmute_copy(&pszupsid), cmaxlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UPS2SAPI<Impl: ISpPhoneticAlphabetConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UPS2SAPI(pszupsid, ::core::mem::transmute_copy(&pszsapiid), cmaxlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxConvertLength<Impl: ISpPhoneticAlphabetConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxConvertLength(csrclength, &*(&bsapi2ups as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcmaxdestlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpPhoneticAlphabetConverter>, ::windows::core::GetTrustLevel, GetLangId::<Impl, OFFSET>, SetLangId::<Impl, OFFSET>, SAPI2UPS::<Impl, OFFSET>, UPS2SAPI::<Impl, OFFSET>, GetMaxConvertLength::<Impl, OFFSET>)
    }
}
pub trait ISpPhoneticAlphabetSelectionImpl: Sized {
    fn IsAlphabetUPS();
    fn SetAlphabetToUPS();
}
impl ::windows::core::RuntimeName for ISpPhoneticAlphabetSelection {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpPhoneticAlphabetSelection";
}
impl ISpPhoneticAlphabetSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetSelectionImpl, const OFFSET: isize>() -> ISpPhoneticAlphabetSelectionVtbl {
        unsafe extern "system" fn IsAlphabetUPS<Impl: ISpPhoneticAlphabetSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisups: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAlphabetUPS(::core::mem::transmute_copy(&pfisups)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphabetToUPS<Impl: ISpPhoneticAlphabetSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceups: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphabetToUPS(&*(&fforceups as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpPhoneticAlphabetSelection>, ::windows::core::GetTrustLevel, IsAlphabetUPS::<Impl, OFFSET>, SetAlphabetToUPS::<Impl, OFFSET>)
    }
}
pub trait ISpPhraseImpl: Sized {
    fn GetPhrase();
    fn GetSerializedPhrase();
    fn GetText();
    fn Discard();
}
impl ::windows::core::RuntimeName for ISpPhrase {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpPhrase";
}
impl ISpPhraseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhraseImpl, const OFFSET: isize>() -> ISpPhraseVtbl {
        unsafe extern "system" fn GetPhrase<Impl: ISpPhraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPPHRASE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhrase(::core::mem::transmute_copy(&ppcomemphrase)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedPhrase<Impl: ISpPhraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedPhrase(::core::mem::transmute_copy(&ppcomemphrase)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ISpPhraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut super::super::Foundation::PWSTR, pbdisplayattributes: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(ulstart, ulcount, &*(&fusetextreplacements as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszcomemtext), ::core::mem::transmute_copy(&pbdisplayattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discard<Impl: ISpPhraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwvaluetypes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Discard(dwvaluetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpPhrase>, ::windows::core::GetTrustLevel, GetPhrase::<Impl, OFFSET>, GetSerializedPhrase::<Impl, OFFSET>, GetText::<Impl, OFFSET>, Discard::<Impl, OFFSET>)
    }
}
pub trait ISpPhrase2Impl: Sized + ISpPhraseImpl {
    fn GetXMLResult();
    fn GetXMLErrorInfo();
    fn GetAudio();
}
impl ::windows::core::RuntimeName for ISpPhrase2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpPhrase2";
}
impl ISpPhrase2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase2Impl, const OFFSET: isize>() -> ISpPhrase2Vtbl {
        unsafe extern "system" fn GetXMLResult<Impl: ISpPhrase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut super::super::Foundation::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpPhrase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLErrorInfo(&*(&psemanticerrorinfo as *const <SPSEMANTICERRORINFO as ::windows::core::Abi>::Abi as *const <SPSEMANTICERRORINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudio<Impl: ISpPhrase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudio(ulstartelement, celements, ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpPhrase2>, ::windows::core::GetTrustLevel, GetXMLResult::<Impl, OFFSET>, GetXMLErrorInfo::<Impl, OFFSET>, GetAudio::<Impl, OFFSET>)
    }
}
pub trait ISpPhraseAltImpl: Sized + ISpPhraseImpl {
    fn GetAltInfo();
    fn Commit();
}
impl ::windows::core::RuntimeName for ISpPhraseAlt {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpPhraseAlt";
}
impl ISpPhraseAltVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhraseAltImpl, const OFFSET: isize>() -> ISpPhraseAltVtbl {
        unsafe extern "system" fn GetAltInfo<Impl: ISpPhraseAltImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAltInfo(::core::mem::transmute_copy(&ppparent), pulstartelementinparent, pcelementsinparent, pcelementsinalt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ISpPhraseAltImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpPhraseAlt>, ::windows::core::GetTrustLevel, GetAltInfo::<Impl, OFFSET>, Commit::<Impl, OFFSET>)
    }
}
pub trait ISpPropertiesImpl: Sized {
    fn SetPropertyNum();
    fn GetPropertyNum();
    fn SetPropertyString();
    fn GetPropertyString();
}
impl ::windows::core::RuntimeName for ISpProperties {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpProperties";
}
impl ISpPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPropertiesImpl, const OFFSET: isize>() -> ISpPropertiesVtbl {
        unsafe extern "system" fn SetPropertyNum<Impl: ISpPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyNum(&*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNum<Impl: ISpPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyNum(&*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), plvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyString<Impl: ISpPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyString(&*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyString<Impl: ISpPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, ppcomemvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyString(&*(&pname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomemvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpProperties>, ::windows::core::GetTrustLevel, SetPropertyNum::<Impl, OFFSET>, GetPropertyNum::<Impl, OFFSET>, SetPropertyString::<Impl, OFFSET>, GetPropertyString::<Impl, OFFSET>)
    }
}
pub trait ISpRecoContextImpl: Sized + ISpEventSourceImpl + ISpNotifySourceImpl {
    fn GetRecognizer();
    fn CreateGrammar();
    fn GetStatus();
    fn GetMaxAlternates();
    fn SetMaxAlternates();
    fn SetAudioOptions();
    fn GetAudioOptions();
    fn DeserializeResult();
    fn Bookmark();
    fn SetAdaptationData();
    fn Pause();
    fn Resume();
    fn SetVoice();
    fn GetVoice();
    fn SetVoicePurgeEvent();
    fn GetVoicePurgeEvent();
    fn SetContextState();
    fn GetContextState();
}
impl ::windows::core::RuntimeName for ISpRecoContext {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecoContext";
}
impl ISpRecoContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContextImpl, const OFFSET: isize>() -> ISpRecoContextVtbl {
        unsafe extern "system" fn GetRecognizer<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognizer(::core::mem::transmute_copy(&pprecognizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGrammar<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullgrammarid: u64, ppgrammar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGrammar(ullgrammarid, ::core::mem::transmute_copy(&ppgrammar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(&*(&pstatus as *const <SPRECOCONTEXTSTATUS as ::windows::core::Abi>::Abi as *const <SPRECOCONTEXTSTATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxAlternates<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalternates: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxAlternates(pcalternates) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxAlternates<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, calternates: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxAlternates(calternates) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioOptions<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPAUDIOOPTIONS, paudioformatid: &::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAudioOptions(options, &*(&paudioformatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pwaveformatex as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioOptions<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioOptions(poptions, &*(&paudioformatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppcomemwfex as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeserializeResult<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeserializeResult(&*(&pserializedresult as *const <SPSERIALIZEDRESULT as ::windows::core::Abi>::Abi as *const <SPSERIALIZEDRESULT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bookmark(options, ullstreamposition, &*(&lparamevent as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdaptationData<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAdaptationData(&*(&padaptationdata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause(dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume(dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoice<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoice: ::windows::core::RawPtr, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVoice(&*(&pvoice as *const <ISpVoice as ::windows::core::Abi>::Abi as *const <ISpVoice as ::windows::core::DefaultType>::DefaultType), &*(&fallowformatchanges as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoice<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoice(::core::mem::transmute_copy(&ppvoice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVoicePurgeEvent(ulleventinterest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoicePurgeEvent<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoicePurgeEvent(pulleventinterest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContextState<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContextState(econtextstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextState<Impl: ISpRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextState(pecontextstate) {
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
            ::windows::core::GetRuntimeClassName::<ISpRecoContext>,
            ::windows::core::GetTrustLevel,
            GetRecognizer::<Impl, OFFSET>,
            CreateGrammar::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetMaxAlternates::<Impl, OFFSET>,
            SetMaxAlternates::<Impl, OFFSET>,
            SetAudioOptions::<Impl, OFFSET>,
            GetAudioOptions::<Impl, OFFSET>,
            DeserializeResult::<Impl, OFFSET>,
            Bookmark::<Impl, OFFSET>,
            SetAdaptationData::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            SetVoice::<Impl, OFFSET>,
            GetVoice::<Impl, OFFSET>,
            SetVoicePurgeEvent::<Impl, OFFSET>,
            GetVoicePurgeEvent::<Impl, OFFSET>,
            SetContextState::<Impl, OFFSET>,
            GetContextState::<Impl, OFFSET>,
        )
    }
}
pub trait ISpRecoContext2Impl: Sized {
    fn SetGrammarOptions();
    fn GetGrammarOptions();
    fn SetAdaptationData2();
}
impl ::windows::core::RuntimeName for ISpRecoContext2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecoContext2";
}
impl ISpRecoContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext2Impl, const OFFSET: isize>() -> ISpRecoContext2Vtbl {
        unsafe extern "system" fn SetGrammarOptions<Impl: ISpRecoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammaroptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGrammarOptions(egrammaroptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrammarOptions<Impl: ISpRecoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammaroptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGrammarOptions(pegrammaroptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdaptationData2<Impl: ISpRecoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: super::super::Foundation::PWSTR, cch: u32, ptopicname: super::super::Foundation::PWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAdaptationData2(&*(&padaptationdata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, &*(&ptopicname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), eadaptationsettings, erelevance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpRecoContext2>, ::windows::core::GetTrustLevel, SetGrammarOptions::<Impl, OFFSET>, GetGrammarOptions::<Impl, OFFSET>, SetAdaptationData2::<Impl, OFFSET>)
    }
}
pub trait ISpRecoGrammarImpl: Sized + ISpGrammarBuilderImpl {
    fn GetGrammarId();
    fn GetRecoContext();
    fn LoadCmdFromFile();
    fn LoadCmdFromObject();
    fn LoadCmdFromResource();
    fn LoadCmdFromMemory();
    fn LoadCmdFromProprietaryGrammar();
    fn SetRuleState();
    fn SetRuleIdState();
    fn LoadDictation();
    fn UnloadDictation();
    fn SetDictationState();
    fn SetWordSequenceData();
    fn SetTextSelection();
    fn IsPronounceable();
    fn SetGrammarState();
    fn SaveCmd();
    fn GetGrammarState();
}
impl ::windows::core::RuntimeName for ISpRecoGrammar {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecoGrammar";
}
impl ISpRecoGrammarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammarImpl, const OFFSET: isize>() -> ISpRecoGrammarVtbl {
        unsafe extern "system" fn GetGrammarId<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullgrammarid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGrammarId(pullgrammarid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoContext<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecoctxt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecoContext(::core::mem::transmute_copy(&pprecoctxt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromFile<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCmdFromFile(&*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromObject<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcid: &::windows::core::GUID, pszgrammarname: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCmdFromObject(&*(&rcid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pszgrammarname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromResource<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HINSTANCE, pszresourcename: super::super::Foundation::PWSTR, pszresourcetype: super::super::Foundation::PWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCmdFromResource(
                &*(&hmodule as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType),
                &*(&pszresourcename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszresourcetype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                wlanguage,
                options,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromMemory<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCmdFromMemory(&*(&pgrammar as *const <SPBINARYGRAMMAR as ::windows::core::Abi>::Abi as *const <SPBINARYGRAMMAR as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromProprietaryGrammar<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidparam: &::windows::core::GUID, pszstringparam: super::super::Foundation::PWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCmdFromProprietaryGrammar(
                &*(&rguidparam as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszstringparam as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvdataprarm as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbdatasize,
                options,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleState<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRuleState(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&preserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), newstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleIdState<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulruleid: u32, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRuleIdState(ulruleid, newstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadDictation<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztopicname: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadDictation(&*(&psztopicname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadDictation<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnloadDictation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictationState<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDictationState(newstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWordSequenceData<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: super::super::Foundation::PWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWordSequenceData(&*(&ptext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchtext, &*(&pinfo as *const <SPTEXTSELECTIONINFO as ::windows::core::Abi>::Abi as *const <SPTEXTSELECTIONINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextSelection<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextSelection(&*(&pinfo as *const <SPTEXTSELECTIONINFO as ::windows::core::Abi>::Abi as *const <SPTEXTSELECTIONINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPronounceable<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPronounceable(&*(&pszword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pwordpronounceable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrammarState<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGrammarState(egrammarstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveCmd<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ppszcomemerrortext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveCmd(&*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszcomemerrortext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrammarState<Impl: ISpRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGrammarState(pegrammarstate) {
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
            ::windows::core::GetRuntimeClassName::<ISpRecoGrammar>,
            ::windows::core::GetTrustLevel,
            GetGrammarId::<Impl, OFFSET>,
            GetRecoContext::<Impl, OFFSET>,
            LoadCmdFromFile::<Impl, OFFSET>,
            LoadCmdFromObject::<Impl, OFFSET>,
            LoadCmdFromResource::<Impl, OFFSET>,
            LoadCmdFromMemory::<Impl, OFFSET>,
            LoadCmdFromProprietaryGrammar::<Impl, OFFSET>,
            SetRuleState::<Impl, OFFSET>,
            SetRuleIdState::<Impl, OFFSET>,
            LoadDictation::<Impl, OFFSET>,
            UnloadDictation::<Impl, OFFSET>,
            SetDictationState::<Impl, OFFSET>,
            SetWordSequenceData::<Impl, OFFSET>,
            SetTextSelection::<Impl, OFFSET>,
            IsPronounceable::<Impl, OFFSET>,
            SetGrammarState::<Impl, OFFSET>,
            SaveCmd::<Impl, OFFSET>,
            GetGrammarState::<Impl, OFFSET>,
        )
    }
}
pub trait ISpRecoGrammar2Impl: Sized {
    fn GetRules();
    fn LoadCmdFromFile2();
    fn LoadCmdFromMemory2();
    fn SetRulePriority();
    fn SetRuleWeight();
    fn SetDictationWeight();
    fn SetGrammarLoader();
    fn SetSMLSecurityManager();
}
impl ::windows::core::RuntimeName for ISpRecoGrammar2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecoGrammar2";
}
impl ISpRecoGrammar2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2Impl, const OFFSET: isize>() -> ISpRecoGrammar2Vtbl {
        unsafe extern "system" fn GetRules<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRules(&*(&ppcomemrules as *const <SPRULE as ::windows::core::Abi>::Abi as *const <SPRULE as ::windows::core::DefaultType>::DefaultType), punumrules) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromFile2<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, options: SPLOADOPTIONS, pszsharinguri: super::super::Foundation::PWSTR, pszbaseuri: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCmdFromFile2(
                &*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                options,
                &*(&pszsharinguri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszbaseuri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromMemory2<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: super::super::Foundation::PWSTR, pszbaseuri: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCmdFromMemory2(
                &*(&pgrammar as *const <SPBINARYGRAMMAR as ::windows::core::Abi>::Abi as *const <SPBINARYGRAMMAR as ::windows::core::DefaultType>::DefaultType),
                options,
                &*(&pszsharinguri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszbaseuri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRulePriority<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: super::super::Foundation::PWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRulePriority(&*(&pszrulename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ulruleid, nrulepriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleWeight<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: super::super::Foundation::PWSTR, ulruleid: u32, flweight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRuleWeight(&*(&pszrulename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ulruleid, flweight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictationWeight<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flweight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDictationWeight(flweight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrammarLoader<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGrammarLoader(&*(&ploader as *const <ISpeechResourceLoader as ::windows::core::Abi>::Abi as *const <ISpeechResourceLoader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMLSecurityManager<Impl: ISpRecoGrammar2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmlsecuritymanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSMLSecurityManager(&*(&psmlsecuritymanager as *const <super::super::System::Com::Urlmon::IInternetSecurityManager as ::windows::core::Abi>::Abi as *const <super::super::System::Com::Urlmon::IInternetSecurityManager as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISpRecoGrammar2>,
            ::windows::core::GetTrustLevel,
            GetRules::<Impl, OFFSET>,
            LoadCmdFromFile2::<Impl, OFFSET>,
            LoadCmdFromMemory2::<Impl, OFFSET>,
            SetRulePriority::<Impl, OFFSET>,
            SetRuleWeight::<Impl, OFFSET>,
            SetDictationWeight::<Impl, OFFSET>,
            SetGrammarLoader::<Impl, OFFSET>,
            SetSMLSecurityManager::<Impl, OFFSET>,
        )
    }
}
pub trait ISpRecoResultImpl: Sized + ISpPhraseImpl {
    fn GetResultTimes();
    fn GetAlternates();
    fn GetAudio();
    fn SpeakAudio();
    fn Serialize();
    fn ScaleAudio();
    fn GetRecoContext();
}
impl ::windows::core::RuntimeName for ISpRecoResult {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecoResult";
}
impl ISpRecoResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResultImpl, const OFFSET: isize>() -> ISpRecoResultVtbl {
        unsafe extern "system" fn GetResultTimes<Impl: ISpRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultTimes(&*(&ptimes as *const <SPRECORESULTTIMES as ::windows::core::Abi>::Abi as *const <SPRECORESULTTIMES as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternates<Impl: ISpRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut ::windows::core::RawPtr, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternates(ulstartelement, celements, ulrequestcount, ::core::mem::transmute_copy(&ppphrases), pcphrasesreturned) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudio<Impl: ISpRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudio(ulstartelement, celements, ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Impl: ISpRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakAudio(ulstartelement, celements, dwflags, pulstreamnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ISpRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&ppcomemserializedresult as *const <SPSERIALIZEDRESULT as ::windows::core::Abi>::Abi as *const <SPSERIALIZEDRESULT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleAudio<Impl: ISpRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformatid: &::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleAudio(&*(&paudioformatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pwaveformatex as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoContext<Impl: ISpRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecoContext(::core::mem::transmute_copy(&pprecocontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpRecoResult>, ::windows::core::GetTrustLevel, GetResultTimes::<Impl, OFFSET>, GetAlternates::<Impl, OFFSET>, GetAudio::<Impl, OFFSET>, SpeakAudio::<Impl, OFFSET>, Serialize::<Impl, OFFSET>, ScaleAudio::<Impl, OFFSET>, GetRecoContext::<Impl, OFFSET>)
    }
}
pub trait ISpRecoResult2Impl: Sized + ISpRecoResultImpl + ISpPhraseImpl {
    fn CommitAlternate();
    fn CommitText();
    fn SetTextFeedback();
}
impl ::windows::core::RuntimeName for ISpRecoResult2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecoResult2";
}
impl ISpRecoResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult2Impl, const OFFSET: isize>() -> ISpRecoResult2Vtbl {
        unsafe extern "system" fn CommitAlternate<Impl: ISpRecoResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrasealt: ::windows::core::RawPtr, ppnewresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAlternate(&*(&pphrasealt as *const <ISpPhraseAlt as ::windows::core::Abi>::Abi as *const <ISpPhraseAlt as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnewresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitText<Impl: ISpRecoResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, pszcorrecteddata: super::super::Foundation::PWSTR, ecommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitText(ulstartelement, celements, &*(&pszcorrecteddata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ecommitflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextFeedback<Impl: ISpRecoResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeedback: super::super::Foundation::PWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextFeedback(&*(&pszfeedback as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&fsuccessful as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpRecoResult2>, ::windows::core::GetTrustLevel, CommitAlternate::<Impl, OFFSET>, CommitText::<Impl, OFFSET>, SetTextFeedback::<Impl, OFFSET>)
    }
}
pub trait ISpRecognizerImpl: Sized + ISpPropertiesImpl {
    fn SetRecognizer();
    fn GetRecognizer();
    fn SetInput();
    fn GetInputObjectToken();
    fn GetInputStream();
    fn CreateRecoContext();
    fn GetRecoProfile();
    fn SetRecoProfile();
    fn IsSharedInstance();
    fn GetRecoState();
    fn SetRecoState();
    fn GetStatus();
    fn GetFormat();
    fn IsUISupported();
    fn DisplayUI();
    fn EmulateRecognition();
}
impl ::windows::core::RuntimeName for ISpRecognizer {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecognizer";
}
impl ISpRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizerImpl, const OFFSET: isize>() -> ISpRecognizerVtbl {
        unsafe extern "system" fn SetRecognizer<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecognizer(&*(&precognizer as *const <ISpObjectToken as ::windows::core::Abi>::Abi as *const <ISpObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecognizer<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognizer(::core::mem::transmute_copy(&pprecognizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInput<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkinput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInput(&*(&punkinput as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&fallowformatchanges as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputObjectToken<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputObjectToken(::core::mem::transmute_copy(&pptoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStream<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStream(::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecoContext<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewctxt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRecoContext(::core::mem::transmute_copy(&ppnewctxt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoProfile<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecoProfile(::core::mem::transmute_copy(&pptoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoProfile<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecoProfile(&*(&ptoken as *const <ISpObjectToken as ::windows::core::Abi>::Abi as *const <ISpObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSharedInstance<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSharedInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoState<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SPRECOSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecoState(pstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoState<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRECOSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecoState(newstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(&*(&pstatus as *const <SPRECOGNIZERSTATUS as ::windows::core::Abi>::Abi as *const <SPRECOGNIZERSTATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waveformattype: SPWAVEFORMATTYPE, pformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(waveformattype, &*(&pformatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomemwfex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(
                &*(&psztypeofui as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvextradata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbextradata,
                &*(&pfsupported as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUI(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&psztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psztypeofui as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvextradata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbextradata,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmulateRecognition<Impl: ISpRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmulateRecognition(&*(&pphrase as *const <ISpPhrase as ::windows::core::Abi>::Abi as *const <ISpPhrase as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISpRecognizer>,
            ::windows::core::GetTrustLevel,
            SetRecognizer::<Impl, OFFSET>,
            GetRecognizer::<Impl, OFFSET>,
            SetInput::<Impl, OFFSET>,
            GetInputObjectToken::<Impl, OFFSET>,
            GetInputStream::<Impl, OFFSET>,
            CreateRecoContext::<Impl, OFFSET>,
            GetRecoProfile::<Impl, OFFSET>,
            SetRecoProfile::<Impl, OFFSET>,
            IsSharedInstance::<Impl, OFFSET>,
            GetRecoState::<Impl, OFFSET>,
            SetRecoState::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetFormat::<Impl, OFFSET>,
            IsUISupported::<Impl, OFFSET>,
            DisplayUI::<Impl, OFFSET>,
            EmulateRecognition::<Impl, OFFSET>,
        )
    }
}
pub trait ISpRecognizer2Impl: Sized {
    fn EmulateRecognitionEx();
    fn SetTrainingState();
    fn ResetAcousticModelAdaptation();
}
impl ::windows::core::RuntimeName for ISpRecognizer2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRecognizer2";
}
impl ISpRecognizer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer2Impl, const OFFSET: isize>() -> ISpRecognizer2Vtbl {
        unsafe extern "system" fn EmulateRecognitionEx<Impl: ISpRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: ::windows::core::RawPtr, dwcompareflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmulateRecognitionEx(&*(&pphrase as *const <ISpPhrase as ::windows::core::Abi>::Abi as *const <ISpPhrase as ::windows::core::DefaultType>::DefaultType), dwcompareflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrainingState<Impl: ISpRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrainingState(&*(&fdoingtraining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&fadaptfromtrainingdata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Impl: ISpRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetAcousticModelAdaptation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpRecognizer2>, ::windows::core::GetTrustLevel, EmulateRecognitionEx::<Impl, OFFSET>, SetTrainingState::<Impl, OFFSET>, ResetAcousticModelAdaptation::<Impl, OFFSET>)
    }
}
pub trait ISpRegDataKeyImpl: Sized + ISpDataKeyImpl {
    fn SetKey();
}
impl ::windows::core::RuntimeName for ISpRegDataKey {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpRegDataKey";
}
impl ISpRegDataKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRegDataKeyImpl, const OFFSET: isize>() -> ISpRegDataKeyVtbl {
        unsafe extern "system" fn SetKey<Impl: ISpRegDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKey(&*(&hkey as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType), &*(&freadonly as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpRegDataKey>, ::windows::core::GetTrustLevel, SetKey::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpResourceManagerImpl: Sized + IServiceProviderImpl {
    fn SetObject();
    fn GetObject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpResourceManager {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpResourceManager";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpResourceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpResourceManagerImpl, const OFFSET: isize>() -> ISpResourceManagerVtbl {
        unsafe extern "system" fn SetObject<Impl: ISpResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: &::windows::core::GUID, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObject(&*(&guidserviceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punkobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: ISpResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: &::windows::core::GUID, objectclsid: &::windows::core::GUID, objectiid: &::windows::core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(
                &*(&guidserviceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&objectclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&objectiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&freleasewhenlastexternalrefreleased as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpResourceManager>, ::windows::core::GetTrustLevel, SetObject::<Impl, OFFSET>, GetObject::<Impl, OFFSET>)
    }
}
pub trait ISpSerializeStateImpl: Sized {
    fn GetSerializedState();
    fn SetSerializedState();
}
impl ::windows::core::RuntimeName for ISpSerializeState {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpSerializeState";
}
impl ISpSerializeStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpSerializeStateImpl, const OFFSET: isize>() -> ISpSerializeStateVtbl {
        unsafe extern "system" fn GetSerializedState<Impl: ISpSerializeStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedState(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pulsize), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSerializedState<Impl: ISpSerializeStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSerializedState(pbdata, ulsize, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpSerializeState>, ::windows::core::GetTrustLevel, GetSerializedState::<Impl, OFFSET>, SetSerializedState::<Impl, OFFSET>)
    }
}
pub trait ISpShortcutImpl: Sized {
    fn AddShortcut();
    fn RemoveShortcut();
    fn GetShortcuts();
    fn GetGeneration();
    fn GetWordsFromGenerationChange();
    fn GetWords();
    fn GetShortcutsForGeneration();
    fn GetGenerationChange();
}
impl ::windows::core::RuntimeName for ISpShortcut {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpShortcut";
}
impl ISpShortcutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcutImpl, const OFFSET: isize>() -> ISpShortcutVtbl {
        unsafe extern "system" fn AddShortcut<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: super::super::Foundation::PWSTR, langid: u16, pszspoken: super::super::Foundation::PWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddShortcut(&*(&pszdisplay as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), langid, &*(&pszspoken as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), shtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShortcut<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: super::super::Foundation::PWSTR, langid: u16, pszspoken: super::super::Foundation::PWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveShortcut(&*(&pszdisplay as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), langid, &*(&pszspoken as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), shtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShortcuts<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShortcuts(langid, &*(&pshortcutpairlist as *const <SPSHORTCUTPAIRLIST as ::windows::core::Abi>::Abi as *const <SPSHORTCUTPAIRLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeneration<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneration(::core::mem::transmute_copy(&pdwgeneration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWordsFromGenerationChange<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWordsFromGenerationChange(pdwgeneration, &*(&pwordlist as *const <SPWORDLIST as ::windows::core::Abi>::Abi as *const <SPWORDLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWords(pdwgeneration, pdwcookie, &*(&pwordlist as *const <SPWORDLIST as ::windows::core::Abi>::Abi as *const <SPWORDLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShortcutsForGeneration<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShortcutsForGeneration(pdwgeneration, pdwcookie, &*(&pshortcutpairlist as *const <SPSHORTCUTPAIRLIST as ::windows::core::Abi>::Abi as *const <SPSHORTCUTPAIRLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationChange<Impl: ISpShortcutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenerationChange(pdwgeneration, &*(&pshortcutpairlist as *const <SPSHORTCUTPAIRLIST as ::windows::core::Abi>::Abi as *const <SPSHORTCUTPAIRLIST as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISpShortcut>,
            ::windows::core::GetTrustLevel,
            AddShortcut::<Impl, OFFSET>,
            RemoveShortcut::<Impl, OFFSET>,
            GetShortcuts::<Impl, OFFSET>,
            GetGeneration::<Impl, OFFSET>,
            GetWordsFromGenerationChange::<Impl, OFFSET>,
            GetWords::<Impl, OFFSET>,
            GetShortcutsForGeneration::<Impl, OFFSET>,
            GetGenerationChange::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpStreamImpl: Sized + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn SetBaseStream();
    fn GetBaseStream();
    fn BindToFile();
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpStream {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpStream";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamImpl, const OFFSET: isize>() -> ISpStreamVtbl {
        unsafe extern "system" fn SetBaseStream<Impl: ISpStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, rguidformat: &::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBaseStream(
                &*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&rguidformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pwaveformatex as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBaseStream<Impl: ISpStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBaseStream(::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFile<Impl: ISpStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, emode: SPFILEMODE, pformatid: &::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindToFile(
                &*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                emode,
                &*(&pformatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pwaveformatex as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType),
                ulleventinterest,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ISpStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpStream>, ::windows::core::GetTrustLevel, SetBaseStream::<Impl, OFFSET>, GetBaseStream::<Impl, OFFSET>, BindToFile::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpStreamFormatImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetFormat();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpStreamFormat {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpStreamFormat";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpStreamFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatImpl, const OFFSET: isize>() -> ISpStreamFormatVtbl {
        unsafe extern "system" fn GetFormat<Impl: ISpStreamFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidformatid: &::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(&*(&pguidformatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomemwaveformatex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpStreamFormat>, ::windows::core::GetTrustLevel, GetFormat::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpStreamFormatConverterImpl: Sized + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn SetBaseStream();
    fn GetBaseStream();
    fn SetFormat();
    fn ResetSeekPosition();
    fn ScaleConvertedToBaseOffset();
    fn ScaleBaseToConvertedOffset();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpStreamFormatConverter {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpStreamFormatConverter";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpStreamFormatConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverterImpl, const OFFSET: isize>() -> ISpStreamFormatConverterVtbl {
        unsafe extern "system" fn SetBaseStream<Impl: ISpStreamFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBaseStream(
                &*(&pstream as *const <ISpStreamFormat as ::windows::core::Abi>::Abi as *const <ISpStreamFormat as ::windows::core::DefaultType>::DefaultType),
                &*(&fsetformattobasestreamformat as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fwritetobasestream as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBaseStream<Impl: ISpStreamFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBaseStream(::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ISpStreamFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidformatidofconvertedstream: &::windows::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormat(&*(&rguidformatidofconvertedstream as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pwaveformatexofconvertedstream as *const <super::Audio::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::Audio::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetSeekPosition<Impl: ISpStreamFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetSeekPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleConvertedToBaseOffset<Impl: ISpStreamFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleConvertedToBaseOffset(ulloffsetconvertedstream, ::core::mem::transmute_copy(&pulloffsetbasestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleBaseToConvertedOffset<Impl: ISpStreamFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleBaseToConvertedOffset(ulloffsetbasestream, ::core::mem::transmute_copy(&pulloffsetconvertedstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpStreamFormatConverter>, ::windows::core::GetTrustLevel, SetBaseStream::<Impl, OFFSET>, GetBaseStream::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, ResetSeekPosition::<Impl, OFFSET>, ScaleConvertedToBaseOffset::<Impl, OFFSET>, ScaleBaseToConvertedOffset::<Impl, OFFSET>)
    }
}
pub trait ISpTranscriptImpl: Sized {
    fn GetTranscript();
    fn AppendTranscript();
}
impl ::windows::core::RuntimeName for ISpTranscript {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpTranscript";
}
impl ISpTranscriptVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpTranscriptImpl, const OFFSET: isize>() -> ISpTranscriptVtbl {
        unsafe extern "system" fn GetTranscript<Impl: ISpTranscriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztranscript: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTranscript(::core::mem::transmute_copy(&ppsztranscript)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTranscript<Impl: ISpTranscriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztranscript: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendTranscript(&*(&psztranscript as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpTranscript>, ::windows::core::GetTrustLevel, GetTranscript::<Impl, OFFSET>, AppendTranscript::<Impl, OFFSET>)
    }
}
pub trait ISpVoiceImpl: Sized + ISpEventSourceImpl + ISpNotifySourceImpl {
    fn SetOutput();
    fn GetOutputObjectToken();
    fn GetOutputStream();
    fn Pause();
    fn Resume();
    fn SetVoice();
    fn GetVoice();
    fn Speak();
    fn SpeakStream();
    fn GetStatus();
    fn Skip();
    fn SetPriority();
    fn GetPriority();
    fn SetAlertBoundary();
    fn GetAlertBoundary();
    fn SetRate();
    fn GetRate();
    fn SetVolume();
    fn GetVolume();
    fn WaitUntilDone();
    fn SetSyncSpeakTimeout();
    fn GetSyncSpeakTimeout();
    fn SpeakCompleteEvent();
    fn IsUISupported();
    fn DisplayUI();
}
impl ::windows::core::RuntimeName for ISpVoice {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpVoice";
}
impl ISpVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoiceImpl, const OFFSET: isize>() -> ISpVoiceVtbl {
        unsafe extern "system" fn SetOutput<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkoutput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutput(&*(&punkoutput as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&fallowformatchanges as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputObjectToken<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjecttoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputObjectToken(::core::mem::transmute_copy(&ppobjecttoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStream<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStream(::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoice<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVoice(&*(&ptoken as *const <ISpObjectToken as ::windows::core::Abi>::Abi as *const <ISpObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoice<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoice(::core::mem::transmute_copy(&pptoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcs: super::super::Foundation::PWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Speak(&*(&pwcs as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pulstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakStream(&*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pulstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(&*(&pstatus as *const <SPVOICESTATUS as ::windows::core::Abi>::Abi as *const <SPVOICESTATUS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszlastbookmark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: super::super::Foundation::PWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(&*(&pitemtype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lnumitems, pulnumskipped) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epriority: SPVPRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(epriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPriority<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepriority: *mut SPVPRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPriority(pepriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertBoundary<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eboundary: SPEVENTENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlertBoundary(eboundary) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlertBoundary<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peboundary: *mut SPEVENTENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlertBoundary(peboundary) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRate<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateadjust: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRate(rateadjust) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRate<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prateadjust: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRate(prateadjust) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usvolume: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolume(usvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolume<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusvolume: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolume(pusvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitUntilDone<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitUntilDone(mstimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSpeakTimeout<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSyncSpeakTimeout(mstimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncSpeakTimeout<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmstimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncSpeakTimeout(pmstimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakCompleteEvent<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakCompleteEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(
                &*(&psztypeofui as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvextradata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbextradata,
                &*(&pfsupported as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUI(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&psztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psztypeofui as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvextradata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbextradata,
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
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISpVoice>,
            ::windows::core::GetTrustLevel,
            SetOutput::<Impl, OFFSET>,
            GetOutputObjectToken::<Impl, OFFSET>,
            GetOutputStream::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            SetVoice::<Impl, OFFSET>,
            GetVoice::<Impl, OFFSET>,
            Speak::<Impl, OFFSET>,
            SpeakStream::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            Skip::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            GetPriority::<Impl, OFFSET>,
            SetAlertBoundary::<Impl, OFFSET>,
            GetAlertBoundary::<Impl, OFFSET>,
            SetRate::<Impl, OFFSET>,
            GetRate::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            GetVolume::<Impl, OFFSET>,
            WaitUntilDone::<Impl, OFFSET>,
            SetSyncSpeakTimeout::<Impl, OFFSET>,
            GetSyncSpeakTimeout::<Impl, OFFSET>,
            SpeakCompleteEvent::<Impl, OFFSET>,
            IsUISupported::<Impl, OFFSET>,
            DisplayUI::<Impl, OFFSET>,
        )
    }
}
pub trait ISpXMLRecoResultImpl: Sized + ISpRecoResultImpl + ISpPhraseImpl {
    fn GetXMLResult();
    fn GetXMLErrorInfo();
}
impl ::windows::core::RuntimeName for ISpXMLRecoResult {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpXMLRecoResult";
}
impl ISpXMLRecoResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpXMLRecoResultImpl, const OFFSET: isize>() -> ISpXMLRecoResultVtbl {
        unsafe extern "system" fn GetXMLResult<Impl: ISpXMLRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut super::super::Foundation::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpXMLRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLErrorInfo(&*(&psemanticerrorinfo as *const <SPSEMANTICERRORINFO as ::windows::core::Abi>::Abi as *const <SPSEMANTICERRORINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpXMLRecoResult>, ::windows::core::GetTrustLevel, GetXMLResult::<Impl, OFFSET>, GetXMLErrorInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn Status();
    fn BufferInfo();
    fn DefaultFormat();
    fn Volume();
    fn SetVolume();
    fn BufferNotifySize();
    fn SetBufferNotifySize();
    fn EventHandle();
    fn SetState();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechAudio {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechAudio";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioImpl, const OFFSET: isize>() -> ISpeechAudioVtbl {
        unsafe extern "system" fn Status<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferInfo<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferInfo(::core::mem::transmute_copy(&bufferinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultFormat<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultFormat(::core::mem::transmute_copy(&streamformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume(::core::mem::transmute_copy(&volume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolume(volume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferNotifySize<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferNotifySize(::core::mem::transmute_copy(&buffernotifysize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferNotifySize(buffernotifysize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventHandle<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventHandle(::core::mem::transmute_copy(&eventhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechAudioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(state) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechAudio>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, OFFSET>,
            BufferInfo::<Impl, OFFSET>,
            DefaultFormat::<Impl, OFFSET>,
            Volume::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            BufferNotifySize::<Impl, OFFSET>,
            SetBufferNotifySize::<Impl, OFFSET>,
            EventHandle::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioBufferInfoImpl: Sized + IDispatchImpl {
    fn MinNotification();
    fn SetMinNotification();
    fn BufferSize();
    fn SetBufferSize();
    fn EventBias();
    fn SetEventBias();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechAudioBufferInfo {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechAudioBufferInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioBufferInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfoImpl, const OFFSET: isize>() -> ISpeechAudioBufferInfoVtbl {
        unsafe extern "system" fn MinNotification<Impl: ISpeechAudioBufferInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinNotification(::core::mem::transmute_copy(&minnotification)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinNotification<Impl: ISpeechAudioBufferInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMinNotification(minnotification) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferSize<Impl: ISpeechAudioBufferInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferSize(::core::mem::transmute_copy(&buffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Impl: ISpeechAudioBufferInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferSize(buffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventBias<Impl: ISpeechAudioBufferInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventBias(::core::mem::transmute_copy(&eventbias)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventBias<Impl: ISpeechAudioBufferInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventBias(eventbias) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechAudioBufferInfo>, ::windows::core::GetTrustLevel, MinNotification::<Impl, OFFSET>, SetMinNotification::<Impl, OFFSET>, BufferSize::<Impl, OFFSET>, SetBufferSize::<Impl, OFFSET>, EventBias::<Impl, OFFSET>, SetEventBias::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioFormatImpl: Sized + IDispatchImpl {
    fn Type();
    fn SetType();
    fn Guid();
    fn SetGuid();
    fn GetWaveFormatEx();
    fn SetWaveFormatEx();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechAudioFormat {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechAudioFormat";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormatImpl, const OFFSET: isize>() -> ISpeechAudioFormatVtbl {
        unsafe extern "system" fn Type<Impl: ISpeechAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut SpeechAudioFormatType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&audioformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ISpeechAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: SpeechAudioFormatType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetType(audioformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Impl: ISpeechAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Impl: ISpeechAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGuid(&*(&guid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWaveFormatEx<Impl: ISpeechAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWaveFormatEx(::core::mem::transmute_copy(&speechwaveformatex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaveFormatEx<Impl: ISpeechAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWaveFormatEx(&*(&speechwaveformatex as *const <ISpeechWaveFormatEx as ::windows::core::Abi>::Abi as *const <ISpeechWaveFormatEx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechAudioFormat>, ::windows::core::GetTrustLevel, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Guid::<Impl, OFFSET>, SetGuid::<Impl, OFFSET>, GetWaveFormatEx::<Impl, OFFSET>, SetWaveFormatEx::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioStatusImpl: Sized + IDispatchImpl {
    fn FreeBufferSpace();
    fn NonBlockingIO();
    fn State();
    fn CurrentSeekPosition();
    fn CurrentDevicePosition();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechAudioStatus {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechAudioStatus";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatusImpl, const OFFSET: isize>() -> ISpeechAudioStatusVtbl {
        unsafe extern "system" fn FreeBufferSpace<Impl: ISpeechAudioStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freebufferspace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeBufferSpace(::core::mem::transmute_copy(&freebufferspace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonBlockingIO<Impl: ISpeechAudioStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonblockingio: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonBlockingIO(::core::mem::transmute_copy(&nonblockingio)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISpeechAudioStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechAudioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSeekPosition<Impl: ISpeechAudioStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentseekposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSeekPosition(::core::mem::transmute_copy(&currentseekposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDevicePosition<Impl: ISpeechAudioStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdeviceposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDevicePosition(::core::mem::transmute_copy(&currentdeviceposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechAudioStatus>, ::windows::core::GetTrustLevel, FreeBufferSpace::<Impl, OFFSET>, NonBlockingIO::<Impl, OFFSET>, State::<Impl, OFFSET>, CurrentSeekPosition::<Impl, OFFSET>, CurrentDevicePosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechBaseStreamImpl: Sized + IDispatchImpl {
    fn Format();
    fn putref_Format();
    fn Read();
    fn Write();
    fn Seek();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechBaseStream {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechBaseStream";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechBaseStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStreamImpl, const OFFSET: isize>() -> ISpeechBaseStreamVtbl {
        unsafe extern "system" fn Format<Impl: ISpeechBaseStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format(::core::mem::transmute_copy(&audioformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Format<Impl: ISpeechBaseStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Format(&*(&audioformat as *const <ISpeechAudioFormat as ::windows::core::Abi>::Abi as *const <ISpeechAudioFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: ISpeechBaseStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&buffer), numberofbytes, ::core::mem::transmute_copy(&bytesread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: ISpeechBaseStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, byteswritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(&*(&buffer as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&byteswritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: ISpeechBaseStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, origin: SpeechStreamSeekPositionType, newposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(&*(&position as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), origin, ::core::mem::transmute_copy(&newposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechBaseStream>, ::windows::core::GetTrustLevel, Format::<Impl, OFFSET>, putref_Format::<Impl, OFFSET>, Read::<Impl, OFFSET>, Write::<Impl, OFFSET>, Seek::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechCustomStreamImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn BaseStream();
    fn putref_BaseStream();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechCustomStream {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechCustomStream";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechCustomStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCustomStreamImpl, const OFFSET: isize>() -> ISpeechCustomStreamVtbl {
        unsafe extern "system" fn BaseStream<Impl: ISpeechCustomStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStream(::core::mem::transmute_copy(&ppunkstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_BaseStream<Impl: ISpeechCustomStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_BaseStream(&*(&punkstream as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechCustomStream>, ::windows::core::GetTrustLevel, BaseStream::<Impl, OFFSET>, putref_BaseStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechDataKeyImpl: Sized + IDispatchImpl {
    fn SetBinaryValue();
    fn GetBinaryValue();
    fn SetStringValue();
    fn GetStringValue();
    fn SetLongValue();
    fn GetLongValue();
    fn OpenKey();
    fn CreateKey();
    fn DeleteKey();
    fn DeleteValue();
    fn EnumKeys();
    fn EnumValues();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechDataKey {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechDataKey";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechDataKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKeyImpl, const OFFSET: isize>() -> ISpeechDataKeyVtbl {
        unsafe extern "system" fn SetBinaryValue<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBinaryValue(&*(&valuename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBinaryValue<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBinaryValue(&*(&valuename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStringValue(&*(&valuename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(&*(&valuename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLongValue<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLongValue(&*(&valuename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongValue<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongValue(&*(&valuename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenKey<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenKey(&*(&subkeyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&subkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKey(&*(&subkeyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&subkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteKey(&*(&subkeyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteValue<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteValue(&*(&valuename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumKeys<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, subkeyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumKeys(index, ::core::mem::transmute_copy(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Impl: ISpeechDataKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, valuename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumValues(index, ::core::mem::transmute_copy(&valuename)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechDataKey>,
            ::windows::core::GetTrustLevel,
            SetBinaryValue::<Impl, OFFSET>,
            GetBinaryValue::<Impl, OFFSET>,
            SetStringValue::<Impl, OFFSET>,
            GetStringValue::<Impl, OFFSET>,
            SetLongValue::<Impl, OFFSET>,
            GetLongValue::<Impl, OFFSET>,
            OpenKey::<Impl, OFFSET>,
            CreateKey::<Impl, OFFSET>,
            DeleteKey::<Impl, OFFSET>,
            DeleteValue::<Impl, OFFSET>,
            EnumKeys::<Impl, OFFSET>,
            EnumValues::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechFileStreamImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn Open();
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechFileStream {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechFileStream";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechFileStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechFileStreamImpl, const OFFSET: isize>() -> ISpeechFileStreamVtbl {
        unsafe extern "system" fn Open<Impl: ISpeechFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemode: SpeechStreamFileMode, doevents: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&filename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), filemode, doevents) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ISpeechFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechFileStream>, ::windows::core::GetTrustLevel, Open::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleImpl: Sized + IDispatchImpl {
    fn Attributes();
    fn InitialState();
    fn Name();
    fn Id();
    fn Clear();
    fn AddResource();
    fn AddState();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechGrammarRule {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechGrammarRule";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>() -> ISpeechGrammarRuleVtbl {
        unsafe extern "system" fn Attributes<Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut SpeechRuleAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes(::core::mem::transmute_copy(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialState<Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialState(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddResource<Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcevalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddResource(&*(&resourcename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&resourcevalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddState<Impl: ISpeechGrammarRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddState(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechGrammarRule>, ::windows::core::GetTrustLevel, Attributes::<Impl, OFFSET>, InitialState::<Impl, OFFSET>, Name::<Impl, OFFSET>, Id::<Impl, OFFSET>, Clear::<Impl, OFFSET>, AddResource::<Impl, OFFSET>, AddState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleStateImpl: Sized + IDispatchImpl {
    fn Rule();
    fn Transitions();
    fn AddWordTransition();
    fn AddRuleTransition();
    fn AddSpecialTransition();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechGrammarRuleState {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechGrammarRuleState";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateImpl, const OFFSET: isize>() -> ISpeechGrammarRuleStateVtbl {
        unsafe extern "system" fn Rule<Impl: ISpeechGrammarRuleStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rule(::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transitions<Impl: ISpeechGrammarRuleStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transitions(::core::mem::transmute_copy(&transitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWordTransition<Impl: ISpeechGrammarRuleStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deststate: ::windows::core::RawPtr, words: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, separators: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: SpeechGrammarWordType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWordTransition(
                &*(&deststate as *const <ISpeechGrammarRuleState as ::windows::core::Abi>::Abi as *const <ISpeechGrammarRuleState as ::windows::core::DefaultType>::DefaultType),
                &*(&words as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&separators as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                r#type,
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                propertyid,
                &*(&propertyvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                weight,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRuleTransition<Impl: ISpeechGrammarRuleStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: ::windows::core::RawPtr, rule: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRuleTransition(
                &*(&destinationstate as *const <ISpeechGrammarRuleState as ::windows::core::Abi>::Abi as *const <ISpeechGrammarRuleState as ::windows::core::DefaultType>::DefaultType),
                &*(&rule as *const <ISpeechGrammarRule as ::windows::core::Abi>::Abi as *const <ISpeechGrammarRule as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                propertyid,
                &*(&propertyvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                weight,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSpecialTransition<Impl: ISpeechGrammarRuleStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: ::windows::core::RawPtr, r#type: SpeechSpecialTransitionType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSpecialTransition(
                &*(&destinationstate as *const <ISpeechGrammarRuleState as ::windows::core::Abi>::Abi as *const <ISpeechGrammarRuleState as ::windows::core::DefaultType>::DefaultType),
                r#type,
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                propertyid,
                &*(&propertyvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                weight,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechGrammarRuleState>, ::windows::core::GetTrustLevel, Rule::<Impl, OFFSET>, Transitions::<Impl, OFFSET>, AddWordTransition::<Impl, OFFSET>, AddRuleTransition::<Impl, OFFSET>, AddSpecialTransition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleStateTransitionImpl: Sized + IDispatchImpl {
    fn Type();
    fn Text();
    fn Rule();
    fn Weight();
    fn PropertyName();
    fn PropertyId();
    fn PropertyValue();
    fn NextState();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechGrammarRuleStateTransition {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechGrammarRuleStateTransition";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleStateTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransitionVtbl {
        unsafe extern "system" fn Type<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut SpeechGrammarRuleStateTransitionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rule(::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight(::core::mem::transmute_copy(&weight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyName<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyName(::core::mem::transmute_copy(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyId<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyId(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyValue(::core::mem::transmute_copy(&propertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextState<Impl: ISpeechGrammarRuleStateTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextState(::core::mem::transmute_copy(&nextstate)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechGrammarRuleStateTransition>,
            ::windows::core::GetTrustLevel,
            Type::<Impl, OFFSET>,
            Text::<Impl, OFFSET>,
            Rule::<Impl, OFFSET>,
            Weight::<Impl, OFFSET>,
            PropertyName::<Impl, OFFSET>,
            PropertyId::<Impl, OFFSET>,
            PropertyValue::<Impl, OFFSET>,
            NextState::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleStateTransitionsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechGrammarRuleStateTransitions {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechGrammarRuleStateTransitions";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleStateTransitionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransitionsImpl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransitionsVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechGrammarRuleStateTransitionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechGrammarRuleStateTransitionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechGrammarRuleStateTransitionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechGrammarRuleStateTransitions>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRulesImpl: Sized + IDispatchImpl {
    fn Count();
    fn FindRule();
    fn Item();
    fn _NewEnum();
    fn Dynamic();
    fn Add();
    fn Commit();
    fn CommitAndSave();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechGrammarRules {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechGrammarRules";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRulesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>() -> ISpeechGrammarRulesVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindRule<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulenameorid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindRule(&*(&rulenameorid as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dynamic<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dynamic(::core::mem::transmute_copy(&dynamic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&rulename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), attributes, ruleid, ::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAndSave<Impl: ISpeechGrammarRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortext: *mut super::super::Foundation::BSTR, savestream: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAndSave(::core::mem::transmute_copy(&errortext), ::core::mem::transmute_copy(&savestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechGrammarRules>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, FindRule::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Dynamic::<Impl, OFFSET>, Add::<Impl, OFFSET>, Commit::<Impl, OFFSET>, CommitAndSave::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconImpl: Sized + IDispatchImpl {
    fn GenerationId();
    fn GetWords();
    fn AddPronunciation();
    fn AddPronunciationByPhoneIds();
    fn RemovePronunciation();
    fn RemovePronunciationByPhoneIds();
    fn GetPronunciations();
    fn GetGenerationChange();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechLexicon {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechLexicon";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconImpl, const OFFSET: isize>() -> ISpeechLexiconVtbl {
        unsafe extern "system" fn GenerationId<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerationId(::core::mem::transmute_copy(&generationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWords(flags, ::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&words)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPronunciation<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPronunciation(&*(&bstrword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), langid, partofspeech, &*(&bstrpronunciation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPronunciationByPhoneIds<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPronunciationByPhoneIds(&*(&bstrword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), langid, partofspeech, &*(&phoneids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePronunciation<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePronunciation(&*(&bstrword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), langid, partofspeech, &*(&bstrpronunciation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePronunciationByPhoneIds<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePronunciationByPhoneIds(&*(&bstrword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), langid, partofspeech, &*(&phoneids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPronunciations<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPronunciations(&*(&bstrword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), langid, typeflags, ::core::mem::transmute_copy(&pppronunciations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationChange<Impl: ISpeechLexiconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32, ppwords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenerationChange(generationid, ::core::mem::transmute_copy(&ppwords)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechLexicon>,
            ::windows::core::GetTrustLevel,
            GenerationId::<Impl, OFFSET>,
            GetWords::<Impl, OFFSET>,
            AddPronunciation::<Impl, OFFSET>,
            AddPronunciationByPhoneIds::<Impl, OFFSET>,
            RemovePronunciation::<Impl, OFFSET>,
            RemovePronunciationByPhoneIds::<Impl, OFFSET>,
            GetPronunciations::<Impl, OFFSET>,
            GetGenerationChange::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconPronunciationImpl: Sized + IDispatchImpl {
    fn Type();
    fn LangId();
    fn PartOfSpeech();
    fn PhoneIds();
    fn Symbolic();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechLexiconPronunciation {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechLexiconPronunciation";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconPronunciationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciationImpl, const OFFSET: isize>() -> ISpeechLexiconPronunciationVtbl {
        unsafe extern "system" fn Type<Impl: ISpeechLexiconPronunciationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicontype: *mut SpeechLexiconType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&lexicontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LangId<Impl: ISpeechLexiconPronunciationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LangId(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartOfSpeech<Impl: ISpeechLexiconPronunciationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partofspeech: *mut SpeechPartOfSpeech) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartOfSpeech(::core::mem::transmute_copy(&partofspeech)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneIds<Impl: ISpeechLexiconPronunciationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneIds(::core::mem::transmute_copy(&phoneids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Symbolic<Impl: ISpeechLexiconPronunciationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbolic: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Symbolic(::core::mem::transmute_copy(&symbolic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechLexiconPronunciation>, ::windows::core::GetTrustLevel, Type::<Impl, OFFSET>, LangId::<Impl, OFFSET>, PartOfSpeech::<Impl, OFFSET>, PhoneIds::<Impl, OFFSET>, Symbolic::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconPronunciationsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechLexiconPronunciations {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechLexiconPronunciations";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconPronunciationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciationsImpl, const OFFSET: isize>() -> ISpeechLexiconPronunciationsVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechLexiconPronunciationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechLexiconPronunciationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pronunciation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pronunciation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechLexiconPronunciationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechLexiconPronunciations>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconWordImpl: Sized + IDispatchImpl {
    fn LangId();
    fn Type();
    fn Word();
    fn Pronunciations();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechLexiconWord {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechLexiconWord";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconWordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWordImpl, const OFFSET: isize>() -> ISpeechLexiconWordVtbl {
        unsafe extern "system" fn LangId<Impl: ISpeechLexiconWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LangId(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ISpeechLexiconWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordtype: *mut SpeechWordType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&wordtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Word<Impl: ISpeechLexiconWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Word(::core::mem::transmute_copy(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciations<Impl: ISpeechLexiconWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pronunciations(::core::mem::transmute_copy(&pronunciations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechLexiconWord>, ::windows::core::GetTrustLevel, LangId::<Impl, OFFSET>, Type::<Impl, OFFSET>, Word::<Impl, OFFSET>, Pronunciations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconWordsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechLexiconWords {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechLexiconWords";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconWordsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWordsImpl, const OFFSET: isize>() -> ISpeechLexiconWordsVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechLexiconWordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechLexiconWordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, word: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechLexiconWordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechLexiconWords>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechMMSysAudioImpl: Sized + ISpeechAudioImpl + ISpeechBaseStreamImpl + IDispatchImpl {
    fn DeviceId();
    fn SetDeviceId();
    fn LineId();
    fn SetLineId();
    fn MMHandle();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechMMSysAudio {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechMMSysAudio";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechMMSysAudioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudioImpl, const OFFSET: isize>() -> ISpeechMMSysAudioVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISpeechMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId(::core::mem::transmute_copy(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Impl: ISpeechMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeviceId(deviceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineId<Impl: ISpeechMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineId(::core::mem::transmute_copy(&lineid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineId<Impl: ISpeechMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineId(lineid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MMHandle<Impl: ISpeechMMSysAudioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MMHandle(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechMMSysAudio>, ::windows::core::GetTrustLevel, DeviceId::<Impl, OFFSET>, SetDeviceId::<Impl, OFFSET>, LineId::<Impl, OFFSET>, SetLineId::<Impl, OFFSET>, MMHandle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechMemoryStreamImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn SetData();
    fn GetData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechMemoryStream {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechMemoryStream";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechMemoryStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMemoryStreamImpl, const OFFSET: isize>() -> ISpeechMemoryStreamVtbl {
        unsafe extern "system" fn SetData<Impl: ISpeechMemoryStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&data as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: ISpeechMemoryStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechMemoryStream>, ::windows::core::GetTrustLevel, SetData::<Impl, OFFSET>, GetData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechObjectTokenImpl: Sized + IDispatchImpl {
    fn Id();
    fn DataKey();
    fn Category();
    fn GetDescription();
    fn SetId();
    fn GetAttribute();
    fn CreateInstance();
    fn Remove();
    fn GetStorageFileName();
    fn RemoveStorageFileName();
    fn IsUISupported();
    fn DisplayUI();
    fn MatchesAttributes();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechObjectToken {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechObjectToken";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectTokenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenImpl, const OFFSET: isize>() -> ISpeechObjectTokenVtbl {
        unsafe extern "system" fn Id<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataKey<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataKey(::core::mem::transmute_copy(&datakey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: i32, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(locale, ::core::mem::transmute_copy(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, categoryid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetId(&*(&id as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&categoryid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), createifnotexist) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributevalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttribute(&*(&attributename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&attributevalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), clscontext, ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&objectstorageclsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageFileName<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFileName(
                &*(&objectstorageclsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&keyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&filename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                folder,
                ::core::mem::transmute_copy(&filepath),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, deletefilea: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStorageFileName(&*(&objectstorageclsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&keyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), deletefilea) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(
                &*(&typeofui as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&extradata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&supported),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUI(
                hwnd,
                &*(&title as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&typeofui as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&extradata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesAttributes<Impl: ISpeechObjectTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, matches: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesAttributes(&*(&attributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&matches)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechObjectToken>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            DataKey::<Impl, OFFSET>,
            Category::<Impl, OFFSET>,
            GetDescription::<Impl, OFFSET>,
            SetId::<Impl, OFFSET>,
            GetAttribute::<Impl, OFFSET>,
            CreateInstance::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            GetStorageFileName::<Impl, OFFSET>,
            RemoveStorageFileName::<Impl, OFFSET>,
            IsUISupported::<Impl, OFFSET>,
            DisplayUI::<Impl, OFFSET>,
            MatchesAttributes::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechObjectTokenCategoryImpl: Sized + IDispatchImpl {
    fn Id();
    fn SetDefault();
    fn Default();
    fn SetId();
    fn GetDataKey();
    fn EnumerateTokens();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechObjectTokenCategory {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechObjectTokenCategory";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectTokenCategoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategoryImpl, const OFFSET: isize>() -> ISpeechObjectTokenCategoryVtbl {
        unsafe extern "system" fn Id<Impl: ISpeechObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Impl: ISpeechObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefault(&*(&tokenid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Default<Impl: ISpeechObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Default(::core::mem::transmute_copy(&tokenid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: ISpeechObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetId(&*(&id as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), createifnotexist) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataKey<Impl: ISpeechObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: SpeechDataKeyLocation, datakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataKey(location, ::core::mem::transmute_copy(&datakey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTokens<Impl: ISpeechObjectTokenCategoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTokens(&*(&requiredattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&optionalattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&tokens)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechObjectTokenCategory>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, SetDefault::<Impl, OFFSET>, Default::<Impl, OFFSET>, SetId::<Impl, OFFSET>, GetDataKey::<Impl, OFFSET>, EnumerateTokens::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechObjectTokensImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechObjectTokens {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechObjectTokens";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectTokensVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokensImpl, const OFFSET: isize>() -> ISpeechObjectTokensVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, token: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechObjectTokensImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechObjectTokens>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhoneConverterImpl: Sized + IDispatchImpl {
    fn LanguageId();
    fn SetLanguageId();
    fn PhoneToId();
    fn IdToPhone();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhoneConverter {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhoneConverter";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhoneConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhoneConverterImpl, const OFFSET: isize>() -> ISpeechPhoneConverterVtbl {
        unsafe extern "system" fn LanguageId<Impl: ISpeechPhoneConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageId(::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageId<Impl: ISpeechPhoneConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLanguageId(languageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneToId<Impl: ISpeechPhoneConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonemes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, idarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneToId(&*(&phonemes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&idarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Impl: ISpeechPhoneConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idarray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, phonemes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdToPhone(&*(&idarray as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phonemes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhoneConverter>, ::windows::core::GetTrustLevel, LanguageId::<Impl, OFFSET>, SetLanguageId::<Impl, OFFSET>, PhoneToId::<Impl, OFFSET>, IdToPhone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseAlternateImpl: Sized + IDispatchImpl {
    fn RecoResult();
    fn StartElementInResult();
    fn NumberOfElementsInResult();
    fn PhraseInfo();
    fn Commit();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseAlternate {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseAlternate";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseAlternateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternateImpl, const OFFSET: isize>() -> ISpeechPhraseAlternateVtbl {
        unsafe extern "system" fn RecoResult<Impl: ISpeechPhraseAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoResult(::core::mem::transmute_copy(&recoresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartElementInResult<Impl: ISpeechPhraseAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartElementInResult(::core::mem::transmute_copy(&startelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElementsInResult<Impl: ISpeechPhraseAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElementsInResult(::core::mem::transmute_copy(&numberofelements)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Impl: ISpeechPhraseAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhraseInfo(::core::mem::transmute_copy(&phraseinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ISpeechPhraseAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseAlternate>, ::windows::core::GetTrustLevel, RecoResult::<Impl, OFFSET>, StartElementInResult::<Impl, OFFSET>, NumberOfElementsInResult::<Impl, OFFSET>, PhraseInfo::<Impl, OFFSET>, Commit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseAlternatesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseAlternates {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseAlternates";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseAlternatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternatesImpl, const OFFSET: isize>() -> ISpeechPhraseAlternatesVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, phrasealternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&phrasealternate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseAlternates>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseElementImpl: Sized + IDispatchImpl {
    fn AudioTimeOffset();
    fn AudioSizeTime();
    fn AudioStreamOffset();
    fn AudioSizeBytes();
    fn RetainedStreamOffset();
    fn RetainedSizeBytes();
    fn DisplayText();
    fn LexicalForm();
    fn Pronunciation();
    fn DisplayAttributes();
    fn RequiredConfidence();
    fn ActualConfidence();
    fn EngineConfidence();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseElement {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseElement";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElementImpl, const OFFSET: isize>() -> ISpeechPhraseElementVtbl {
        unsafe extern "system" fn AudioTimeOffset<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiotimeoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioTimeOffset(::core::mem::transmute_copy(&audiotimeoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeTime(::core::mem::transmute_copy(&audiosizetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamOffset<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioStreamOffset(::core::mem::transmute_copy(&audiostreamoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeBytes(::core::mem::transmute_copy(&audiosizebytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedStreamOffset<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedstreamoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedStreamOffset(::core::mem::transmute_copy(&retainedstreamoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedSizeBytes(::core::mem::transmute_copy(&retainedsizebytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayText<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText(::core::mem::transmute_copy(&displaytext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LexicalForm<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicalform: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LexicalForm(::core::mem::transmute_copy(&lexicalform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciation<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciation: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pronunciation(::core::mem::transmute_copy(&pronunciation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAttributes<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAttributes(::core::mem::transmute_copy(&displayattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiredConfidence<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredConfidence(::core::mem::transmute_copy(&requiredconfidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualConfidence<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualConfidence(::core::mem::transmute_copy(&actualconfidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Impl: ISpeechPhraseElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineConfidence(::core::mem::transmute_copy(&engineconfidence)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechPhraseElement>,
            ::windows::core::GetTrustLevel,
            AudioTimeOffset::<Impl, OFFSET>,
            AudioSizeTime::<Impl, OFFSET>,
            AudioStreamOffset::<Impl, OFFSET>,
            AudioSizeBytes::<Impl, OFFSET>,
            RetainedStreamOffset::<Impl, OFFSET>,
            RetainedSizeBytes::<Impl, OFFSET>,
            DisplayText::<Impl, OFFSET>,
            LexicalForm::<Impl, OFFSET>,
            Pronunciation::<Impl, OFFSET>,
            DisplayAttributes::<Impl, OFFSET>,
            RequiredConfidence::<Impl, OFFSET>,
            ActualConfidence::<Impl, OFFSET>,
            EngineConfidence::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseElementsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseElements {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseElements";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElementsImpl, const OFFSET: isize>() -> ISpeechPhraseElementsVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseElements>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseInfoImpl: Sized + IDispatchImpl {
    fn LanguageId();
    fn GrammarId();
    fn StartTime();
    fn AudioStreamPosition();
    fn AudioSizeBytes();
    fn RetainedSizeBytes();
    fn AudioSizeTime();
    fn Rule();
    fn Properties();
    fn Elements();
    fn Replacements();
    fn EngineId();
    fn EnginePrivateData();
    fn SaveToMemory();
    fn GetText();
    fn GetDisplayAttributes();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseInfo {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>() -> ISpeechPhraseInfoVtbl {
        unsafe extern "system" fn LanguageId<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageId(::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrammarId<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrammarId(::core::mem::transmute_copy(&grammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime(::core::mem::transmute_copy(&starttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamPosition<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioStreamPosition(::core::mem::transmute_copy(&audiostreamposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiosizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeBytes(::core::mem::transmute_copy(&paudiosizebytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedSizeBytes(::core::mem::transmute_copy(&retainedsizebytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeTime(::core::mem::transmute_copy(&audiosizetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rule(::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Elements<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Elements(::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacements<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replacements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Replacements(::core::mem::transmute_copy(&replacements)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineId<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineidguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineId(::core::mem::transmute_copy(&engineidguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnginePrivateData<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privatedata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnginePrivateData(::core::mem::transmute_copy(&privatedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToMemory(::core::mem::transmute_copy(&phraseblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: i16, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(startelement, elements, usereplacements, ::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributes<Impl: ISpeechPhraseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: i16, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayAttributes(startelement, elements, usereplacements, ::core::mem::transmute_copy(&displayattributes)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechPhraseInfo>,
            ::windows::core::GetTrustLevel,
            LanguageId::<Impl, OFFSET>,
            GrammarId::<Impl, OFFSET>,
            StartTime::<Impl, OFFSET>,
            AudioStreamPosition::<Impl, OFFSET>,
            AudioSizeBytes::<Impl, OFFSET>,
            RetainedSizeBytes::<Impl, OFFSET>,
            AudioSizeTime::<Impl, OFFSET>,
            Rule::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Elements::<Impl, OFFSET>,
            Replacements::<Impl, OFFSET>,
            EngineId::<Impl, OFFSET>,
            EnginePrivateData::<Impl, OFFSET>,
            SaveToMemory::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            GetDisplayAttributes::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseInfoBuilderImpl: Sized + IDispatchImpl {
    fn RestorePhraseFromMemory();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseInfoBuilder {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseInfoBuilder";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseInfoBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfoBuilderImpl, const OFFSET: isize>() -> ISpeechPhraseInfoBuilderVtbl {
        unsafe extern "system" fn RestorePhraseFromMemory<Impl: ISpeechPhraseInfoBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinmemory: *const super::super::System::Com::VARIANT, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestorePhraseFromMemory(&*(&phraseinmemory as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phraseinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseInfoBuilder>, ::windows::core::GetTrustLevel, RestorePhraseFromMemory::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhrasePropertiesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseProperties {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseProperties";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhrasePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhrasePropertiesImpl, const OFFSET: isize>() -> ISpeechPhrasePropertiesVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhrasePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhrasePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhrasePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseProperties>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhrasePropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn Value();
    fn FirstElement();
    fn NumberOfElements();
    fn EngineConfidence();
    fn Confidence();
    fn Parent();
    fn Children();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseProperty {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseProperty";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhrasePropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>() -> ISpeechPhrasePropertyVtbl {
        unsafe extern "system" fn Name<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstElement(::core::mem::transmute_copy(&firstelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElements(::core::mem::transmute_copy(&numberofelements)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineConfidence(::core::mem::transmute_copy(&confidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confidence(::core::mem::transmute_copy(&confidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&parentproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: ISpeechPhrasePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children(::core::mem::transmute_copy(&children)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechPhraseProperty>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            Value::<Impl, OFFSET>,
            FirstElement::<Impl, OFFSET>,
            NumberOfElements::<Impl, OFFSET>,
            EngineConfidence::<Impl, OFFSET>,
            Confidence::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            Children::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseReplacementImpl: Sized + IDispatchImpl {
    fn DisplayAttributes();
    fn Text();
    fn FirstElement();
    fn NumberOfElements();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseReplacement {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseReplacement";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseReplacementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacementImpl, const OFFSET: isize>() -> ISpeechPhraseReplacementVtbl {
        unsafe extern "system" fn DisplayAttributes<Impl: ISpeechPhraseReplacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAttributes(::core::mem::transmute_copy(&displayattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ISpeechPhraseReplacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Impl: ISpeechPhraseReplacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstElement(::core::mem::transmute_copy(&firstelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Impl: ISpeechPhraseReplacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElements(::core::mem::transmute_copy(&numberofelements)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseReplacement>, ::windows::core::GetTrustLevel, DisplayAttributes::<Impl, OFFSET>, Text::<Impl, OFFSET>, FirstElement::<Impl, OFFSET>, NumberOfElements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseReplacementsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseReplacements {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseReplacements";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseReplacementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacementsImpl, const OFFSET: isize>() -> ISpeechPhraseReplacementsVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseReplacementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseReplacementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, reps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&reps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseReplacementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseReplacements>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseRuleImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn FirstElement();
    fn NumberOfElements();
    fn Parent();
    fn Children();
    fn Confidence();
    fn EngineConfidence();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseRule {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseRule";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>() -> ISpeechPhraseRuleVtbl {
        unsafe extern "system" fn Name<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstElement(::core::mem::transmute_copy(&firstelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElements(::core::mem::transmute_copy(&numberofelements)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&parent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children(::core::mem::transmute_copy(&children)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confidence(::core::mem::transmute_copy(&actualconfidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Impl: ISpeechPhraseRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineConfidence(::core::mem::transmute_copy(&engineconfidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseRule>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Id::<Impl, OFFSET>, FirstElement::<Impl, OFFSET>, NumberOfElements::<Impl, OFFSET>, Parent::<Impl, OFFSET>, Children::<Impl, OFFSET>, Confidence::<Impl, OFFSET>, EngineConfidence::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseRulesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechPhraseRules {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechPhraseRules";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseRulesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRulesImpl, const OFFSET: isize>() -> ISpeechPhraseRulesVtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechPhraseRules>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoContextImpl: Sized + IDispatchImpl {
    fn Recognizer();
    fn AudioInputInterferenceStatus();
    fn RequestedUIType();
    fn putref_Voice();
    fn Voice();
    fn SetAllowVoiceFormatMatchingOnNextSet();
    fn AllowVoiceFormatMatchingOnNextSet();
    fn SetVoicePurgeEvent();
    fn VoicePurgeEvent();
    fn SetEventInterests();
    fn EventInterests();
    fn SetCmdMaxAlternates();
    fn CmdMaxAlternates();
    fn SetState();
    fn State();
    fn SetRetainedAudio();
    fn RetainedAudio();
    fn putref_RetainedAudioFormat();
    fn RetainedAudioFormat();
    fn Pause();
    fn Resume();
    fn CreateGrammar();
    fn CreateResultFromMemory();
    fn Bookmark();
    fn SetAdaptationData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecoContext {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecoContext";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContextImpl, const OFFSET: isize>() -> ISpeechRecoContextVtbl {
        unsafe extern "system" fn Recognizer<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognizer(::core::mem::transmute_copy(&recognizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioInputInterferenceStatus<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interference: *mut SpeechInterference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioInputInterferenceStatus(::core::mem::transmute_copy(&interference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUIType<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uitype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedUIType(::core::mem::transmute_copy(&uitype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Voice(&*(&voice as *const <ISpeechVoice as ::windows::core::Abi>::Abi as *const <ISpeechVoice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Voice<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Voice(::core::mem::transmute_copy(&voice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowVoiceFormatMatchingOnNextSet<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowVoiceFormatMatchingOnNextSet(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowVoiceFormatMatchingOnNextSet<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowVoiceFormatMatchingOnNextSet(::core::mem::transmute_copy(&pallow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVoicePurgeEvent(eventinterest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VoicePurgeEvent<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VoicePurgeEvent(::core::mem::transmute_copy(&eventinterest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventInterests(eventinterest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterests<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventInterests(::core::mem::transmute_copy(&eventinterest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCmdMaxAlternates<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCmdMaxAlternates(maxalternates) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdMaxAlternates<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdMaxAlternates(::core::mem::transmute_copy(&maxalternates)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecoContextState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecoContextState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetainedAudio<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SpeechRetainedAudioOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRetainedAudio(option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedAudio<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: *mut SpeechRetainedAudioOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedAudio(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RetainedAudioFormat<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_RetainedAudioFormat(&*(&format as *const <ISpeechAudioFormat as ::windows::core::Abi>::Abi as *const <ISpeechAudioFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedAudioFormat<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedAudioFormat(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGrammar<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, grammar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGrammar(&*(&grammarid as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&grammar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultFromMemory<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *const super::super::System::Com::VARIANT, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResultFromMemory(&*(&resultblock as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SpeechBookmarkOptions, streampos: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bookmarkid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bookmark(options, &*(&streampos as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&bookmarkid as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdaptationData<Impl: ISpeechRecoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adaptationstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAdaptationData(&*(&adaptationstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechRecoContext>,
            ::windows::core::GetTrustLevel,
            Recognizer::<Impl, OFFSET>,
            AudioInputInterferenceStatus::<Impl, OFFSET>,
            RequestedUIType::<Impl, OFFSET>,
            putref_Voice::<Impl, OFFSET>,
            Voice::<Impl, OFFSET>,
            SetAllowVoiceFormatMatchingOnNextSet::<Impl, OFFSET>,
            AllowVoiceFormatMatchingOnNextSet::<Impl, OFFSET>,
            SetVoicePurgeEvent::<Impl, OFFSET>,
            VoicePurgeEvent::<Impl, OFFSET>,
            SetEventInterests::<Impl, OFFSET>,
            EventInterests::<Impl, OFFSET>,
            SetCmdMaxAlternates::<Impl, OFFSET>,
            CmdMaxAlternates::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            SetRetainedAudio::<Impl, OFFSET>,
            RetainedAudio::<Impl, OFFSET>,
            putref_RetainedAudioFormat::<Impl, OFFSET>,
            RetainedAudioFormat::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            CreateGrammar::<Impl, OFFSET>,
            CreateResultFromMemory::<Impl, OFFSET>,
            Bookmark::<Impl, OFFSET>,
            SetAdaptationData::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoGrammarImpl: Sized + IDispatchImpl {
    fn Id();
    fn RecoContext();
    fn SetState();
    fn State();
    fn Rules();
    fn Reset();
    fn CmdLoadFromFile();
    fn CmdLoadFromObject();
    fn CmdLoadFromResource();
    fn CmdLoadFromMemory();
    fn CmdLoadFromProprietaryGrammar();
    fn CmdSetRuleState();
    fn CmdSetRuleIdState();
    fn DictationLoad();
    fn DictationUnload();
    fn DictationSetState();
    fn SetWordSequenceData();
    fn SetTextSelection();
    fn IsPronounceable();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecoGrammar {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecoGrammar";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoGrammarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>() -> ISpeechRecoGrammarVtbl {
        unsafe extern "system" fn Id<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecoContext<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoContext(::core::mem::transmute_copy(&recocontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechGrammarState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechGrammarState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rules(::core::mem::transmute_copy(&rules)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset(newlanguage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdLoadFromFile<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdLoadFromFile(&*(&filename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loadoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdLoadFromObject<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grammarname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdLoadFromObject(&*(&classid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&grammarname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loadoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdLoadFromResource<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: i32, resourcename: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, resourcetype: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, languageid: i32, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdLoadFromResource(hmodule, &*(&resourcename as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&resourcetype as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), languageid, loadoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdLoadFromMemory<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdLoadFromMemory(&*(&grammardata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), loadoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdLoadFromProprietaryGrammar<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proprietaryguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarydata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdLoadFromProprietaryGrammar(
                &*(&proprietaryguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&proprietarystring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&proprietarydata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                loadoption,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdSetRuleState<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdSetRuleState(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CmdSetRuleIdState<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleid: i32, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdSetRuleIdState(ruleid, state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DictationLoad<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topicname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DictationLoad(&*(&topicname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loadoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DictationUnload<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DictationUnload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DictationSetState<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DictationSetState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWordSequenceData<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, textlength: i32, info: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWordSequenceData(&*(&text as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), textlength, &*(&info as *const <ISpeechTextSelectionInformation as ::windows::core::Abi>::Abi as *const <ISpeechTextSelectionInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextSelection<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, info: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextSelection(&*(&info as *const <ISpeechTextSelectionInformation as ::windows::core::Abi>::Abi as *const <ISpeechTextSelectionInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPronounceable<Impl: ISpeechRecoGrammarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPronounceable(&*(&word as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&wordpronounceable)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechRecoGrammar>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            RecoContext::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            Rules::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            CmdLoadFromFile::<Impl, OFFSET>,
            CmdLoadFromObject::<Impl, OFFSET>,
            CmdLoadFromResource::<Impl, OFFSET>,
            CmdLoadFromMemory::<Impl, OFFSET>,
            CmdLoadFromProprietaryGrammar::<Impl, OFFSET>,
            CmdSetRuleState::<Impl, OFFSET>,
            CmdSetRuleIdState::<Impl, OFFSET>,
            DictationLoad::<Impl, OFFSET>,
            DictationUnload::<Impl, OFFSET>,
            DictationSetState::<Impl, OFFSET>,
            SetWordSequenceData::<Impl, OFFSET>,
            SetTextSelection::<Impl, OFFSET>,
            IsPronounceable::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResultImpl: Sized + IDispatchImpl {
    fn RecoContext();
    fn Times();
    fn putref_AudioFormat();
    fn AudioFormat();
    fn PhraseInfo();
    fn Alternates();
    fn Audio();
    fn SpeakAudio();
    fn SaveToMemory();
    fn DiscardResultInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecoResult {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecoResult";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultImpl, const OFFSET: isize>() -> ISpeechRecoResultVtbl {
        unsafe extern "system" fn RecoContext<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoContext(::core::mem::transmute_copy(&recocontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Times(::core::mem::transmute_copy(&times)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AudioFormat(&*(&format as *const <ISpeechAudioFormat as ::windows::core::Abi>::Abi as *const <ISpeechAudioFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormat<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormat(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhraseInfo(::core::mem::transmute_copy(&phraseinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alternates(requestcount, startelement, elements, ::core::mem::transmute_copy(&alternates)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Audio(startelement, elements, ::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakAudio(startelement, elements, flags, ::core::mem::transmute_copy(&streamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToMemory(::core::mem::transmute_copy(&resultblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Impl: ISpeechRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscardResultInfo(valuetypes) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechRecoResult>,
            ::windows::core::GetTrustLevel,
            RecoContext::<Impl, OFFSET>,
            Times::<Impl, OFFSET>,
            putref_AudioFormat::<Impl, OFFSET>,
            AudioFormat::<Impl, OFFSET>,
            PhraseInfo::<Impl, OFFSET>,
            Alternates::<Impl, OFFSET>,
            Audio::<Impl, OFFSET>,
            SpeakAudio::<Impl, OFFSET>,
            SaveToMemory::<Impl, OFFSET>,
            DiscardResultInfo::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResult2Impl: Sized + ISpeechRecoResultImpl + IDispatchImpl {
    fn SetTextFeedback();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecoResult2 {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecoResult2";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult2Impl, const OFFSET: isize>() -> ISpeechRecoResult2Vtbl {
        unsafe extern "system" fn SetTextFeedback<Impl: ISpeechRecoResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextFeedback(&*(&feedback as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), wassuccessful) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechRecoResult2>, ::windows::core::GetTrustLevel, SetTextFeedback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResultDispatchImpl: Sized + IDispatchImpl {
    fn RecoContext();
    fn Times();
    fn putref_AudioFormat();
    fn AudioFormat();
    fn PhraseInfo();
    fn Alternates();
    fn Audio();
    fn SpeakAudio();
    fn SaveToMemory();
    fn DiscardResultInfo();
    fn GetXMLResult();
    fn GetXMLErrorInfo();
    fn SetTextFeedback();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecoResultDispatch {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecoResultDispatch";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResultDispatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>() -> ISpeechRecoResultDispatchVtbl {
        unsafe extern "system" fn RecoContext<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoContext(::core::mem::transmute_copy(&recocontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Times(::core::mem::transmute_copy(&times)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AudioFormat(&*(&format as *const <ISpeechAudioFormat as ::windows::core::Abi>::Abi as *const <ISpeechAudioFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormat<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormat(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhraseInfo(::core::mem::transmute_copy(&phraseinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alternates(requestcount, startelement, elements, ::core::mem::transmute_copy(&alternates)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Audio(startelement, elements, ::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakAudio(startelement, elements, flags, ::core::mem::transmute_copy(&streamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToMemory(::core::mem::transmute_copy(&resultblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscardResultInfo(valuetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLResult<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLResult(options, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut ::windows::core::HRESULT, iserror: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextFeedback<Impl: ISpeechRecoResultDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextFeedback(&*(&feedback as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), wassuccessful) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechRecoResultDispatch>,
            ::windows::core::GetTrustLevel,
            RecoContext::<Impl, OFFSET>,
            Times::<Impl, OFFSET>,
            putref_AudioFormat::<Impl, OFFSET>,
            AudioFormat::<Impl, OFFSET>,
            PhraseInfo::<Impl, OFFSET>,
            Alternates::<Impl, OFFSET>,
            Audio::<Impl, OFFSET>,
            SpeakAudio::<Impl, OFFSET>,
            SaveToMemory::<Impl, OFFSET>,
            DiscardResultInfo::<Impl, OFFSET>,
            GetXMLResult::<Impl, OFFSET>,
            GetXMLErrorInfo::<Impl, OFFSET>,
            SetTextFeedback::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResultTimesImpl: Sized + IDispatchImpl {
    fn StreamTime();
    fn Length();
    fn TickCount();
    fn OffsetFromStart();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecoResultTimes {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecoResultTimes";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResultTimesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultTimesImpl, const OFFSET: isize>() -> ISpeechRecoResultTimesVtbl {
        unsafe extern "system" fn StreamTime<Impl: ISpeechRecoResultTimesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamTime(::core::mem::transmute_copy(&time)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: ISpeechRecoResultTimesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ISpeechRecoResultTimesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount(::core::mem::transmute_copy(&tickcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromStart<Impl: ISpeechRecoResultTimesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetFromStart(::core::mem::transmute_copy(&offsetfromstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechRecoResultTimes>, ::windows::core::GetTrustLevel, StreamTime::<Impl, OFFSET>, Length::<Impl, OFFSET>, TickCount::<Impl, OFFSET>, OffsetFromStart::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecognizerImpl: Sized + IDispatchImpl {
    fn putref_Recognizer();
    fn Recognizer();
    fn SetAllowAudioInputFormatChangesOnNextSet();
    fn AllowAudioInputFormatChangesOnNextSet();
    fn putref_AudioInput();
    fn AudioInput();
    fn putref_AudioInputStream();
    fn AudioInputStream();
    fn IsShared();
    fn SetState();
    fn State();
    fn Status();
    fn putref_Profile();
    fn Profile();
    fn EmulateRecognition();
    fn CreateRecoContext();
    fn GetFormat();
    fn SetPropertyNumber();
    fn GetPropertyNumber();
    fn SetPropertyString();
    fn GetPropertyString();
    fn IsUISupported();
    fn DisplayUI();
    fn GetRecognizers();
    fn GetAudioInputs();
    fn GetProfiles();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecognizer {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecognizer";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerImpl, const OFFSET: isize>() -> ISpeechRecognizerVtbl {
        unsafe extern "system" fn putref_Recognizer<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Recognizer(&*(&recognizer as *const <ISpeechObjectToken as ::windows::core::Abi>::Abi as *const <ISpeechObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognizer<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognizer(::core::mem::transmute_copy(&recognizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAudioInputFormatChangesOnNextSet<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowAudioInputFormatChangesOnNextSet(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowAudioInputFormatChangesOnNextSet<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowAudioInputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInput<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AudioInput(&*(&audioinput as *const <ISpeechObjectToken as ::windows::core::Abi>::Abi as *const <ISpeechObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioInput<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioInput(::core::mem::transmute_copy(&audioinput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInputStream<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AudioInputStream(&*(&audioinputstream as *const <ISpeechBaseStream as ::windows::core::Abi>::Abi as *const <ISpeechBaseStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioInputStream<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioInputStream(::core::mem::transmute_copy(&audioinputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShared<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shared: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShared(::core::mem::transmute_copy(&shared)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Profile<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Profile(&*(&profile as *const <ISpeechObjectToken as ::windows::core::Abi>::Abi as *const <ISpeechObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&profile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmulateRecognition<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textelements: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmulateRecognition(&*(&textelements as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&elementdisplayattributes as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), languageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecoContext<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRecoContext(::core::mem::transmute_copy(&newcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SpeechFormatType, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(r#type, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyNumber<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyNumber(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), value, ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNumber<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyNumber(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), value, ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyString<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyString(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyString<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyString(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(&*(&typeofui as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&extradata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUI(
                hwndparent,
                &*(&title as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&typeofui as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&extradata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecognizers<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognizers(&*(&requiredattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&optionalattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objecttokens)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioInputs<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioInputs(&*(&requiredattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&optionalattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objecttokens)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfiles<Impl: ISpeechRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfiles(&*(&requiredattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&optionalattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objecttokens)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechRecognizer>,
            ::windows::core::GetTrustLevel,
            putref_Recognizer::<Impl, OFFSET>,
            Recognizer::<Impl, OFFSET>,
            SetAllowAudioInputFormatChangesOnNextSet::<Impl, OFFSET>,
            AllowAudioInputFormatChangesOnNextSet::<Impl, OFFSET>,
            putref_AudioInput::<Impl, OFFSET>,
            AudioInput::<Impl, OFFSET>,
            putref_AudioInputStream::<Impl, OFFSET>,
            AudioInputStream::<Impl, OFFSET>,
            IsShared::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            putref_Profile::<Impl, OFFSET>,
            Profile::<Impl, OFFSET>,
            EmulateRecognition::<Impl, OFFSET>,
            CreateRecoContext::<Impl, OFFSET>,
            GetFormat::<Impl, OFFSET>,
            SetPropertyNumber::<Impl, OFFSET>,
            GetPropertyNumber::<Impl, OFFSET>,
            SetPropertyString::<Impl, OFFSET>,
            GetPropertyString::<Impl, OFFSET>,
            IsUISupported::<Impl, OFFSET>,
            DisplayUI::<Impl, OFFSET>,
            GetRecognizers::<Impl, OFFSET>,
            GetAudioInputs::<Impl, OFFSET>,
            GetProfiles::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecognizerStatusImpl: Sized + IDispatchImpl {
    fn AudioStatus();
    fn CurrentStreamPosition();
    fn CurrentStreamNumber();
    fn NumberOfActiveRules();
    fn ClsidEngine();
    fn SupportedLanguages();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechRecognizerStatus {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechRecognizerStatus";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecognizerStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatusImpl, const OFFSET: isize>() -> ISpeechRecognizerStatusVtbl {
        unsafe extern "system" fn AudioStatus<Impl: ISpeechRecognizerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioStatus(::core::mem::transmute_copy(&audiostatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamPosition<Impl: ISpeechRecognizerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcurrentstreampos: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStreamPosition(::core::mem::transmute_copy(&pcurrentstreampos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamNumber<Impl: ISpeechRecognizerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStreamNumber(::core::mem::transmute_copy(&streamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfActiveRules<Impl: ISpeechRecognizerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofactiverules: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfActiveRules(::core::mem::transmute_copy(&numberofactiverules)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClsidEngine<Impl: ISpeechRecognizerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidengine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClsidEngine(::core::mem::transmute_copy(&clsidengine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedLanguages<Impl: ISpeechRecognizerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedlanguages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedLanguages(::core::mem::transmute_copy(&supportedlanguages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechRecognizerStatus>, ::windows::core::GetTrustLevel, AudioStatus::<Impl, OFFSET>, CurrentStreamPosition::<Impl, OFFSET>, CurrentStreamNumber::<Impl, OFFSET>, NumberOfActiveRules::<Impl, OFFSET>, ClsidEngine::<Impl, OFFSET>, SupportedLanguages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechResourceLoaderImpl: Sized + IDispatchImpl {
    fn LoadResource();
    fn GetLocalCopy();
    fn ReleaseLocalCopy();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechResourceLoader {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechResourceLoader";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechResourceLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechResourceLoaderImpl, const OFFSET: isize>() -> ISpeechResourceLoaderVtbl {
        unsafe extern "system" fn LoadResource<Impl: ISpeechResourceLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, falwaysreload: i16, pstream: *mut *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR, pfmodified: *mut i16, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadResource(&*(&bstrresourceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), falwaysreload, ::core::mem::transmute_copy(&pstream), ::core::mem::transmute_copy(&pbstrmimetype), pfmodified, ::core::mem::transmute_copy(&pbstrredirecturl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalCopy<Impl: ISpeechResourceLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlocalpath: *mut super::super::Foundation::BSTR, pbstrmimetype: *mut super::super::Foundation::BSTR, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalCopy(&*(&bstrresourceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrlocalpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pbstrredirecturl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseLocalCopy<Impl: ISpeechResourceLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlocalpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseLocalCopy(&*(&pbstrlocalpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechResourceLoader>, ::windows::core::GetTrustLevel, LoadResource::<Impl, OFFSET>, GetLocalCopy::<Impl, OFFSET>, ReleaseLocalCopy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechTextSelectionInformationImpl: Sized + IDispatchImpl {
    fn SetActiveOffset();
    fn ActiveOffset();
    fn SetActiveLength();
    fn ActiveLength();
    fn SetSelectionOffset();
    fn SelectionOffset();
    fn SetSelectionLength();
    fn SelectionLength();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechTextSelectionInformation {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechTextSelectionInformation";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechTextSelectionInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>() -> ISpeechTextSelectionInformationVtbl {
        unsafe extern "system" fn SetActiveOffset<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveOffset(activeoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveOffset<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveOffset(::core::mem::transmute_copy(&activeoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveLength<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveLength(activelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveLength<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveLength(::core::mem::transmute_copy(&activelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionOffset<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelectionOffset(selectionoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionOffset<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionOffset(::core::mem::transmute_copy(&selectionoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionLength<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelectionLength(selectionlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionLength<Impl: ISpeechTextSelectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionLength(::core::mem::transmute_copy(&selectionlength)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechTextSelectionInformation>,
            ::windows::core::GetTrustLevel,
            SetActiveOffset::<Impl, OFFSET>,
            ActiveOffset::<Impl, OFFSET>,
            SetActiveLength::<Impl, OFFSET>,
            ActiveLength::<Impl, OFFSET>,
            SetSelectionOffset::<Impl, OFFSET>,
            SelectionOffset::<Impl, OFFSET>,
            SetSelectionLength::<Impl, OFFSET>,
            SelectionLength::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechVoiceImpl: Sized + IDispatchImpl {
    fn Status();
    fn Voice();
    fn putref_Voice();
    fn AudioOutput();
    fn putref_AudioOutput();
    fn AudioOutputStream();
    fn putref_AudioOutputStream();
    fn Rate();
    fn SetRate();
    fn Volume();
    fn SetVolume();
    fn SetAllowAudioOutputFormatChangesOnNextSet();
    fn AllowAudioOutputFormatChangesOnNextSet();
    fn EventInterests();
    fn SetEventInterests();
    fn SetPriority();
    fn Priority();
    fn SetAlertBoundary();
    fn AlertBoundary();
    fn SetSynchronousSpeakTimeout();
    fn SynchronousSpeakTimeout();
    fn Speak();
    fn SpeakStream();
    fn Pause();
    fn Resume();
    fn Skip();
    fn GetVoices();
    fn GetAudioOutputs();
    fn WaitUntilDone();
    fn SpeakCompleteEvent();
    fn IsUISupported();
    fn DisplayUI();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechVoice {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechVoice";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceImpl, const OFFSET: isize>() -> ISpeechVoiceVtbl {
        unsafe extern "system" fn Status<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Voice<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Voice(::core::mem::transmute_copy(&voice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Voice(&*(&voice as *const <ISpeechObjectToken as ::windows::core::Abi>::Abi as *const <ISpeechObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioOutput<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioOutput(::core::mem::transmute_copy(&audiooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutput<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AudioOutput(&*(&audiooutput as *const <ISpeechObjectToken as ::windows::core::Abi>::Abi as *const <ISpeechObjectToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioOutputStream<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioOutputStream(::core::mem::transmute_copy(&audiooutputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutputStream<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AudioOutputStream(&*(&audiooutputstream as *const <ISpeechBaseStream as ::windows::core::Abi>::Abi as *const <ISpeechBaseStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rate<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rate(::core::mem::transmute_copy(&rate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRate<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRate(rate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume(::core::mem::transmute_copy(&volume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolume(volume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAudioOutputFormatChangesOnNextSet<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowAudioOutputFormatChangesOnNextSet(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowAudioOutputFormatChangesOnNextSet<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowAudioOutputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterests<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventInterests(::core::mem::transmute_copy(&eventinterestflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventInterests(eventinterestflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: SpeechVoicePriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: *mut SpeechVoicePriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&priority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertBoundary<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlertBoundary(boundary) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlertBoundary<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertBoundary(::core::mem::transmute_copy(&boundary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynchronousSpeakTimeout<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSynchronousSpeakTimeout(mstimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SynchronousSpeakTimeout<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SynchronousSpeakTimeout(::core::mem::transmute_copy(&mstimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Speak(&*(&text as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&streamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakStream(&*(&stream as *const <ISpeechBaseStream as ::windows::core::Abi>::Abi as *const <ISpeechBaseStream as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&streamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numitems: i32, numskipped: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(&*(&r#type as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), numitems, ::core::mem::transmute_copy(&numskipped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoices<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoices(&*(&requiredattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&optionalattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objecttokens)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioOutputs<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioOutputs(&*(&requiredattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&optionalattributes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objecttokens)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitUntilDone<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32, done: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitUntilDone(mstimeout, ::core::mem::transmute_copy(&done)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakCompleteEvent<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakCompleteEvent(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(&*(&typeofui as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&extradata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpeechVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUI(
                hwndparent,
                &*(&title as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&typeofui as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&extradata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
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
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISpeechVoice>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, OFFSET>,
            Voice::<Impl, OFFSET>,
            putref_Voice::<Impl, OFFSET>,
            AudioOutput::<Impl, OFFSET>,
            putref_AudioOutput::<Impl, OFFSET>,
            AudioOutputStream::<Impl, OFFSET>,
            putref_AudioOutputStream::<Impl, OFFSET>,
            Rate::<Impl, OFFSET>,
            SetRate::<Impl, OFFSET>,
            Volume::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            SetAllowAudioOutputFormatChangesOnNextSet::<Impl, OFFSET>,
            AllowAudioOutputFormatChangesOnNextSet::<Impl, OFFSET>,
            EventInterests::<Impl, OFFSET>,
            SetEventInterests::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            SetAlertBoundary::<Impl, OFFSET>,
            AlertBoundary::<Impl, OFFSET>,
            SetSynchronousSpeakTimeout::<Impl, OFFSET>,
            SynchronousSpeakTimeout::<Impl, OFFSET>,
            Speak::<Impl, OFFSET>,
            SpeakStream::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            Skip::<Impl, OFFSET>,
            GetVoices::<Impl, OFFSET>,
            GetAudioOutputs::<Impl, OFFSET>,
            WaitUntilDone::<Impl, OFFSET>,
            SpeakCompleteEvent::<Impl, OFFSET>,
            IsUISupported::<Impl, OFFSET>,
            DisplayUI::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechVoiceStatusImpl: Sized + IDispatchImpl {
    fn CurrentStreamNumber();
    fn LastStreamNumberQueued();
    fn LastHResult();
    fn RunningState();
    fn InputWordPosition();
    fn InputWordLength();
    fn InputSentencePosition();
    fn InputSentenceLength();
    fn LastBookmark();
    fn LastBookmarkId();
    fn PhonemeId();
    fn VisemeId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechVoiceStatus {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechVoiceStatus";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechVoiceStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>() -> ISpeechVoiceStatusVtbl {
        unsafe extern "system" fn CurrentStreamNumber<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStreamNumber(::core::mem::transmute_copy(&streamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastStreamNumberQueued<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastStreamNumberQueued(::core::mem::transmute_copy(&streamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastHResult<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastHResult(::core::mem::transmute_copy(&hresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningState<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRunState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningState(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordPosition<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputWordPosition(::core::mem::transmute_copy(&position)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordLength<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputWordLength(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentencePosition<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputSentencePosition(::core::mem::transmute_copy(&position)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentenceLength<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputSentenceLength(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmark<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBookmark(::core::mem::transmute_copy(&bookmark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmarkId<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmarkid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBookmarkId(::core::mem::transmute_copy(&bookmarkid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhonemeId<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhonemeId(::core::mem::transmute_copy(&phoneid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisemeId<Impl: ISpeechVoiceStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visemeid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisemeId(::core::mem::transmute_copy(&visemeid)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechVoiceStatus>,
            ::windows::core::GetTrustLevel,
            CurrentStreamNumber::<Impl, OFFSET>,
            LastStreamNumberQueued::<Impl, OFFSET>,
            LastHResult::<Impl, OFFSET>,
            RunningState::<Impl, OFFSET>,
            InputWordPosition::<Impl, OFFSET>,
            InputWordLength::<Impl, OFFSET>,
            InputSentencePosition::<Impl, OFFSET>,
            InputSentenceLength::<Impl, OFFSET>,
            LastBookmark::<Impl, OFFSET>,
            LastBookmarkId::<Impl, OFFSET>,
            PhonemeId::<Impl, OFFSET>,
            VisemeId::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechWaveFormatExImpl: Sized + IDispatchImpl {
    fn FormatTag();
    fn SetFormatTag();
    fn Channels();
    fn SetChannels();
    fn SamplesPerSec();
    fn SetSamplesPerSec();
    fn AvgBytesPerSec();
    fn SetAvgBytesPerSec();
    fn BlockAlign();
    fn SetBlockAlign();
    fn BitsPerSample();
    fn SetBitsPerSample();
    fn ExtraData();
    fn SetExtraData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechWaveFormatEx {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechWaveFormatEx";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechWaveFormatExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>() -> ISpeechWaveFormatExVtbl {
        unsafe extern "system" fn FormatTag<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatTag(::core::mem::transmute_copy(&formattag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormatTag(formattag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Channels<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channels(::core::mem::transmute_copy(&channels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannels(channels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SamplesPerSec<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamplesPerSec(::core::mem::transmute_copy(&samplespersec)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSamplesPerSec(samplespersec) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvgBytesPerSec<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvgBytesPerSec(::core::mem::transmute_copy(&avgbytespersec)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAvgBytesPerSec(avgbytespersec) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockAlign<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockAlign(::core::mem::transmute_copy(&blockalign)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlockAlign(blockalign) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitsPerSample<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerSample(::core::mem::transmute_copy(&bitspersample)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBitsPerSample(bitspersample) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtraData<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtraData(::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtraData<Impl: ISpeechWaveFormatExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtraData(&*(&extradata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISpeechWaveFormatEx>,
            ::windows::core::GetTrustLevel,
            FormatTag::<Impl, OFFSET>,
            SetFormatTag::<Impl, OFFSET>,
            Channels::<Impl, OFFSET>,
            SetChannels::<Impl, OFFSET>,
            SamplesPerSec::<Impl, OFFSET>,
            SetSamplesPerSec::<Impl, OFFSET>,
            AvgBytesPerSec::<Impl, OFFSET>,
            SetAvgBytesPerSec::<Impl, OFFSET>,
            BlockAlign::<Impl, OFFSET>,
            SetBlockAlign::<Impl, OFFSET>,
            BitsPerSample::<Impl, OFFSET>,
            SetBitsPerSample::<Impl, OFFSET>,
            ExtraData::<Impl, OFFSET>,
            SetExtraData::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechXMLRecoResultImpl: Sized + ISpeechRecoResultImpl + IDispatchImpl {
    fn GetXMLResult();
    fn GetXMLErrorInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISpeechXMLRecoResult {
    const NAME: &'static str = "Windows.Win32.Media.Speech.ISpeechXMLRecoResult";
}
#[cfg(feature = "Win32_System_Com")]
impl ISpeechXMLRecoResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechXMLRecoResultImpl, const OFFSET: isize>() -> ISpeechXMLRecoResultVtbl {
        unsafe extern "system" fn GetXMLResult<Impl: ISpeechXMLRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLResult(options, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpeechXMLRecoResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut i32, iserror: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechXMLRecoResult>, ::windows::core::GetTrustLevel, GetXMLResult::<Impl, OFFSET>, GetXMLErrorInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ISpeechRecoContextEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _ISpeechRecoContextEvents {
    const NAME: &'static str = "Windows.Win32.Media.Speech._ISpeechRecoContextEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _ISpeechRecoContextEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISpeechRecoContextEventsImpl, const OFFSET: isize>() -> _ISpeechRecoContextEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_ISpeechRecoContextEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ISpeechVoiceEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _ISpeechVoiceEvents {
    const NAME: &'static str = "Windows.Win32.Media.Speech._ISpeechVoiceEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _ISpeechVoiceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISpeechVoiceEventsImpl, const OFFSET: isize>() -> _ISpeechVoiceEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_ISpeechVoiceEvents>, ::windows::core::GetTrustLevel)
    }
}
