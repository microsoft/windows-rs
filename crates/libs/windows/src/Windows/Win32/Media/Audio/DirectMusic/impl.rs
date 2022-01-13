#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusicImpl: Sized {
    fn EnumPort(&mut self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()>;
    fn CreateMusicBuffer(&mut self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreatePort(&mut self, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumMasterClock(&mut self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::Result<()>;
    fn GetMasterClock(&mut self, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
    fn SetMasterClock(&mut self, rguidclock: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Activate(&mut self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDefaultPort(&mut self, pguidport: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetDirectSound(&mut self, pdirectsound: ::core::option::Option<super::DirectSound::IDirectSound>, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl IDirectMusicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicVtbl {
        unsafe extern "system" fn EnumPort<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumPort(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pportcaps)).into()
        }
        unsafe extern "system" fn CreateMusicBuffer<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateMusicBuffer(::core::mem::transmute_copy(&pbufferdesc), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute(&punkouter)).into()
        }
        unsafe extern "system" fn CreatePort<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePort(::core::mem::transmute_copy(&rclsidport), ::core::mem::transmute_copy(&pportparams), ::core::mem::transmute_copy(&ppport), ::core::mem::transmute(&punkouter)).into()
        }
        unsafe extern "system" fn EnumMasterClock<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumMasterClock(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&lpclockinfo)).into()
        }
        unsafe extern "system" fn GetMasterClock<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMasterClock(::core::mem::transmute_copy(&pguidclock), ::core::mem::transmute_copy(&ppreferenceclock)).into()
        }
        unsafe extern "system" fn SetMasterClock<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidclock: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMasterClock(::core::mem::transmute_copy(&rguidclock)).into()
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetDefaultPort<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidport: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultPort(::core::mem::transmute_copy(&pguidport)).into()
        }
        unsafe extern "system" fn SetDirectSound<Impl: IDirectMusicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirectSound(::core::mem::transmute(&pdirectsound), ::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumPort: EnumPort::<Impl, IMPL_OFFSET>,
            CreateMusicBuffer: CreateMusicBuffer::<Impl, IMPL_OFFSET>,
            CreatePort: CreatePort::<Impl, IMPL_OFFSET>,
            EnumMasterClock: EnumMasterClock::<Impl, IMPL_OFFSET>,
            GetMasterClock: GetMasterClock::<Impl, IMPL_OFFSET>,
            SetMasterClock: SetMasterClock::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            GetDefaultPort: GetDefaultPort::<Impl, IMPL_OFFSET>,
            SetDirectSound: SetDirectSound::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusic as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusic8Impl: Sized + IDirectMusicImpl {
    fn SetExternalMasterClock(&mut self, pclock: ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl IDirectMusic8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusic8Vtbl {
        unsafe extern "system" fn SetExternalMasterClock<Impl: IDirectMusic8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalMasterClock(::core::mem::transmute(&pclock)).into()
        }
        Self { base: IDirectMusicVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetExternalMasterClock: SetExternalMasterClock::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusic8 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicBufferImpl: Sized {
    fn Flush(&mut self) -> ::windows::core::Result<()>;
    fn TotalTime(&mut self, prttime: *mut i64) -> ::windows::core::Result<()>;
    fn PackStructured(&mut self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::Result<()>;
    fn PackUnstructured(&mut self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::Result<()>;
    fn ResetReadPtr(&mut self) -> ::windows::core::Result<()>;
    fn GetNextEvent(&mut self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetRawBufferPtr(&mut self, ppdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetStartTime(&mut self, prt: *mut i64) -> ::windows::core::Result<()>;
    fn GetUsedBytes(&mut self, pcb: *mut u32) -> ::windows::core::Result<()>;
    fn GetMaxBytes(&mut self, pcb: *mut u32) -> ::windows::core::Result<()>;
    fn GetBufferFormat(&mut self, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetStartTime(&mut self, rt: i64) -> ::windows::core::Result<()>;
    fn SetUsedBytes(&mut self, cb: u32) -> ::windows::core::Result<()>;
}
impl IDirectMusicBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicBufferVtbl {
        unsafe extern "system" fn Flush<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn TotalTime<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prttime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TotalTime(::core::mem::transmute_copy(&prttime)).into()
        }
        unsafe extern "system" fn PackStructured<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PackStructured(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannelmessage)).into()
        }
        unsafe extern "system" fn PackUnstructured<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PackUnstructured(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&lpb)).into()
        }
        unsafe extern "system" fn ResetReadPtr<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetReadPtr().into()
        }
        unsafe extern "system" fn GetNextEvent<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextEvent(::core::mem::transmute_copy(&prt), ::core::mem::transmute_copy(&pdwchannelgroup), ::core::mem::transmute_copy(&pdwlength), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetRawBufferPtr<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRawBufferPtr(::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetStartTime<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStartTime(::core::mem::transmute_copy(&prt)).into()
        }
        unsafe extern "system" fn GetUsedBytes<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUsedBytes(::core::mem::transmute_copy(&pcb)).into()
        }
        unsafe extern "system" fn GetMaxBytes<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMaxBytes(::core::mem::transmute_copy(&pcb)).into()
        }
        unsafe extern "system" fn GetBufferFormat<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferFormat(::core::mem::transmute_copy(&pguidformat)).into()
        }
        unsafe extern "system" fn SetStartTime<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&rt)).into()
        }
        unsafe extern "system" fn SetUsedBytes<Impl: IDirectMusicBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUsedBytes(::core::mem::transmute_copy(&cb)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Flush: Flush::<Impl, IMPL_OFFSET>,
            TotalTime: TotalTime::<Impl, IMPL_OFFSET>,
            PackStructured: PackStructured::<Impl, IMPL_OFFSET>,
            PackUnstructured: PackUnstructured::<Impl, IMPL_OFFSET>,
            ResetReadPtr: ResetReadPtr::<Impl, IMPL_OFFSET>,
            GetNextEvent: GetNextEvent::<Impl, IMPL_OFFSET>,
            GetRawBufferPtr: GetRawBufferPtr::<Impl, IMPL_OFFSET>,
            GetStartTime: GetStartTime::<Impl, IMPL_OFFSET>,
            GetUsedBytes: GetUsedBytes::<Impl, IMPL_OFFSET>,
            GetMaxBytes: GetMaxBytes::<Impl, IMPL_OFFSET>,
            GetBufferFormat: GetBufferFormat::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            SetUsedBytes: SetUsedBytes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectMusicCollectionImpl: Sized {
    fn GetInstrument(&mut self, dwpatch: u32) -> ::windows::core::Result<IDirectMusicInstrument>;
    fn EnumInstrument(&mut self, dwindex: u32, pdwpatch: *mut u32, pwszname: super::super::super::Foundation::PWSTR, dwnamelen: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectMusicCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicCollectionVtbl {
        unsafe extern "system" fn GetInstrument<Impl: IDirectMusicCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpatch: u32, ppinstrument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstrument(::core::mem::transmute_copy(&dwpatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinstrument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumInstrument<Impl: IDirectMusicCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: super::super::super::Foundation::PWSTR, dwnamelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumInstrument(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pdwpatch), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&dwnamelen)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInstrument: GetInstrument::<Impl, IMPL_OFFSET>,
            EnumInstrument: EnumInstrument::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicCollection as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicDownloadImpl: Sized {
    fn GetBuffer(&mut self, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IDirectMusicDownloadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicDownloadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicDownloadVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IDirectMusicDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&ppvbuffer), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetBuffer: GetBuffer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicDownload as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicDownloadedInstrumentImpl: Sized {}
impl IDirectMusicDownloadedInstrumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicDownloadedInstrumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicDownloadedInstrumentVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicDownloadedInstrument as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicInstrumentImpl: Sized {
    fn GetPatch(&mut self, pdwpatch: *mut u32) -> ::windows::core::Result<()>;
    fn SetPatch(&mut self, dwpatch: u32) -> ::windows::core::Result<()>;
}
impl IDirectMusicInstrumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicInstrumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicInstrumentVtbl {
        unsafe extern "system" fn GetPatch<Impl: IDirectMusicInstrumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPatch(::core::mem::transmute_copy(&pdwpatch)).into()
        }
        unsafe extern "system" fn SetPatch<Impl: IDirectMusicInstrumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPatch(::core::mem::transmute_copy(&dwpatch)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPatch: GetPatch::<Impl, IMPL_OFFSET>,
            SetPatch: SetPatch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicInstrument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
pub trait IDirectMusicPortImpl: Sized {
    fn PlayBuffer(&mut self, pbuffer: ::core::option::Option<IDirectMusicBuffer>) -> ::windows::core::Result<()>;
    fn SetReadNotificationHandle(&mut self, hevent: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn Read(&mut self, pbuffer: ::core::option::Option<IDirectMusicBuffer>) -> ::windows::core::Result<()>;
    fn DownloadInstrument(&mut self, pinstrument: ::core::option::Option<IDirectMusicInstrument>, ppdownloadedinstrument: *mut ::core::option::Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::Result<()>;
    fn UnloadInstrument(&mut self, pdownloadedinstrument: ::core::option::Option<IDirectMusicDownloadedInstrument>) -> ::windows::core::Result<()>;
    fn GetLatencyClock(&mut self) -> ::windows::core::Result<super::super::IReferenceClock>;
    fn GetRunningStats(&mut self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()>;
    fn Compact(&mut self) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()>;
    fn DeviceIoControl(&mut self, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn SetNumChannelGroups(&mut self, dwchannelgroups: u32) -> ::windows::core::Result<()>;
    fn GetNumChannelGroups(&mut self, pdwchannelgroups: *mut u32) -> ::windows::core::Result<()>;
    fn Activate(&mut self, factive: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetChannelPriority(&mut self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()>;
    fn GetChannelPriority(&mut self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()>;
    fn SetDirectSound(&mut self, pdirectsound: ::core::option::Option<super::DirectSound::IDirectSound>, pdirectsoundbuffer: ::core::option::Option<super::DirectSound::IDirectSoundBuffer>) -> ::windows::core::Result<()>;
    fn GetFormat(&mut self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl IDirectMusicPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicPortVtbl {
        unsafe extern "system" fn PlayBuffer<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn SetReadNotificationHandle<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadNotificationHandle(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Read<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn DownloadInstrument<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstrument: ::windows::core::RawPtr, ppdownloadedinstrument: *mut ::windows::core::RawPtr, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadInstrument(::core::mem::transmute(&pinstrument), ::core::mem::transmute_copy(&ppdownloadedinstrument), ::core::mem::transmute_copy(&pnoteranges), ::core::mem::transmute_copy(&dwnumnoteranges)).into()
        }
        unsafe extern "system" fn UnloadInstrument<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdownloadedinstrument: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnloadInstrument(::core::mem::transmute(&pdownloadedinstrument)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatencyClock() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningStats<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRunningStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn Compact<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&pportcaps)).into()
        }
        unsafe extern "system" fn DeviceIoControl<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceIoControl(::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&noutbuffersize), ::core::mem::transmute_copy(&lpbytesreturned), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn SetNumChannelGroups<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroups: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumChannelGroups(::core::mem::transmute_copy(&dwchannelgroups)).into()
        }
        unsafe extern "system" fn GetNumChannelGroups<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwchannelgroups: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumChannelGroups(::core::mem::transmute_copy(&pdwchannelgroups)).into()
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factive: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&factive)).into()
        }
        unsafe extern "system" fn SetChannelPriority<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetChannelPriority<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwpriority)).into()
        }
        unsafe extern "system" fn SetDirectSound<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirectSound(::core::mem::transmute(&pdirectsound), ::core::mem::transmute(&pdirectsoundbuffer)).into()
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectMusicPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&pdwwaveformatexsize), ::core::mem::transmute_copy(&pdwbuffersize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PlayBuffer: PlayBuffer::<Impl, IMPL_OFFSET>,
            SetReadNotificationHandle: SetReadNotificationHandle::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            DownloadInstrument: DownloadInstrument::<Impl, IMPL_OFFSET>,
            UnloadInstrument: UnloadInstrument::<Impl, IMPL_OFFSET>,
            GetLatencyClock: GetLatencyClock::<Impl, IMPL_OFFSET>,
            GetRunningStats: GetRunningStats::<Impl, IMPL_OFFSET>,
            Compact: Compact::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            DeviceIoControl: DeviceIoControl::<Impl, IMPL_OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Impl, IMPL_OFFSET>,
            GetNumChannelGroups: GetNumChannelGroups::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            SetChannelPriority: SetChannelPriority::<Impl, IMPL_OFFSET>,
            GetChannelPriority: GetChannelPriority::<Impl, IMPL_OFFSET>,
            SetDirectSound: SetDirectSound::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicPort as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicPortDownloadImpl: Sized {
    fn GetBuffer(&mut self, dwdlid: u32) -> ::windows::core::Result<IDirectMusicDownload>;
    fn AllocateBuffer(&mut self, dwsize: u32) -> ::windows::core::Result<IDirectMusicDownload>;
    fn GetDLId(&mut self, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::Result<()>;
    fn GetAppend(&mut self, pdwappend: *mut u32) -> ::windows::core::Result<()>;
    fn Download(&mut self, pidmdownload: ::core::option::Option<IDirectMusicDownload>) -> ::windows::core::Result<()>;
    fn Unload(&mut self, pidmdownload: ::core::option::Option<IDirectMusicDownload>) -> ::windows::core::Result<()>;
}
impl IDirectMusicPortDownloadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownloadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicPortDownloadVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&dwdlid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidmdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateBuffer<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateBuffer(::core::mem::transmute_copy(&dwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidmdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDLId<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDLId(::core::mem::transmute_copy(&pdwstartdlid), ::core::mem::transmute_copy(&dwcount)).into()
        }
        unsafe extern "system" fn GetAppend<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAppend(::core::mem::transmute_copy(&pdwappend)).into()
        }
        unsafe extern "system" fn Download<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Download(::core::mem::transmute(&pidmdownload)).into()
        }
        unsafe extern "system" fn Unload<Impl: IDirectMusicPortDownloadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unload(::core::mem::transmute(&pidmdownload)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            AllocateBuffer: AllocateBuffer::<Impl, IMPL_OFFSET>,
            GetDLId: GetDLId::<Impl, IMPL_OFFSET>,
            GetAppend: GetAppend::<Impl, IMPL_OFFSET>,
            Download: Download::<Impl, IMPL_OFFSET>,
            Unload: Unload::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicPortDownload as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectMusicSynthImpl: Sized {
    fn Open(&mut self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn SetNumChannelGroups(&mut self, dwgroups: u32) -> ::windows::core::Result<()>;
    fn Download(&mut self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::Result<()>;
    fn Unload(&mut self, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn PlayBuffer(&mut self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::Result<()>;
    fn GetRunningStats(&mut self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()>;
    fn GetPortCaps(&mut self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()>;
    fn SetMasterClock(&mut self, pclock: ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
    fn GetLatencyClock(&mut self) -> ::windows::core::Result<super::super::IReferenceClock>;
    fn Activate(&mut self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSynthSink(&mut self, psynthsink: ::core::option::Option<IDirectMusicSynthSink>) -> ::windows::core::Result<()>;
    fn Render(&mut self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::Result<()>;
    fn SetChannelPriority(&mut self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()>;
    fn GetChannelPriority(&mut self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()>;
    fn GetFormat(&mut self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetAppend(&mut self, pdwappend: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectMusicSynthVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicSynthVtbl {
        unsafe extern "system" fn Open<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pportparams)).into()
        }
        unsafe extern "system" fn Close<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetNumChannelGroups<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwgroups: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumChannelGroups(::core::mem::transmute_copy(&dwgroups)).into()
        }
        unsafe extern "system" fn Download<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Download(::core::mem::transmute_copy(&phdownload), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&pbfree)).into()
        }
        unsafe extern "system" fn Unload<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unload(::core::mem::transmute_copy(&hdownload), ::core::mem::transmute_copy(&lpfreehandle), ::core::mem::transmute_copy(&huserdata)).into()
        }
        unsafe extern "system" fn PlayBuffer<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayBuffer(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffer)).into()
        }
        unsafe extern "system" fn GetRunningStats<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRunningStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetPortCaps<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPortCaps(::core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn SetMasterClock<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMasterClock(::core::mem::transmute(&pclock)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatencyClock() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetSynthSink<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynthsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSynthSink(::core::mem::transmute(&psynthsink)).into()
        }
        unsafe extern "system" fn Render<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&llposition)).into()
        }
        unsafe extern "system" fn SetChannelPriority<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetChannelPriority<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwpriority)).into()
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&pdwwaveformatexsize)).into()
        }
        unsafe extern "system" fn GetAppend<Impl: IDirectMusicSynthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAppend(::core::mem::transmute_copy(&pdwappend)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Impl, IMPL_OFFSET>,
            Download: Download::<Impl, IMPL_OFFSET>,
            Unload: Unload::<Impl, IMPL_OFFSET>,
            PlayBuffer: PlayBuffer::<Impl, IMPL_OFFSET>,
            GetRunningStats: GetRunningStats::<Impl, IMPL_OFFSET>,
            GetPortCaps: GetPortCaps::<Impl, IMPL_OFFSET>,
            SetMasterClock: SetMasterClock::<Impl, IMPL_OFFSET>,
            GetLatencyClock: GetLatencyClock::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            SetSynthSink: SetSynthSink::<Impl, IMPL_OFFSET>,
            Render: Render::<Impl, IMPL_OFFSET>,
            SetChannelPriority: SetChannelPriority::<Impl, IMPL_OFFSET>,
            GetChannelPriority: GetChannelPriority::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
            GetAppend: GetAppend::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicSynth as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectMusicSynth8Impl: Sized + IDirectMusicSynthImpl {
    fn PlayVoice(&mut self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::Result<()>;
    fn StopVoice(&mut self, rt: i64, dwvoiceid: u32) -> ::windows::core::Result<()>;
    fn GetVoiceState(&mut self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::Result<()>;
    fn Refresh(&mut self, dwdownloadid: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn AssignChannelToBuses(&mut self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectMusicSynth8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicSynth8Vtbl {
        unsafe extern "system" fn PlayVoice<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayVoice(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwvoiceid), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwdlid), ::core::mem::transmute_copy(&prpitch), ::core::mem::transmute_copy(&vrvolume), ::core::mem::transmute_copy(&stvoicestart), ::core::mem::transmute_copy(&stloopstart), ::core::mem::transmute_copy(&stloopend)).into()
        }
        unsafe extern "system" fn StopVoice<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopVoice(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwvoiceid)).into()
        }
        unsafe extern "system" fn GetVoiceState<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVoiceState(::core::mem::transmute_copy(&dwvoice), ::core::mem::transmute_copy(&cbvoice), ::core::mem::transmute_copy(&dwvoicestate)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh(::core::mem::transmute_copy(&dwdownloadid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn AssignChannelToBuses<Impl: IDirectMusicSynth8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssignChannelToBuses(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwbuses), ::core::mem::transmute_copy(&cbuses)).into()
        }
        Self {
            base: IDirectMusicSynthVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PlayVoice: PlayVoice::<Impl, IMPL_OFFSET>,
            StopVoice: StopVoice::<Impl, IMPL_OFFSET>,
            GetVoiceState: GetVoiceState::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            AssignChannelToBuses: AssignChannelToBuses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicSynth8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusicSynthSinkImpl: Sized {
    fn Init(&mut self, psynth: ::core::option::Option<IDirectMusicSynth>) -> ::windows::core::Result<()>;
    fn SetMasterClock(&mut self, pclock: ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
    fn GetLatencyClock(&mut self) -> ::windows::core::Result<super::super::IReferenceClock>;
    fn Activate(&mut self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SampleToRefTime(&mut self, llsampletime: i64, prftime: *mut i64) -> ::windows::core::Result<()>;
    fn RefTimeToSample(&mut self, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::Result<()>;
    fn SetDirectSound(&mut self, pdirectsound: ::core::option::Option<super::DirectSound::IDirectSound>, pdirectsoundbuffer: ::core::option::Option<super::DirectSound::IDirectSoundBuffer>) -> ::windows::core::Result<()>;
    fn GetDesiredBufferSize(&mut self, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl IDirectMusicSynthSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicSynthSinkVtbl {
        unsafe extern "system" fn Init<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynth: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute(&psynth)).into()
        }
        unsafe extern "system" fn SetMasterClock<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMasterClock(::core::mem::transmute(&pclock)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatencyClock() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SampleToRefTime<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SampleToRefTime(::core::mem::transmute_copy(&llsampletime), ::core::mem::transmute_copy(&prftime)).into()
        }
        unsafe extern "system" fn RefTimeToSample<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefTimeToSample(::core::mem::transmute_copy(&rftime), ::core::mem::transmute_copy(&pllsampletime)).into()
        }
        unsafe extern "system" fn SetDirectSound<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirectSound(::core::mem::transmute(&pdirectsound), ::core::mem::transmute(&pdirectsoundbuffer)).into()
        }
        unsafe extern "system" fn GetDesiredBufferSize<Impl: IDirectMusicSynthSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesiredBufferSize(::core::mem::transmute_copy(&pdwbuffersizeinsamples)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            SetMasterClock: SetMasterClock::<Impl, IMPL_OFFSET>,
            GetLatencyClock: GetLatencyClock::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            SampleToRefTime: SampleToRefTime::<Impl, IMPL_OFFSET>,
            RefTimeToSample: RefTimeToSample::<Impl, IMPL_OFFSET>,
            SetDirectSound: SetDirectSound::<Impl, IMPL_OFFSET>,
            GetDesiredBufferSize: GetDesiredBufferSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicSynthSink as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicThruImpl: Sized {
    fn ThruChannel(&mut self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::core::option::Option<IDirectMusicPort>) -> ::windows::core::Result<()>;
}
impl IDirectMusicThruVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicThruImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectMusicThruVtbl {
        unsafe extern "system" fn ThruChannel<Impl: IDirectMusicThruImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ThruChannel(::core::mem::transmute_copy(&dwsourcechannelgroup), ::core::mem::transmute_copy(&dwsourcechannel), ::core::mem::transmute_copy(&dwdestinationchannelgroup), ::core::mem::transmute_copy(&dwdestinationchannel), ::core::mem::transmute(&pdestinationport)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ThruChannel: ThruChannel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicThru as ::windows::core::Interface>::IID
    }
}
