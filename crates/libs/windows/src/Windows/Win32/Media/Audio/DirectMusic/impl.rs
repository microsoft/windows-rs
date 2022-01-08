pub trait IDirectMusicImpl: Sized {
    fn EnumPort();
    fn CreateMusicBuffer();
    fn CreatePort();
    fn EnumMasterClock();
    fn GetMasterClock();
    fn SetMasterClock();
    fn Activate();
    fn GetDefaultPort();
    fn SetDirectSound();
}
impl ::windows::core::RuntimeName for IDirectMusic {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusic";
}
impl IDirectMusicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicImpl, const OFFSET: isize>() -> IDirectMusicVtbl {
        unsafe extern "system" fn EnumPort<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPort(dwindex, &*(&pportcaps as *const <DMUS_PORTCAPS as ::windows::core::Abi>::Abi as *const <DMUS_PORTCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMusicBuffer<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMusicBuffer(&*(&pbufferdesc as *const <DMUS_BUFFERDESC as ::windows::core::Abi>::Abi as *const <DMUS_BUFFERDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbuffer), &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePort<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsidport: &::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePort(
                &*(&rclsidport as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pportparams as *const <DMUS_PORTPARAMS8 as ::windows::core::Abi>::Abi as *const <DMUS_PORTPARAMS8 as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppport),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumMasterClock<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumMasterClock(dwindex, &*(&lpclockinfo as *const <DMUS_CLOCKINFO8 as ::windows::core::Abi>::Abi as *const <DMUS_CLOCKINFO8 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMasterClock<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMasterClock(&*(&pguidclock as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppreferenceclock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterClock<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidclock: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMasterClock(&*(&rguidclock as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&fenable as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultPort<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidport: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultPort(&*(&pguidport as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirectSound<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDirectSound(&*(&pdirectsound as *const <super::DirectSound::IDirectSound as ::windows::core::Abi>::Abi as *const <super::DirectSound::IDirectSound as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IDirectMusic>,
            ::windows::core::GetTrustLevel,
            EnumPort::<Impl, OFFSET>,
            CreateMusicBuffer::<Impl, OFFSET>,
            CreatePort::<Impl, OFFSET>,
            EnumMasterClock::<Impl, OFFSET>,
            GetMasterClock::<Impl, OFFSET>,
            SetMasterClock::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            GetDefaultPort::<Impl, OFFSET>,
            SetDirectSound::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectMusic8Impl: Sized + IDirectMusicImpl {
    fn SetExternalMasterClock();
}
impl ::windows::core::RuntimeName for IDirectMusic8 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusic8";
}
impl IDirectMusic8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic8Impl, const OFFSET: isize>() -> IDirectMusic8Vtbl {
        unsafe extern "system" fn SetExternalMasterClock<Impl: IDirectMusic8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExternalMasterClock(&*(&pclock as *const <super::super::IReferenceClock as ::windows::core::Abi>::Abi as *const <super::super::IReferenceClock as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusic8>, ::windows::core::GetTrustLevel, SetExternalMasterClock::<Impl, OFFSET>)
    }
}
pub trait IDirectMusicBufferImpl: Sized {
    fn Flush();
    fn TotalTime();
    fn PackStructured();
    fn PackUnstructured();
    fn ResetReadPtr();
    fn GetNextEvent();
    fn GetRawBufferPtr();
    fn GetStartTime();
    fn GetUsedBytes();
    fn GetMaxBytes();
    fn GetBufferFormat();
    fn SetStartTime();
    fn SetUsedBytes();
}
impl ::windows::core::RuntimeName for IDirectMusicBuffer {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicBuffer";
}
impl IDirectMusicBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBufferImpl, const OFFSET: isize>() -> IDirectMusicBufferVtbl {
        unsafe extern "system" fn Flush<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTime<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prttime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalTime(prttime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackStructured<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackStructured(rt, dwchannelgroup, dwchannelmessage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackUnstructured<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackUnstructured(rt, dwchannelgroup, cb, lpb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetReadPtr<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetReadPtr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextEvent<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextEvent(prt, pdwchannelgroup, pdwlength, ppdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRawBufferPtr<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRawBufferPtr(ppdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStartTime<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStartTime(prt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedBytes<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsedBytes(pcb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxBytes<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxBytes(pcb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferFormat<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferFormat(&*(&pguidformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStartTime(rt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUsedBytes<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUsedBytes(cb) {
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
            ::windows::core::GetRuntimeClassName::<IDirectMusicBuffer>,
            ::windows::core::GetTrustLevel,
            Flush::<Impl, OFFSET>,
            TotalTime::<Impl, OFFSET>,
            PackStructured::<Impl, OFFSET>,
            PackUnstructured::<Impl, OFFSET>,
            ResetReadPtr::<Impl, OFFSET>,
            GetNextEvent::<Impl, OFFSET>,
            GetRawBufferPtr::<Impl, OFFSET>,
            GetStartTime::<Impl, OFFSET>,
            GetUsedBytes::<Impl, OFFSET>,
            GetMaxBytes::<Impl, OFFSET>,
            GetBufferFormat::<Impl, OFFSET>,
            SetStartTime::<Impl, OFFSET>,
            SetUsedBytes::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectMusicCollectionImpl: Sized {
    fn GetInstrument();
    fn EnumInstrument();
}
impl ::windows::core::RuntimeName for IDirectMusicCollection {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicCollection";
}
impl IDirectMusicCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicCollectionImpl, const OFFSET: isize>() -> IDirectMusicCollectionVtbl {
        unsafe extern "system" fn GetInstrument<Impl: IDirectMusicCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpatch: u32, ppinstrument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstrument(dwpatch, ::core::mem::transmute_copy(&ppinstrument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumInstrument<Impl: IDirectMusicCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: super::super::super::Foundation::PWSTR, dwnamelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumInstrument(dwindex, pdwpatch, &*(&pwszname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwnamelen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusicCollection>, ::windows::core::GetTrustLevel, GetInstrument::<Impl, OFFSET>, EnumInstrument::<Impl, OFFSET>)
    }
}
pub trait IDirectMusicDownloadImpl: Sized {
    fn GetBuffer();
}
impl ::windows::core::RuntimeName for IDirectMusicDownload {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicDownload";
}
impl IDirectMusicDownloadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicDownloadImpl, const OFFSET: isize>() -> IDirectMusicDownloadVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IDirectMusicDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(&*(&ppvbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusicDownload>, ::windows::core::GetTrustLevel, GetBuffer::<Impl, OFFSET>)
    }
}
pub trait IDirectMusicDownloadedInstrumentImpl: Sized {}
impl ::windows::core::RuntimeName for IDirectMusicDownloadedInstrument {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicDownloadedInstrument";
}
impl IDirectMusicDownloadedInstrumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicDownloadedInstrumentImpl, const OFFSET: isize>() -> IDirectMusicDownloadedInstrumentVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusicDownloadedInstrument>, ::windows::core::GetTrustLevel)
    }
}
pub trait IDirectMusicInstrumentImpl: Sized {
    fn GetPatch();
    fn SetPatch();
}
impl ::windows::core::RuntimeName for IDirectMusicInstrument {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicInstrument";
}
impl IDirectMusicInstrumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicInstrumentImpl, const OFFSET: isize>() -> IDirectMusicInstrumentVtbl {
        unsafe extern "system" fn GetPatch<Impl: IDirectMusicInstrumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatch(pdwpatch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPatch<Impl: IDirectMusicInstrumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPatch(dwpatch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusicInstrument>, ::windows::core::GetTrustLevel, GetPatch::<Impl, OFFSET>, SetPatch::<Impl, OFFSET>)
    }
}
pub trait IDirectMusicPortImpl: Sized {
    fn PlayBuffer();
    fn SetReadNotificationHandle();
    fn Read();
    fn DownloadInstrument();
    fn UnloadInstrument();
    fn GetLatencyClock();
    fn GetRunningStats();
    fn Compact();
    fn GetCaps();
    fn DeviceIoControl();
    fn SetNumChannelGroups();
    fn GetNumChannelGroups();
    fn Activate();
    fn SetChannelPriority();
    fn GetChannelPriority();
    fn SetDirectSound();
    fn GetFormat();
}
impl ::windows::core::RuntimeName for IDirectMusicPort {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicPort";
}
impl IDirectMusicPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortImpl, const OFFSET: isize>() -> IDirectMusicPortVtbl {
        unsafe extern "system" fn PlayBuffer<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayBuffer(&*(&pbuffer as *const <IDirectMusicBuffer as ::windows::core::Abi>::Abi as *const <IDirectMusicBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadNotificationHandle<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReadNotificationHandle(&*(&hevent as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(&*(&pbuffer as *const <IDirectMusicBuffer as ::windows::core::Abi>::Abi as *const <IDirectMusicBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadInstrument<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstrument: ::windows::core::RawPtr, ppdownloadedinstrument: *mut ::windows::core::RawPtr, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadInstrument(&*(&pinstrument as *const <IDirectMusicInstrument as ::windows::core::Abi>::Abi as *const <IDirectMusicInstrument as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdownloadedinstrument), &*(&pnoteranges as *const <DMUS_NOTERANGE as ::windows::core::Abi>::Abi as *const <DMUS_NOTERANGE as ::windows::core::DefaultType>::DefaultType), dwnumnoteranges) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadInstrument<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdownloadedinstrument: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnloadInstrument(&*(&pdownloadedinstrument as *const <IDirectMusicDownloadedInstrument as ::windows::core::Abi>::Abi as *const <IDirectMusicDownloadedInstrument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatencyClock<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatencyClock(::core::mem::transmute_copy(&ppclock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningStats<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRunningStats(&*(&pstats as *const <DMUS_SYNTHSTATS as ::windows::core::Abi>::Abi as *const <DMUS_SYNTHSTATS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compact<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaps(&*(&pportcaps as *const <DMUS_PORTCAPS as ::windows::core::Abi>::Abi as *const <DMUS_PORTCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIoControl(
                dwiocontrolcode,
                &*(&lpinbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ninbuffersize,
                &*(&lpoutbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                noutbuffersize,
                lpbytesreturned,
                &*(&lpoverlapped as *const <super::super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumChannelGroups<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroups: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNumChannelGroups(dwchannelgroups) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumChannelGroups<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwchannelgroups: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumChannelGroups(pdwchannelgroups) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factive: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&factive as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelPriority<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelPriority(dwchannelgroup, dwchannel, dwpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelPriority<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelPriority(dwchannelgroup, dwchannel, pdwpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirectSound<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDirectSound(&*(&pdirectsound as *const <super::DirectSound::IDirectSound as ::windows::core::Abi>::Abi as *const <super::DirectSound::IDirectSound as ::windows::core::DefaultType>::DefaultType), &*(&pdirectsoundbuffer as *const <super::DirectSound::IDirectSoundBuffer as ::windows::core::Abi>::Abi as *const <super::DirectSound::IDirectSoundBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(&*(&pwaveformatex as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), pdwwaveformatexsize, pdwbuffersize) {
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
            ::windows::core::GetRuntimeClassName::<IDirectMusicPort>,
            ::windows::core::GetTrustLevel,
            PlayBuffer::<Impl, OFFSET>,
            SetReadNotificationHandle::<Impl, OFFSET>,
            Read::<Impl, OFFSET>,
            DownloadInstrument::<Impl, OFFSET>,
            UnloadInstrument::<Impl, OFFSET>,
            GetLatencyClock::<Impl, OFFSET>,
            GetRunningStats::<Impl, OFFSET>,
            Compact::<Impl, OFFSET>,
            GetCaps::<Impl, OFFSET>,
            DeviceIoControl::<Impl, OFFSET>,
            SetNumChannelGroups::<Impl, OFFSET>,
            GetNumChannelGroups::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            SetChannelPriority::<Impl, OFFSET>,
            GetChannelPriority::<Impl, OFFSET>,
            SetDirectSound::<Impl, OFFSET>,
            GetFormat::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectMusicPortDownloadImpl: Sized {
    fn GetBuffer();
    fn AllocateBuffer();
    fn GetDLId();
    fn GetAppend();
    fn Download();
    fn Unload();
}
impl ::windows::core::RuntimeName for IDirectMusicPortDownload {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicPortDownload";
}
impl IDirectMusicPortDownloadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>() -> IDirectMusicPortDownloadVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(dwdlid, ::core::mem::transmute_copy(&ppidmdownload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateBuffer<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateBuffer(dwsize, ::core::mem::transmute_copy(&ppidmdownload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDLId<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDLId(pdwstartdlid, dwcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppend<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppend(pdwappend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Download(&*(&pidmdownload as *const <IDirectMusicDownload as ::windows::core::Abi>::Abi as *const <IDirectMusicDownload as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unload<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unload(&*(&pidmdownload as *const <IDirectMusicDownload as ::windows::core::Abi>::Abi as *const <IDirectMusicDownload as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusicPortDownload>, ::windows::core::GetTrustLevel, GetBuffer::<Impl, OFFSET>, AllocateBuffer::<Impl, OFFSET>, GetDLId::<Impl, OFFSET>, GetAppend::<Impl, OFFSET>, Download::<Impl, OFFSET>, Unload::<Impl, OFFSET>)
    }
}
pub trait IDirectMusicSynthImpl: Sized {
    fn Open();
    fn Close();
    fn SetNumChannelGroups();
    fn Download();
    fn Unload();
    fn PlayBuffer();
    fn GetRunningStats();
    fn GetPortCaps();
    fn SetMasterClock();
    fn GetLatencyClock();
    fn Activate();
    fn SetSynthSink();
    fn Render();
    fn SetChannelPriority();
    fn GetChannelPriority();
    fn GetFormat();
    fn GetAppend();
}
impl ::windows::core::RuntimeName for IDirectMusicSynth {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicSynth";
}
impl IDirectMusicSynthVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthImpl, const OFFSET: isize>() -> IDirectMusicSynthVtbl {
        unsafe extern "system" fn Open<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pportparams as *const <DMUS_PORTPARAMS8 as ::windows::core::Abi>::Abi as *const <DMUS_PORTPARAMS8 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNumChannelGroups<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwgroups: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNumChannelGroups(dwgroups) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Download(&*(&phdownload as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&pvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pbfree) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unload<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unload(&*(&hdownload as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), lpfreehandle, &*(&huserdata as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayBuffer<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayBuffer(rt, pbbuffer, cbbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningStats<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRunningStats(&*(&pstats as *const <DMUS_SYNTHSTATS as ::windows::core::Abi>::Abi as *const <DMUS_SYNTHSTATS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortCaps<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPortCaps(&*(&pcaps as *const <DMUS_PORTCAPS as ::windows::core::Abi>::Abi as *const <DMUS_PORTCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterClock<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMasterClock(&*(&pclock as *const <super::super::IReferenceClock as ::windows::core::Abi>::Abi as *const <super::super::IReferenceClock as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatencyClock<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatencyClock(::core::mem::transmute_copy(&ppclock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&fenable as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynthSink<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynthsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSynthSink(&*(&psynthsink as *const <IDirectMusicSynthSink as ::windows::core::Abi>::Abi as *const <IDirectMusicSynthSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Render(pbuffer, dwlength, llposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelPriority<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelPriority(dwchannelgroup, dwchannel, dwpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelPriority<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelPriority(dwchannelgroup, dwchannel, pdwpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(&*(&pwaveformatex as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), pdwwaveformatexsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppend<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppend(pdwappend) {
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
            ::windows::core::GetRuntimeClassName::<IDirectMusicSynth>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            SetNumChannelGroups::<Impl, OFFSET>,
            Download::<Impl, OFFSET>,
            Unload::<Impl, OFFSET>,
            PlayBuffer::<Impl, OFFSET>,
            GetRunningStats::<Impl, OFFSET>,
            GetPortCaps::<Impl, OFFSET>,
            SetMasterClock::<Impl, OFFSET>,
            GetLatencyClock::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            SetSynthSink::<Impl, OFFSET>,
            Render::<Impl, OFFSET>,
            SetChannelPriority::<Impl, OFFSET>,
            GetChannelPriority::<Impl, OFFSET>,
            GetFormat::<Impl, OFFSET>,
            GetAppend::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectMusicSynth8Impl: Sized + IDirectMusicSynthImpl {
    fn PlayVoice();
    fn StopVoice();
    fn GetVoiceState();
    fn Refresh();
    fn AssignChannelToBuses();
}
impl ::windows::core::RuntimeName for IDirectMusicSynth8 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicSynth8";
}
impl IDirectMusicSynth8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8Impl, const OFFSET: isize>() -> IDirectMusicSynth8Vtbl {
        unsafe extern "system" fn PlayVoice<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayVoice(rt, dwvoiceid, dwchannelgroup, dwchannel, dwdlid, prpitch, vrvolume, stvoicestart, stloopstart, stloopend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopVoice<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopVoice(rt, dwvoiceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoiceState<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoiceState(dwvoice, cbvoice, &*(&dwvoicestate as *const <DMUS_VOICE_STATE as ::windows::core::Abi>::Abi as *const <DMUS_VOICE_STATE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh(dwdownloadid, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssignChannelToBuses<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssignChannelToBuses(dwchannelgroup, dwchannel, pdwbuses, cbuses) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusicSynth8>, ::windows::core::GetTrustLevel, PlayVoice::<Impl, OFFSET>, StopVoice::<Impl, OFFSET>, GetVoiceState::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, AssignChannelToBuses::<Impl, OFFSET>)
    }
}
pub trait IDirectMusicSynthSinkImpl: Sized {
    fn Init();
    fn SetMasterClock();
    fn GetLatencyClock();
    fn Activate();
    fn SampleToRefTime();
    fn RefTimeToSample();
    fn SetDirectSound();
    fn GetDesiredBufferSize();
}
impl ::windows::core::RuntimeName for IDirectMusicSynthSink {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicSynthSink";
}
impl IDirectMusicSynthSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>() -> IDirectMusicSynthSinkVtbl {
        unsafe extern "system" fn Init<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynth: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(&*(&psynth as *const <IDirectMusicSynth as ::windows::core::Abi>::Abi as *const <IDirectMusicSynth as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterClock<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMasterClock(&*(&pclock as *const <super::super::IReferenceClock as ::windows::core::Abi>::Abi as *const <super::super::IReferenceClock as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatencyClock<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatencyClock(::core::mem::transmute_copy(&ppclock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&fenable as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SampleToRefTime<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SampleToRefTime(llsampletime, prftime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefTimeToSample<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefTimeToSample(rftime, pllsampletime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirectSound<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDirectSound(&*(&pdirectsound as *const <super::DirectSound::IDirectSound as ::windows::core::Abi>::Abi as *const <super::DirectSound::IDirectSound as ::windows::core::DefaultType>::DefaultType), &*(&pdirectsoundbuffer as *const <super::DirectSound::IDirectSoundBuffer as ::windows::core::Abi>::Abi as *const <super::DirectSound::IDirectSoundBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesiredBufferSize<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesiredBufferSize(pdwbuffersizeinsamples) {
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
            ::windows::core::GetRuntimeClassName::<IDirectMusicSynthSink>,
            ::windows::core::GetTrustLevel,
            Init::<Impl, OFFSET>,
            SetMasterClock::<Impl, OFFSET>,
            GetLatencyClock::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            SampleToRefTime::<Impl, OFFSET>,
            RefTimeToSample::<Impl, OFFSET>,
            SetDirectSound::<Impl, OFFSET>,
            GetDesiredBufferSize::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectMusicThruImpl: Sized {
    fn ThruChannel();
}
impl ::windows::core::RuntimeName for IDirectMusicThru {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectMusic.IDirectMusicThru";
}
impl IDirectMusicThruVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicThruImpl, const OFFSET: isize>() -> IDirectMusicThruVtbl {
        unsafe extern "system" fn ThruChannel<Impl: IDirectMusicThruImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThruChannel(dwsourcechannelgroup, dwsourcechannel, dwdestinationchannelgroup, dwdestinationchannel, &*(&pdestinationport as *const <IDirectMusicPort as ::windows::core::Abi>::Abi as *const <IDirectMusicPort as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectMusicThru>, ::windows::core::GetTrustLevel, ThruChannel::<Impl, OFFSET>)
    }
}
