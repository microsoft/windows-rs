pub trait IAVIEditStreamImpl: Sized {
    fn Cut();
    fn Copy();
    fn Paste();
    fn Clone();
    fn SetInfo();
}
impl ::windows::core::RuntimeName for IAVIEditStream {
    const NAME: &'static str = "Windows.Win32.Media.Multimedia.IAVIEditStream";
}
impl IAVIEditStreamVtbl {
    pub const fn new<Impl: IAVIEditStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAVIEditStreamVtbl {
        unsafe extern "system" fn Cut<Impl: IAVIEditStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cut(plstart, pllength, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: IAVIEditStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy(plstart, pllength, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Impl: IAVIEditStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpos: *mut i32, pllength: *mut i32, pstream: ::windows::core::RawPtr, lstart: i32, lend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Paste(plpos, pllength, &*(&pstream as *const <IAVIStream as ::windows::core::Abi>::Abi as *const <IAVIStream as ::windows::core::DefaultType>::DefaultType), lstart, lend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IAVIEditStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfo<Impl: IAVIEditStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInfo(&*(&lpinfo as *const <AVISTREAMINFOW as ::windows::core::Abi>::Abi as *const <AVISTREAMINFOW as ::windows::core::DefaultType>::DefaultType), cbinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAVIEditStream>, base.5, Cut::<Impl, OFFSET>, Copy::<Impl, OFFSET>, Paste::<Impl, OFFSET>, Clone::<Impl, OFFSET>, SetInfo::<Impl, OFFSET>)
    }
}
pub trait IAVIFileImpl: Sized {
    fn Info();
    fn GetStream();
    fn CreateStream();
    fn WriteData();
    fn ReadData();
    fn EndRecord();
    fn DeleteStream();
}
impl ::windows::core::RuntimeName for IAVIFile {
    const NAME: &'static str = "Windows.Win32.Media.Multimedia.IAVIFile";
}
impl IAVIFileVtbl {
    pub const fn new<Impl: IAVIFileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAVIFileVtbl {
        unsafe extern "system" fn Info<Impl: IAVIFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Info(::core::mem::transmute_copy(&pfi), lsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAVIFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&ppstream), fcctype, lparam) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStream<Impl: IAVIFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, psi: *const AVISTREAMINFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStream(::core::mem::transmute_copy(&ppstream), &*(&psi as *const <AVISTREAMINFOW as ::windows::core::Abi>::Abi as *const <AVISTREAMINFOW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteData<Impl: IAVIFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteData(ckid, &*(&lpdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadData<Impl: IAVIFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadData(ckid, ::core::mem::transmute_copy(&lpdata), lpcbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRecord<Impl: IAVIFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndRecord() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteStream<Impl: IAVIFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteStream(fcctype, lparam) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAVIFile>, base.5, Info::<Impl, OFFSET>, GetStream::<Impl, OFFSET>, CreateStream::<Impl, OFFSET>, WriteData::<Impl, OFFSET>, ReadData::<Impl, OFFSET>, EndRecord::<Impl, OFFSET>, DeleteStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAVIPersistFileImpl: Sized + IPersistFileImpl + IPersistImpl {
    fn Reserved1();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAVIPersistFile {
    const NAME: &'static str = "Windows.Win32.Media.Multimedia.IAVIPersistFile";
}
#[cfg(feature = "Win32_System_Com")]
impl IAVIPersistFileVtbl {
    pub const fn new<Impl: IAVIPersistFileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAVIPersistFileVtbl {
        unsafe extern "system" fn Reserved1<Impl: IAVIPersistFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reserved1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAVIPersistFile>, base.5, Reserved1::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IAVIStream {
    const NAME: &'static str = "Windows.Win32.Media.Multimedia.IAVIStream";
}
impl IAVIStreamVtbl {
    pub const fn new<Impl: IAVIStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAVIStreamVtbl {
        unsafe extern "system" fn Create<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&lparam1 as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam2 as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Info(::core::mem::transmute_copy(&psi), lsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindSample<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32, lflags: i32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindSample(lpos, lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadFormat<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadFormat(lpos, ::core::mem::transmute_copy(&lpformat), lpcbformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormat(lpos, &*(&lpformat as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Read(lstart, lsamples, ::core::mem::transmute_copy(&lpbuffer), cbbuffer, ::core::mem::transmute_copy(&plbytes), ::core::mem::transmute_copy(&plsamples)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Write(lstart, lsamples, &*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbbuffer, dwflags, ::core::mem::transmute_copy(&plsampwritten), ::core::mem::transmute_copy(&plbyteswritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(lstart, lsamples) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadData<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadData(fcc, ::core::mem::transmute_copy(&lp), lpcb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteData<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteData(fcc, &*(&lp as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfo<Impl: IAVIStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInfo(&*(&lpinfo as *const <AVISTREAMINFOW as ::windows::core::Abi>::Abi as *const <AVISTREAMINFOW as ::windows::core::DefaultType>::DefaultType), cbinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAVIStream>, base.5, Create::<Impl, OFFSET>, Info::<Impl, OFFSET>, FindSample::<Impl, OFFSET>, ReadFormat::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, Read::<Impl, OFFSET>, Write::<Impl, OFFSET>, Delete::<Impl, OFFSET>, ReadData::<Impl, OFFSET>, WriteData::<Impl, OFFSET>, SetInfo::<Impl, OFFSET>)
    }
}
pub trait IAVIStreamingImpl: Sized {
    fn Begin();
    fn End();
}
impl ::windows::core::RuntimeName for IAVIStreaming {
    const NAME: &'static str = "Windows.Win32.Media.Multimedia.IAVIStreaming";
}
impl IAVIStreamingVtbl {
    pub const fn new<Impl: IAVIStreamingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAVIStreamingVtbl {
        unsafe extern "system" fn Begin<Impl: IAVIStreamingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin(lstart, lend, lrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IAVIStreamingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).End() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAVIStreaming>, base.5, Begin::<Impl, OFFSET>, End::<Impl, OFFSET>)
    }
}
pub trait IGetFrameImpl: Sized {
    fn GetFrame();
    fn Begin();
    fn End();
    fn SetFormat();
}
impl ::windows::core::RuntimeName for IGetFrame {
    const NAME: &'static str = "Windows.Win32.Media.Multimedia.IGetFrame";
}
impl IGetFrameVtbl {
    pub const fn new<Impl: IGetFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetFrameVtbl {
        unsafe extern "system" fn GetFrame<Impl: IGetFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrame(lpos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin<Impl: IGetFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin(lstart, lend, lrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IGetFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).End() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: IGetFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormat(&*(&lpbi as *const <super::super::Graphics::Gdi::BITMAPINFOHEADER as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::BITMAPINFOHEADER as ::windows::core::DefaultType>::DefaultType), &*(&lpbits as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), x, y, dx, dy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetFrame>, base.5, GetFrame::<Impl, OFFSET>, Begin::<Impl, OFFSET>, End::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>)
    }
}
