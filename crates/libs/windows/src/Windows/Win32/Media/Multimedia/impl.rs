#[cfg(feature = "Win32_Foundation")]
pub trait IAVIEditStream_Impl: Sized {
    fn Cut(&self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()>;
    fn Copy(&self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()>;
    fn Paste(&self, plpos: *mut i32, pllength: *mut i32, pstream: &::core::option::Option<IAVIStream>, lstart: i32, lend: i32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IAVIStream>;
    fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIEditStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStream_Impl, const OFFSET: isize>() -> IAVIEditStream_Vtbl {
        unsafe extern "system" fn Cut<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cut(::core::mem::transmute_copy(&plstart), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Copy(::core::mem::transmute_copy(&plstart), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Paste<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpos: *mut i32, pllength: *mut i32, pstream: ::windows::core::RawPtr, lstart: i32, lend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Paste(::core::mem::transmute_copy(&plpos), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfo(::core::mem::transmute_copy(&lpinfo), ::core::mem::transmute_copy(&cbinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cut: Cut::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIEditStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIFile_Impl: Sized {
    fn Info(&self, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::Result<()>;
    fn GetStream(&self, ppstream: *mut ::core::option::Option<IAVIStream>, fcctype: u32, lparam: i32) -> ::windows::core::Result<()>;
    fn CreateStream(&self, ppstream: *mut ::core::option::Option<IAVIStream>, psi: *const AVISTREAMINFOW) -> ::windows::core::Result<()>;
    fn WriteData(&self, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::Result<()>;
    fn ReadData(&self, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::Result<()>;
    fn EndRecord(&self) -> ::windows::core::Result<()>;
    fn DeleteStream(&self, fcctype: u32, lparam: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>() -> IAVIFile_Vtbl {
        unsafe extern "system" fn Info<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Info(::core::mem::transmute_copy(&pfi), ::core::mem::transmute_copy(&lsize)).into()
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStream(::core::mem::transmute_copy(&ppstream), ::core::mem::transmute_copy(&fcctype), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn CreateStream<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, psi: *const AVISTREAMINFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateStream(::core::mem::transmute_copy(&ppstream), ::core::mem::transmute_copy(&psi)).into()
        }
        unsafe extern "system" fn WriteData<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteData(::core::mem::transmute_copy(&ckid), ::core::mem::transmute_copy(&lpdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn ReadData<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadData(::core::mem::transmute_copy(&ckid), ::core::mem::transmute_copy(&lpdata), ::core::mem::transmute_copy(&lpcbdata)).into()
        }
        unsafe extern "system" fn EndRecord<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndRecord().into()
        }
        unsafe extern "system" fn DeleteStream<Identity: ::windows::core::IUnknownImpl, Impl: IAVIFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteStream(::core::mem::transmute_copy(&fcctype), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Info: Info::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            ReadData: ReadData::<Identity, Impl, OFFSET>,
            EndRecord: EndRecord::<Identity, Impl, OFFSET>,
            DeleteStream: DeleteStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIFile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAVIPersistFile_Impl: Sized + super::super::System::Com::IPersist_Impl + super::super::System::Com::IPersistFile_Impl {
    fn Reserved1(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAVIPersistFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIPersistFile_Impl, const OFFSET: isize>() -> IAVIPersistFile_Vtbl {
        unsafe extern "system" fn Reserved1<Identity: ::windows::core::IUnknownImpl, Impl: IAVIPersistFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reserved1().into()
        }
        Self { base: super::super::System::Com::IPersistFile_Vtbl::new::<Identity, Impl, OFFSET>(), Reserved1: Reserved1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIPersistFile as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IPersist as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IPersistFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIStream_Impl: Sized {
    fn Create(&self, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn Info(&self, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::Result<()>;
    fn FindSample(&self, lpos: i32, lflags: i32) -> i32;
    fn ReadFormat(&self, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::Result<()>;
    fn SetFormat(&self, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::Result<()>;
    fn Read(&self, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::Result<()>;
    fn Write(&self, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::Result<()>;
    fn Delete(&self, lstart: i32, lsamples: i32) -> ::windows::core::Result<()>;
    fn ReadData(&self, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::Result<()>;
    fn WriteData(&self, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::Result<()>;
    fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAVIStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>() -> IAVIStream_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&lparam1), ::core::mem::transmute_copy(&lparam2)).into()
        }
        unsafe extern "system" fn Info<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Info(::core::mem::transmute_copy(&psi), ::core::mem::transmute_copy(&lsize)).into()
        }
        unsafe extern "system" fn FindSample<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lflags: i32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindSample(::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lflags))
        }
        unsafe extern "system" fn ReadFormat<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadFormat(::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lpformat), ::core::mem::transmute_copy(&lpcbformat)).into()
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lpformat), ::core::mem::transmute_copy(&cbformat)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&plbytes), ::core::mem::transmute_copy(&plsamples)).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&plsampwritten), ::core::mem::transmute_copy(&plbyteswritten)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples)).into()
        }
        unsafe extern "system" fn ReadData<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadData(::core::mem::transmute_copy(&fcc), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&lpcb)).into()
        }
        unsafe extern "system" fn WriteData<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteData(::core::mem::transmute_copy(&fcc), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfo(::core::mem::transmute_copy(&lpinfo), ::core::mem::transmute_copy(&cbinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Info: Info::<Identity, Impl, OFFSET>,
            FindSample: FindSample::<Identity, Impl, OFFSET>,
            ReadFormat: ReadFormat::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            ReadData: ReadData::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIStream as ::windows::core::Interface>::IID
    }
}
pub trait IAVIStreaming_Impl: Sized {
    fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::Result<()>;
    fn End(&self) -> ::windows::core::Result<()>;
}
impl IAVIStreaming_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStreaming_Impl, const OFFSET: isize>() -> IAVIStreaming_Vtbl {
        unsafe extern "system" fn Begin<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStreaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend), ::core::mem::transmute_copy(&lrate)).into()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl, Impl: IAVIStreaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).End().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Begin: Begin::<Identity, Impl, OFFSET>, End: End::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAVIStreaming as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IGetFrame_Impl: Sized {
    fn GetFrame(&self, lpos: i32) -> *mut ::core::ffi::c_void;
    fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::Result<()>;
    fn End(&self) -> ::windows::core::Result<()>;
    fn SetFormat(&self, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IGetFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetFrame_Impl, const OFFSET: isize>() -> IGetFrame_Vtbl {
        unsafe extern "system" fn GetFrame<Identity: ::windows::core::IUnknownImpl, Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpos: i32) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFrame(::core::mem::transmute_copy(&lpos))
        }
        unsafe extern "system" fn Begin<Identity: ::windows::core::IUnknownImpl, Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin(::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend), ::core::mem::transmute_copy(&lrate)).into()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl, Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).End().into()
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IGetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&lpbi), ::core::mem::transmute_copy(&lpbits), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFrame: GetFrame::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetFrame as ::windows::core::Interface>::IID
    }
}
