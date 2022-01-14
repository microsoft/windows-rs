#[cfg(feature = "Win32_Foundation")]
pub trait IAVIEditStream_Impl: Sized {
    fn Cut(&mut self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()>;
    fn Copy(&mut self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()>;
    fn Paste(&mut self, plpos: *mut i32, pllength: *mut i32, pstream: ::core::option::Option<IAVIStream>, lstart: i32, lend: i32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IAVIStream>;
    fn SetInfo(&mut self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIEditStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIEditStream_Vtbl {
        unsafe extern "system" fn Cut<Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cut(::core::mem::transmute_copy(&plstart), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Copy<Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Copy(::core::mem::transmute_copy(&plstart), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Paste<Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpos: *mut i32, pllength: *mut i32, pstream: ::windows::core::RawPtr, lstart: i32, lend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Paste(::core::mem::transmute_copy(&plpos), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend)).into()
        }
        unsafe extern "system" fn Clone<Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfo<Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfo(::core::mem::transmute_copy(&lpinfo), ::core::mem::transmute_copy(&cbinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Cut: Cut::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Paste: Paste::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            SetInfo: SetInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIEditStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIFile_Impl: Sized {
    fn Info(&mut self, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::Result<()>;
    fn GetStream(&mut self, ppstream: *mut ::core::option::Option<IAVIStream>, fcctype: u32, lparam: i32) -> ::windows::core::Result<()>;
    fn CreateStream(&mut self, ppstream: *mut ::core::option::Option<IAVIStream>, psi: *const AVISTREAMINFOW) -> ::windows::core::Result<()>;
    fn WriteData(&mut self, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::Result<()>;
    fn ReadData(&mut self, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::Result<()>;
    fn EndRecord(&mut self) -> ::windows::core::Result<()>;
    fn DeleteStream(&mut self, fcctype: u32, lparam: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIFile_Vtbl {
        unsafe extern "system" fn Info<Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Info(::core::mem::transmute_copy(&pfi), ::core::mem::transmute_copy(&lsize)).into()
        }
        unsafe extern "system" fn GetStream<Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStream(::core::mem::transmute_copy(&ppstream), ::core::mem::transmute_copy(&fcctype), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn CreateStream<Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, psi: *const AVISTREAMINFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateStream(::core::mem::transmute_copy(&ppstream), ::core::mem::transmute_copy(&psi)).into()
        }
        unsafe extern "system" fn WriteData<Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteData(::core::mem::transmute_copy(&ckid), ::core::mem::transmute_copy(&lpdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn ReadData<Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadData(::core::mem::transmute_copy(&ckid), ::core::mem::transmute_copy(&lpdata), ::core::mem::transmute_copy(&lpcbdata)).into()
        }
        unsafe extern "system" fn EndRecord<Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndRecord().into()
        }
        unsafe extern "system" fn DeleteStream<Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteStream(::core::mem::transmute_copy(&fcctype), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Info: Info::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            CreateStream: CreateStream::<Impl, IMPL_OFFSET>,
            WriteData: WriteData::<Impl, IMPL_OFFSET>,
            ReadData: ReadData::<Impl, IMPL_OFFSET>,
            EndRecord: EndRecord::<Impl, IMPL_OFFSET>,
            DeleteStream: DeleteStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIFile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAVIPersistFile_Impl: Sized + super::super::System::Com::IPersist_Impl + super::super::System::Com::IPersistFile_Impl {
    fn Reserved1(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAVIPersistFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIPersistFile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIPersistFile_Vtbl {
        unsafe extern "system" fn Reserved1<Impl: IAVIPersistFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1().into()
        }
        Self { base: super::super::System::Com::IPersistFile_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Reserved1: Reserved1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIPersistFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIStream_Impl: Sized {
    fn Create(&mut self, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn Info(&mut self, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::Result<()>;
    fn FindSample(&mut self, lpos: i32, lflags: i32) -> i32;
    fn ReadFormat(&mut self, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::Result<()>;
    fn SetFormat(&mut self, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::Result<()>;
    fn Read(&mut self, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::Result<()>;
    fn Write(&mut self, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::Result<()>;
    fn Delete(&mut self, lstart: i32, lsamples: i32) -> ::windows::core::Result<()>;
    fn ReadData(&mut self, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::Result<()>;
    fn WriteData(&mut self, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::Result<()>;
    fn SetInfo(&mut self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIStream_Vtbl {
        unsafe extern "system" fn Create<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&lparam1), ::core::mem::transmute_copy(&lparam2)).into()
        }
        unsafe extern "system" fn Info<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Info(::core::mem::transmute_copy(&psi), ::core::mem::transmute_copy(&lsize)).into()
        }
        unsafe extern "system" fn FindSample<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lflags: i32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindSample(::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lflags))
        }
        unsafe extern "system" fn ReadFormat<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadFormat(::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lpformat), ::core::mem::transmute_copy(&lpcbformat)).into()
        }
        unsafe extern "system" fn SetFormat<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lpformat), ::core::mem::transmute_copy(&cbformat)).into()
        }
        unsafe extern "system" fn Read<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&plbytes), ::core::mem::transmute_copy(&plsamples)).into()
        }
        unsafe extern "system" fn Write<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&plsampwritten), ::core::mem::transmute_copy(&plbyteswritten)).into()
        }
        unsafe extern "system" fn Delete<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples)).into()
        }
        unsafe extern "system" fn ReadData<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadData(::core::mem::transmute_copy(&fcc), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&lpcb)).into()
        }
        unsafe extern "system" fn WriteData<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteData(::core::mem::transmute_copy(&fcc), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn SetInfo<Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfo(::core::mem::transmute_copy(&lpinfo), ::core::mem::transmute_copy(&cbinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Info: Info::<Impl, IMPL_OFFSET>,
            FindSample: FindSample::<Impl, IMPL_OFFSET>,
            ReadFormat: ReadFormat::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            ReadData: ReadData::<Impl, IMPL_OFFSET>,
            WriteData: WriteData::<Impl, IMPL_OFFSET>,
            SetInfo: SetInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIStream as ::windows::core::Interface>::IID
    }
}
pub trait IAVIStreaming_Impl: Sized {
    fn Begin(&mut self, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::Result<()>;
    fn End(&mut self) -> ::windows::core::Result<()>;
}
impl IAVIStreaming_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStreaming_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAVIStreaming_Vtbl {
        unsafe extern "system" fn Begin<Impl: IAVIStreaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend), ::core::mem::transmute_copy(&lrate)).into()
        }
        unsafe extern "system" fn End<Impl: IAVIStreaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Begin: Begin::<Impl, IMPL_OFFSET>, End: End::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIStreaming as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IGetFrame_Impl: Sized {
    fn GetFrame(&mut self, lpos: i32) -> *mut ::core::ffi::c_void;
    fn Begin(&mut self, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::Result<()>;
    fn End(&mut self) -> ::windows::core::Result<()>;
    fn SetFormat(&mut self, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IGetFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetFrame_Vtbl {
        unsafe extern "system" fn GetFrame<Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFrame(::core::mem::transmute_copy(&lpos))
        }
        unsafe extern "system" fn Begin<Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend), ::core::mem::transmute_copy(&lrate)).into()
        }
        unsafe extern "system" fn End<Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End().into()
        }
        unsafe extern "system" fn SetFormat<Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&lpbi), ::core::mem::transmute_copy(&lpbits), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFrame: GetFrame::<Impl, IMPL_OFFSET>,
            Begin: Begin::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetFrame as ::windows::core::Interface>::IID
    }
}
