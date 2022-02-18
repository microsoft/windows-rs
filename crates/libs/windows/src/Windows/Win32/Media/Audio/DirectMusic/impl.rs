#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusic_Impl: Sized {
    fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()>;
    fn CreateMusicBuffer(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreatePort(&self, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::Result<()>;
    fn GetMasterClock(&self, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
    fn SetMasterClock(&self, rguidclock: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Activate(&self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDefaultPort(&self, pguidport: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: &::core::option::Option<super::DirectSound::IDirectSound>, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl IDirectMusic_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>() -> IDirectMusic_Vtbl {
        unsafe extern "system" fn EnumPort<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumPort(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pportcaps)).into()
        }
        unsafe extern "system" fn CreateMusicBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateMusicBuffer(::core::mem::transmute_copy(&pbufferdesc), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute(&punkouter)).into()
        }
        unsafe extern "system" fn CreatePort<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePort(::core::mem::transmute_copy(&rclsidport), ::core::mem::transmute_copy(&pportparams), ::core::mem::transmute_copy(&ppport), ::core::mem::transmute(&punkouter)).into()
        }
        unsafe extern "system" fn EnumMasterClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumMasterClock(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&lpclockinfo)).into()
        }
        unsafe extern "system" fn GetMasterClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMasterClock(::core::mem::transmute_copy(&pguidclock), ::core::mem::transmute_copy(&ppreferenceclock)).into()
        }
        unsafe extern "system" fn SetMasterClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidclock: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMasterClock(::core::mem::transmute_copy(&rguidclock)).into()
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetDefaultPort<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidport: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDefaultPort(::core::mem::transmute_copy(&pguidport)).into()
        }
        unsafe extern "system" fn SetDirectSound<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDirectSound(::core::mem::transmute(&pdirectsound), ::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumPort: EnumPort::<Identity, Impl, OFFSET>,
            CreateMusicBuffer: CreateMusicBuffer::<Identity, Impl, OFFSET>,
            CreatePort: CreatePort::<Identity, Impl, OFFSET>,
            EnumMasterClock: EnumMasterClock::<Identity, Impl, OFFSET>,
            GetMasterClock: GetMasterClock::<Identity, Impl, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            GetDefaultPort: GetDefaultPort::<Identity, Impl, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusic as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusic8_Impl: Sized + IDirectMusic_Impl {
    fn SetExternalMasterClock(&self, pclock: &::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl IDirectMusic8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic8_Impl, const OFFSET: isize>() -> IDirectMusic8_Vtbl {
        unsafe extern "system" fn SetExternalMasterClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusic8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExternalMasterClock(::core::mem::transmute(&pclock)).into()
        }
        Self { base: IDirectMusic_Vtbl::new::<Identity, Impl, OFFSET>(), SetExternalMasterClock: SetExternalMasterClock::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusic8 as ::windows::core::Interface>::IID || iid == &<IDirectMusic as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicBuffer_Impl: Sized {
    fn Flush(&self) -> ::windows::core::Result<()>;
    fn TotalTime(&self, prttime: *mut i64) -> ::windows::core::Result<()>;
    fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::Result<()>;
    fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::Result<()>;
    fn ResetReadPtr(&self) -> ::windows::core::Result<()>;
    fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetStartTime(&self, prt: *mut i64) -> ::windows::core::Result<()>;
    fn GetUsedBytes(&self, pcb: *mut u32) -> ::windows::core::Result<()>;
    fn GetMaxBytes(&self, pcb: *mut u32) -> ::windows::core::Result<()>;
    fn GetBufferFormat(&self, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetStartTime(&self, rt: i64) -> ::windows::core::Result<()>;
    fn SetUsedBytes(&self, cb: u32) -> ::windows::core::Result<()>;
}
impl IDirectMusicBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>() -> IDirectMusicBuffer_Vtbl {
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn TotalTime<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prttime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TotalTime(::core::mem::transmute_copy(&prttime)).into()
        }
        unsafe extern "system" fn PackStructured<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PackStructured(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannelmessage)).into()
        }
        unsafe extern "system" fn PackUnstructured<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PackUnstructured(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&lpb)).into()
        }
        unsafe extern "system" fn ResetReadPtr<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetReadPtr().into()
        }
        unsafe extern "system" fn GetNextEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextEvent(::core::mem::transmute_copy(&prt), ::core::mem::transmute_copy(&pdwchannelgroup), ::core::mem::transmute_copy(&pdwlength), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetRawBufferPtr<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRawBufferPtr(::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStartTime(::core::mem::transmute_copy(&prt)).into()
        }
        unsafe extern "system" fn GetUsedBytes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUsedBytes(::core::mem::transmute_copy(&pcb)).into()
        }
        unsafe extern "system" fn GetMaxBytes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMaxBytes(::core::mem::transmute_copy(&pcb)).into()
        }
        unsafe extern "system" fn GetBufferFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferFormat(::core::mem::transmute_copy(&pguidformat)).into()
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&rt)).into()
        }
        unsafe extern "system" fn SetUsedBytes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUsedBytes(::core::mem::transmute_copy(&cb)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Flush: Flush::<Identity, Impl, OFFSET>,
            TotalTime: TotalTime::<Identity, Impl, OFFSET>,
            PackStructured: PackStructured::<Identity, Impl, OFFSET>,
            PackUnstructured: PackUnstructured::<Identity, Impl, OFFSET>,
            ResetReadPtr: ResetReadPtr::<Identity, Impl, OFFSET>,
            GetNextEvent: GetNextEvent::<Identity, Impl, OFFSET>,
            GetRawBufferPtr: GetRawBufferPtr::<Identity, Impl, OFFSET>,
            GetStartTime: GetStartTime::<Identity, Impl, OFFSET>,
            GetUsedBytes: GetUsedBytes::<Identity, Impl, OFFSET>,
            GetMaxBytes: GetMaxBytes::<Identity, Impl, OFFSET>,
            GetBufferFormat: GetBufferFormat::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            SetUsedBytes: SetUsedBytes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicCollection_Impl: Sized {
    fn GetInstrument(&self, dwpatch: u32) -> ::windows::core::Result<IDirectMusicInstrument>;
    fn EnumInstrument(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: &::windows::core::PCWSTR, dwnamelen: u32) -> ::windows::core::Result<()>;
}
impl IDirectMusicCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicCollection_Impl, const OFFSET: isize>() -> IDirectMusicCollection_Vtbl {
        unsafe extern "system" fn GetInstrument<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpatch: u32, ppinstrument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstrument(::core::mem::transmute_copy(&dwpatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinstrument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumInstrument<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: ::windows::core::PCWSTR, dwnamelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumInstrument(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pdwpatch), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&dwnamelen)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInstrument: GetInstrument::<Identity, Impl, OFFSET>,
            EnumInstrument: EnumInstrument::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicCollection as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicDownload_Impl: Sized {
    fn GetBuffer(&self, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IDirectMusicDownload_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicDownload_Impl, const OFFSET: isize>() -> IDirectMusicDownload_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicDownload_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&ppvbuffer), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetBuffer: GetBuffer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicDownload as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicDownloadedInstrument_Impl: Sized {}
impl IDirectMusicDownloadedInstrument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicDownloadedInstrument_Impl, const OFFSET: isize>() -> IDirectMusicDownloadedInstrument_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicDownloadedInstrument as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicInstrument_Impl: Sized {
    fn GetPatch(&self, pdwpatch: *mut u32) -> ::windows::core::Result<()>;
    fn SetPatch(&self, dwpatch: u32) -> ::windows::core::Result<()>;
}
impl IDirectMusicInstrument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicInstrument_Impl, const OFFSET: isize>() -> IDirectMusicInstrument_Vtbl {
        unsafe extern "system" fn GetPatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicInstrument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPatch(::core::mem::transmute_copy(&pdwpatch)).into()
        }
        unsafe extern "system" fn SetPatch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicInstrument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPatch(::core::mem::transmute_copy(&dwpatch)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPatch: GetPatch::<Identity, Impl, OFFSET>,
            SetPatch: SetPatch::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicInstrument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
pub trait IDirectMusicPort_Impl: Sized {
    fn PlayBuffer(&self, pbuffer: &::core::option::Option<IDirectMusicBuffer>) -> ::windows::core::Result<()>;
    fn SetReadNotificationHandle(&self, hevent: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn Read(&self, pbuffer: &::core::option::Option<IDirectMusicBuffer>) -> ::windows::core::Result<()>;
    fn DownloadInstrument(&self, pinstrument: &::core::option::Option<IDirectMusicInstrument>, ppdownloadedinstrument: *mut ::core::option::Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::Result<()>;
    fn UnloadInstrument(&self, pdownloadedinstrument: &::core::option::Option<IDirectMusicDownloadedInstrument>) -> ::windows::core::Result<()>;
    fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock>;
    fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()>;
    fn Compact(&self) -> ::windows::core::Result<()>;
    fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> ::windows::core::Result<()>;
    fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> ::windows::core::Result<()>;
    fn Activate(&self, factive: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()>;
    fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: &::core::option::Option<super::DirectSound::IDirectSound>, pdirectsoundbuffer: &::core::option::Option<super::DirectSound::IDirectSoundBuffer>) -> ::windows::core::Result<()>;
    fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl IDirectMusicPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>() -> IDirectMusicPort_Vtbl {
        unsafe extern "system" fn PlayBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlayBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn SetReadNotificationHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReadNotificationHandle(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn DownloadInstrument<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstrument: ::windows::core::RawPtr, ppdownloadedinstrument: *mut ::windows::core::RawPtr, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DownloadInstrument(::core::mem::transmute(&pinstrument), ::core::mem::transmute_copy(&ppdownloadedinstrument), ::core::mem::transmute_copy(&pnoteranges), ::core::mem::transmute_copy(&dwnumnoteranges)).into()
        }
        unsafe extern "system" fn UnloadInstrument<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdownloadedinstrument: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnloadInstrument(::core::mem::transmute(&pdownloadedinstrument)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLatencyClock() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningStats<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRunningStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn Compact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&pportcaps)).into()
        }
        unsafe extern "system" fn DeviceIoControl<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceIoControl(::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&noutbuffersize), ::core::mem::transmute_copy(&lpbytesreturned), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroups: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumChannelGroups(::core::mem::transmute_copy(&dwchannelgroups)).into()
        }
        unsafe extern "system" fn GetNumChannelGroups<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwchannelgroups: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumChannelGroups(::core::mem::transmute_copy(&pdwchannelgroups)).into()
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factive: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&factive)).into()
        }
        unsafe extern "system" fn SetChannelPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetChannelPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwpriority)).into()
        }
        unsafe extern "system" fn SetDirectSound<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDirectSound(::core::mem::transmute(&pdirectsound), ::core::mem::transmute(&pdirectsoundbuffer)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&pdwwaveformatexsize), ::core::mem::transmute_copy(&pdwbuffersize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PlayBuffer: PlayBuffer::<Identity, Impl, OFFSET>,
            SetReadNotificationHandle: SetReadNotificationHandle::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            DownloadInstrument: DownloadInstrument::<Identity, Impl, OFFSET>,
            UnloadInstrument: UnloadInstrument::<Identity, Impl, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, Impl, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, Impl, OFFSET>,
            Compact: Compact::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, Impl, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, Impl, OFFSET>,
            GetNumChannelGroups: GetNumChannelGroups::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, Impl, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, Impl, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicPort as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicPortDownload_Impl: Sized {
    fn GetBuffer(&self, dwdlid: u32) -> ::windows::core::Result<IDirectMusicDownload>;
    fn AllocateBuffer(&self, dwsize: u32) -> ::windows::core::Result<IDirectMusicDownload>;
    fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::Result<()>;
    fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()>;
    fn Download(&self, pidmdownload: &::core::option::Option<IDirectMusicDownload>) -> ::windows::core::Result<()>;
    fn Unload(&self, pidmdownload: &::core::option::Option<IDirectMusicDownload>) -> ::windows::core::Result<()>;
}
impl IDirectMusicPortDownload_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownload_Impl, const OFFSET: isize>() -> IDirectMusicPortDownload_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&dwdlid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidmdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllocateBuffer(::core::mem::transmute_copy(&dwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidmdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDLId<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDLId(::core::mem::transmute_copy(&pdwstartdlid), ::core::mem::transmute_copy(&dwcount)).into()
        }
        unsafe extern "system" fn GetAppend<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAppend(::core::mem::transmute_copy(&pdwappend)).into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Download(::core::mem::transmute(&pidmdownload)).into()
        }
        unsafe extern "system" fn Unload<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicPortDownload_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unload(::core::mem::transmute(&pidmdownload)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            GetDLId: GetDLId::<Identity, Impl, OFFSET>,
            GetAppend: GetAppend::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            Unload: Unload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicPortDownload as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectMusicSynth_Impl: Sized {
    fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::core::Result<()>;
    fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::Result<()>;
    fn Unload(&self, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::Result<()>;
    fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()>;
    fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()>;
    fn SetMasterClock(&self, pclock: &::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
    fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock>;
    fn Activate(&self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSynthSink(&self, psynthsink: &::core::option::Option<IDirectMusicSynthSink>) -> ::windows::core::Result<()>;
    fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::Result<()>;
    fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()>;
    fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()>;
    fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectMusicSynth_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>() -> IDirectMusicSynth_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pportparams)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwgroups: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumChannelGroups(::core::mem::transmute_copy(&dwgroups)).into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Download(::core::mem::transmute_copy(&phdownload), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&pbfree)).into()
        }
        unsafe extern "system" fn Unload<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unload(::core::mem::transmute_copy(&hdownload), ::core::mem::transmute_copy(&lpfreehandle), ::core::mem::transmute_copy(&huserdata)).into()
        }
        unsafe extern "system" fn PlayBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlayBuffer(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffer)).into()
        }
        unsafe extern "system" fn GetRunningStats<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRunningStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetPortCaps<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPortCaps(::core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn SetMasterClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMasterClock(::core::mem::transmute(&pclock)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLatencyClock() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetSynthSink<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynthsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSynthSink(::core::mem::transmute(&psynthsink)).into()
        }
        unsafe extern "system" fn Render<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&llposition)).into()
        }
        unsafe extern "system" fn SetChannelPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetChannelPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChannelPriority(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwpriority)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&pdwwaveformatexsize)).into()
        }
        unsafe extern "system" fn GetAppend<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAppend(::core::mem::transmute_copy(&pdwappend)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            Unload: Unload::<Identity, Impl, OFFSET>,
            PlayBuffer: PlayBuffer::<Identity, Impl, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, Impl, OFFSET>,
            GetPortCaps: GetPortCaps::<Identity, Impl, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, Impl, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            SetSynthSink: SetSynthSink::<Identity, Impl, OFFSET>,
            Render: Render::<Identity, Impl, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, Impl, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetAppend: GetAppend::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicSynth as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectMusicSynth8_Impl: Sized + IDirectMusicSynth_Impl {
    fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::Result<()>;
    fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> ::windows::core::Result<()>;
    fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::Result<()>;
    fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectMusicSynth8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8_Impl, const OFFSET: isize>() -> IDirectMusicSynth8_Vtbl {
        unsafe extern "system" fn PlayVoice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlayVoice(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwvoiceid), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwdlid), ::core::mem::transmute_copy(&prpitch), ::core::mem::transmute_copy(&vrvolume), ::core::mem::transmute_copy(&stvoicestart), ::core::mem::transmute_copy(&stloopstart), ::core::mem::transmute_copy(&stloopend)).into()
        }
        unsafe extern "system" fn StopVoice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopVoice(::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwvoiceid)).into()
        }
        unsafe extern "system" fn GetVoiceState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVoiceState(::core::mem::transmute_copy(&dwvoice), ::core::mem::transmute_copy(&cbvoice), ::core::mem::transmute_copy(&dwvoicestate)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh(::core::mem::transmute_copy(&dwdownloadid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn AssignChannelToBuses<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynth8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssignChannelToBuses(::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwbuses), ::core::mem::transmute_copy(&cbuses)).into()
        }
        Self {
            base: IDirectMusicSynth_Vtbl::new::<Identity, Impl, OFFSET>(),
            PlayVoice: PlayVoice::<Identity, Impl, OFFSET>,
            StopVoice: StopVoice::<Identity, Impl, OFFSET>,
            GetVoiceState: GetVoiceState::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            AssignChannelToBuses: AssignChannelToBuses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicSynth8 as ::windows::core::Interface>::IID || iid == &<IDirectMusicSynth as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusicSynthSink_Impl: Sized {
    fn Init(&self, psynth: &::core::option::Option<IDirectMusicSynth>) -> ::windows::core::Result<()>;
    fn SetMasterClock(&self, pclock: &::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()>;
    fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock>;
    fn Activate(&self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> ::windows::core::Result<()>;
    fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: &::core::option::Option<super::DirectSound::IDirectSound>, pdirectsoundbuffer: &::core::option::Option<super::DirectSound::IDirectSoundBuffer>) -> ::windows::core::Result<()>;
    fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl IDirectMusicSynthSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>() -> IDirectMusicSynthSink_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynth: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute(&psynth)).into()
        }
        unsafe extern "system" fn SetMasterClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMasterClock(::core::mem::transmute(&pclock)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLatencyClock() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SampleToRefTime<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SampleToRefTime(::core::mem::transmute_copy(&llsampletime), ::core::mem::transmute_copy(&prftime)).into()
        }
        unsafe extern "system" fn RefTimeToSample<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefTimeToSample(::core::mem::transmute_copy(&rftime), ::core::mem::transmute_copy(&pllsampletime)).into()
        }
        unsafe extern "system" fn SetDirectSound<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDirectSound(::core::mem::transmute(&pdirectsound), ::core::mem::transmute(&pdirectsoundbuffer)).into()
        }
        unsafe extern "system" fn GetDesiredBufferSize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicSynthSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesiredBufferSize(::core::mem::transmute_copy(&pdwbuffersizeinsamples)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, Impl, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            SampleToRefTime: SampleToRefTime::<Identity, Impl, OFFSET>,
            RefTimeToSample: RefTimeToSample::<Identity, Impl, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, Impl, OFFSET>,
            GetDesiredBufferSize: GetDesiredBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicSynthSink as ::windows::core::Interface>::IID
    }
}
pub trait IDirectMusicThru_Impl: Sized {
    fn ThruChannel(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: &::core::option::Option<IDirectMusicPort>) -> ::windows::core::Result<()>;
}
impl IDirectMusicThru_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicThru_Impl, const OFFSET: isize>() -> IDirectMusicThru_Vtbl {
        unsafe extern "system" fn ThruChannel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectMusicThru_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ThruChannel(::core::mem::transmute_copy(&dwsourcechannelgroup), ::core::mem::transmute_copy(&dwsourcechannel), ::core::mem::transmute_copy(&dwdestinationchannelgroup), ::core::mem::transmute_copy(&dwdestinationchannel), ::core::mem::transmute(&pdestinationport)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ThruChannel: ThruChannel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectMusicThru as ::windows::core::Interface>::IID
    }
}
