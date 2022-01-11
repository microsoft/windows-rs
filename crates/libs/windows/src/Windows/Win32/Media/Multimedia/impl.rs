#[cfg(feature = "Win32_Foundation")]
pub trait IAVIEditStreamImpl: Sized {
    fn Cut();
    fn Copy();
    fn Paste();
    fn Clone();
    fn SetInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIEditStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIEditStreamVtbl {
        unsafe extern "system" fn Cut<Impl: IAVIEditStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copy<Impl: IAVIEditStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Paste<Impl: IAVIEditStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpos: *mut i32, pllength: *mut i32, pstream: ::windows::core::RawPtr, lstart: i32, lend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IAVIEditStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInfo<Impl: IAVIEditStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cut::<Impl, IMPL_OFFSET>, Copy::<Impl, IMPL_OFFSET>, Paste::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, SetInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIEditStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIFileImpl: Sized {
    fn Info();
    fn GetStream();
    fn CreateStream();
    fn WriteData();
    fn ReadData();
    fn EndRecord();
    fn DeleteStream();
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIFileVtbl {
        unsafe extern "system" fn Info<Impl: IAVIFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IAVIFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStream<Impl: IAVIFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, psi: *const AVISTREAMINFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteData<Impl: IAVIFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadData<Impl: IAVIFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndRecord<Impl: IAVIFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteStream<Impl: IAVIFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Info::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, CreateStream::<Impl, IMPL_OFFSET>, WriteData::<Impl, IMPL_OFFSET>, ReadData::<Impl, IMPL_OFFSET>, EndRecord::<Impl, IMPL_OFFSET>, DeleteStream::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIFile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAVIPersistFileImpl: Sized + IPersistFileImpl + IPersistImpl {
    fn Reserved1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAVIPersistFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIPersistFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIPersistFileVtbl {
        unsafe extern "system" fn Reserved1<Impl: IAVIPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, SaveCompleted::<Impl, IMPL_OFFSET>, GetCurFile::<Impl, IMPL_OFFSET>, Reserved1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIPersistFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIStreamImpl: Sized {
    fn Create();
    fn Info();
    fn FindSample();
    fn ReadFormat();
    fn SetFormat();
    fn Read();
    fn Write();
    fn Delete();
    fn ReadData();
    fn WriteData();
    fn SetInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIStreamVtbl {
        unsafe extern "system" fn Create<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Info<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindSample<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lflags: i32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadFormat<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormat<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Read<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadData<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteData<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInfo<Impl: IAVIStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Info::<Impl, IMPL_OFFSET>,
            FindSample::<Impl, IMPL_OFFSET>,
            ReadFormat::<Impl, IMPL_OFFSET>,
            SetFormat::<Impl, IMPL_OFFSET>,
            Read::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            ReadData::<Impl, IMPL_OFFSET>,
            WriteData::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIStream as ::windows::core::Interface>::IID
    }
}
pub trait IAVIStreamingImpl: Sized {
    fn Begin();
    fn End();
}
impl IAVIStreamingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStreamingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIStreamingVtbl {
        unsafe extern "system" fn Begin<Impl: IAVIStreamingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IAVIStreamingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIStreaming as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IGetFrameImpl: Sized {
    fn GetFrame();
    fn Begin();
    fn End();
    fn SetFormat();
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IGetFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetFrameVtbl {
        unsafe extern "system" fn GetFrame<Impl: IGetFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin<Impl: IGetFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IGetFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormat<Impl: IGetFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFrame::<Impl, IMPL_OFFSET>, Begin::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>, SetFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetFrame as ::windows::core::Interface>::IID
    }
}
