pub trait IDirectWriterLockImpl: Sized {
    fn WaitForWriteAccess();
    fn ReleaseWriteAccess();
    fn HaveWriteAccess();
}
impl ::windows::core::RuntimeName for IDirectWriterLock {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IDirectWriterLock";
}
impl IDirectWriterLockVtbl {
    pub const fn new<Impl: IDirectWriterLockImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectWriterLockVtbl {
        unsafe extern "system" fn WaitForWriteAccess<Impl: IDirectWriterLockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForWriteAccess(dwtimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseWriteAccess<Impl: IDirectWriterLockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseWriteAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HaveWriteAccess<Impl: IDirectWriterLockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HaveWriteAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectWriterLock>, base.5, WaitForWriteAccess::<Impl, OFFSET>, ReleaseWriteAccess::<Impl, OFFSET>, HaveWriteAccess::<Impl, OFFSET>)
    }
}
pub trait IEnumSTATPROPSETSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSTATPROPSETSTG {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IEnumSTATPROPSETSTG";
}
impl IEnumSTATPROPSETSTGVtbl {
    pub const fn new<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSTATPROPSETSTGVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSTATPROPSETSTG>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSTATPROPSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSTATPROPSTG {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IEnumSTATPROPSTG";
}
impl IEnumSTATPROPSTGVtbl {
    pub const fn new<Impl: IEnumSTATPROPSTGImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSTATPROPSTGVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATPROPSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATPROPSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATPROPSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATPROPSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSTATPROPSTG>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSTATSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSTATSTG {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IEnumSTATSTG";
}
impl IEnumSTATSTGVtbl {
    pub const fn new<Impl: IEnumSTATSTGImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSTATSTGVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATSTGImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSTATSTG>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IFillLockBytesImpl: Sized {
    fn FillAppend();
    fn FillAt();
    fn SetFillSize();
    fn Terminate();
}
impl ::windows::core::RuntimeName for IFillLockBytes {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IFillLockBytes";
}
impl IFillLockBytesVtbl {
    pub const fn new<Impl: IFillLockBytesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFillLockBytesVtbl {
        unsafe extern "system" fn FillAppend<Impl: IFillLockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillAppend(&*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cb, ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillAt<Impl: IFillLockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillAt(uloffset, &*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cb, ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillSize<Impl: IFillLockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFillSize(ulsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IFillLockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminate(&*(&bcanceled as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFillLockBytes>, base.5, FillAppend::<Impl, OFFSET>, FillAt::<Impl, OFFSET>, SetFillSize::<Impl, OFFSET>, Terminate::<Impl, OFFSET>)
    }
}
pub trait ILayoutStorageImpl: Sized {
    fn LayoutScript();
    fn BeginMonitor();
    fn EndMonitor();
    fn ReLayoutDocfile();
    fn ReLayoutDocfileOnILockBytes();
}
impl ::windows::core::RuntimeName for ILayoutStorage {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.ILayoutStorage";
}
impl ILayoutStorageVtbl {
    pub const fn new<Impl: ILayoutStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILayoutStorageVtbl {
        unsafe extern "system" fn LayoutScript<Impl: ILayoutStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LayoutScript(&*(&pstoragelayout as *const <super::StorageLayout as ::windows::core::Abi>::Abi as *const <super::StorageLayout as ::windows::core::DefaultType>::DefaultType), nentries, glfinterleavedflag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMonitor<Impl: ILayoutStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginMonitor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMonitor<Impl: ILayoutStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndMonitor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReLayoutDocfile<Impl: ILayoutStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsnewdfname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReLayoutDocfile(&*(&pwcsnewdfname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Impl: ILayoutStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pilockbytes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReLayoutDocfileOnILockBytes(&*(&pilockbytes as *const <ILockBytes as ::windows::core::Abi>::Abi as *const <ILockBytes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILayoutStorage>, base.5, LayoutScript::<Impl, OFFSET>, BeginMonitor::<Impl, OFFSET>, EndMonitor::<Impl, OFFSET>, ReLayoutDocfile::<Impl, OFFSET>, ReLayoutDocfileOnILockBytes::<Impl, OFFSET>)
    }
}
pub trait ILockBytesImpl: Sized {
    fn ReadAt();
    fn WriteAt();
    fn Flush();
    fn SetSize();
    fn LockRegion();
    fn UnlockRegion();
    fn Stat();
}
impl ::windows::core::RuntimeName for ILockBytes {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.ILockBytes";
}
impl ILockBytesVtbl {
    pub const fn new<Impl: ILockBytesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILockBytesVtbl {
        unsafe extern "system" fn ReadAt<Impl: ILockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadAt(uloffset, ::core::mem::transmute_copy(&pv), cb, ::core::mem::transmute_copy(&pcbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteAt<Impl: ILockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteAt(uloffset, &*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cb, ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: ILockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: ILockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSize(cb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRegion<Impl: ILockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockRegion(liboffset, cb, dwlocktype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockRegion<Impl: ILockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlockRegion(liboffset, cb, dwlocktype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stat<Impl: ILockBytesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stat(::core::mem::transmute_copy(&pstatstg), grfstatflag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILockBytes>, base.5, ReadAt::<Impl, OFFSET>, WriteAt::<Impl, OFFSET>, Flush::<Impl, OFFSET>, SetSize::<Impl, OFFSET>, LockRegion::<Impl, OFFSET>, UnlockRegion::<Impl, OFFSET>, Stat::<Impl, OFFSET>)
    }
}
pub trait IPersistStorageImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn InitNew();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn HandsOffStorage();
}
impl ::windows::core::RuntimeName for IPersistStorage {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IPersistStorage";
}
impl IPersistStorageVtbl {
    pub const fn new<Impl: IPersistStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPersistStorageVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitNew<Impl: IPersistStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitNew(&*(&pstg as *const <IStorage as ::windows::core::Abi>::Abi as *const <IStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Load(&*(&pstg as *const <IStorage as ::windows::core::Abi>::Abi as *const <IStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstgsave: ::windows::core::RawPtr, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Save(&*(&pstgsave as *const <IStorage as ::windows::core::Abi>::Abi as *const <IStorage as ::windows::core::DefaultType>::DefaultType), &*(&fsameasload as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstgnew: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveCompleted(&*(&pstgnew as *const <IStorage as ::windows::core::Abi>::Abi as *const <IStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandsOffStorage<Impl: IPersistStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandsOffStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPersistStorage>, base.5, IsDirty::<Impl, OFFSET>, InitNew::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>, SaveCompleted::<Impl, OFFSET>, HandsOffStorage::<Impl, OFFSET>)
    }
}
pub trait IPropertyBagImpl: Sized {
    fn Read();
    fn Write();
}
impl ::windows::core::RuntimeName for IPropertyBag {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IPropertyBag";
}
impl IPropertyBagVtbl {
    pub const fn new<Impl: IPropertyBagImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyBagVtbl {
        unsafe extern "system" fn Read<Impl: IPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *mut super::VARIANT, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Read(&*(&pszpropname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvar as *const <super::VARIANT as ::windows::core::Abi>::Abi as *const <super::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&perrorlog as *const <super::IErrorLog as ::windows::core::Abi>::Abi as *const <super::IErrorLog as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Write(&*(&pszpropname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvar as *const <super::VARIANT as ::windows::core::Abi>::Abi as *const <super::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyBag>, base.5, Read::<Impl, OFFSET>, Write::<Impl, OFFSET>)
    }
}
pub trait IPropertyBag2Impl: Sized {
    fn Read();
    fn Write();
    fn CountProperties();
    fn GetPropertyInfo();
    fn LoadObject();
}
impl ::windows::core::RuntimeName for IPropertyBag2 {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IPropertyBag2";
}
impl IPropertyBag2Vtbl {
    pub const fn new<Impl: IPropertyBag2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyBag2Vtbl {
        unsafe extern "system" fn Read<Impl: IPropertyBag2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::windows::core::RawPtr, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Read(cproperties, &*(&ppropbag as *const <PROPBAG2 as ::windows::core::Abi>::Abi as *const <PROPBAG2 as ::windows::core::DefaultType>::DefaultType), &*(&perrlog as *const <super::IErrorLog as ::windows::core::Abi>::Abi as *const <super::IErrorLog as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarvalue), phrerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IPropertyBag2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Write(cproperties, &*(&ppropbag as *const <PROPBAG2 as ::windows::core::Abi>::Abi as *const <PROPBAG2 as ::windows::core::DefaultType>::DefaultType), &*(&pvarvalue as *const <super::VARIANT as ::windows::core::Abi>::Abi as *const <super::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountProperties<Impl: IPropertyBag2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CountProperties(::core::mem::transmute_copy(&pcproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Impl: IPropertyBag2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyInfo(iproperty, cproperties, ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute_copy(&pcproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadObject<Impl: IPropertyBag2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrname: super::super::super::Foundation::PWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadObject(
                &*(&pstrname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwhint,
                &*(&punkobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&perrlog as *const <super::IErrorLog as ::windows::core::Abi>::Abi as *const <super::IErrorLog as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyBag2>, base.5, Read::<Impl, OFFSET>, Write::<Impl, OFFSET>, CountProperties::<Impl, OFFSET>, GetPropertyInfo::<Impl, OFFSET>, LoadObject::<Impl, OFFSET>)
    }
}
pub trait IPropertySetStorageImpl: Sized {
    fn Create();
    fn Open();
    fn Delete();
    fn Enum();
}
impl ::windows::core::RuntimeName for IPropertySetStorage {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IPropertySetStorage";
}
impl IPropertySetStorageVtbl {
    pub const fn new<Impl: IPropertySetStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertySetStorageVtbl {
        unsafe extern "system" fn Create<Impl: IPropertySetStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&rfmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), grfflags, grfmode, ::core::mem::transmute_copy(&ppprstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IPropertySetStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(&*(&rfmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), grfmode, ::core::mem::transmute_copy(&ppprstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPropertySetStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&rfmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Impl: IPropertySetStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enum(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertySetStorage>, base.5, Create::<Impl, OFFSET>, Open::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Enum::<Impl, OFFSET>)
    }
}
pub trait IPropertyStorageImpl: Sized {
    fn ReadMultiple();
    fn WriteMultiple();
    fn DeleteMultiple();
    fn ReadPropertyNames();
    fn WritePropertyNames();
    fn DeletePropertyNames();
    fn Commit();
    fn Revert();
    fn Enum();
    fn SetTimes();
    fn SetClass();
    fn Stat();
}
impl ::windows::core::RuntimeName for IPropertyStorage {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IPropertyStorage";
}
impl IPropertyStorageVtbl {
    pub const fn new<Impl: IPropertyStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyStorageVtbl {
        unsafe extern "system" fn ReadMultiple<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadMultiple(cpspec, &*(&rgpspec as *const <PROPSPEC as ::windows::core::Abi>::Abi as *const <PROPSPEC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rgpropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteMultiple<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteMultiple(cpspec, &*(&rgpspec as *const <PROPSPEC as ::windows::core::Abi>::Abi as *const <PROPSPEC as ::windows::core::DefaultType>::DefaultType), &*(&rgpropvar as *const <PROPVARIANT as ::windows::core::Abi>::Abi as *const <PROPVARIANT as ::windows::core::DefaultType>::DefaultType), propidnamefirst) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMultiple<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteMultiple(cpspec, &*(&rgpspec as *const <PROPSPEC as ::windows::core::Abi>::Abi as *const <PROPSPEC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadPropertyNames(cpropid, rgpropid, ::core::mem::transmute_copy(&rglpwstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePropertyNames<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WritePropertyNames(cpropid, rgpropid, &*(&rglpwstrname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeletePropertyNames(cpropid, rgpropid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit(grfcommitflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revert<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Revert() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enum(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimes<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimes(
                &*(&pctime as *const <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&patime as *const <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&pmtime as *const <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClass(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stat<Impl: IPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stat(::core::mem::transmute_copy(&pstatpsstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyStorage>, base.5, ReadMultiple::<Impl, OFFSET>, WriteMultiple::<Impl, OFFSET>, DeleteMultiple::<Impl, OFFSET>, ReadPropertyNames::<Impl, OFFSET>, WritePropertyNames::<Impl, OFFSET>, DeletePropertyNames::<Impl, OFFSET>, Commit::<Impl, OFFSET>, Revert::<Impl, OFFSET>, Enum::<Impl, OFFSET>, SetTimes::<Impl, OFFSET>, SetClass::<Impl, OFFSET>, Stat::<Impl, OFFSET>)
    }
}
pub trait IRootStorageImpl: Sized {
    fn SwitchToFile();
}
impl ::windows::core::RuntimeName for IRootStorage {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IRootStorage";
}
impl IRootStorageVtbl {
    pub const fn new<Impl: IRootStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRootStorageVtbl {
        unsafe extern "system" fn SwitchToFile<Impl: IRootStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfile: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SwitchToFile(&*(&pszfile as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRootStorage>, base.5, SwitchToFile::<Impl, OFFSET>)
    }
}
pub trait IStorageImpl: Sized {
    fn CreateStream();
    fn OpenStream();
    fn CreateStorage();
    fn OpenStorage();
    fn CopyTo();
    fn MoveElementTo();
    fn Commit();
    fn Revert();
    fn EnumElements();
    fn DestroyElement();
    fn RenameElement();
    fn SetElementTimes();
    fn SetClass();
    fn SetStateBits();
    fn Stat();
}
impl ::windows::core::RuntimeName for IStorage {
    const NAME: &'static str = "Windows.Win32.System.Com.StructuredStorage.IStorage";
}
impl IStorageVtbl {
    pub const fn new<Impl: IStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStorageVtbl {
        unsafe extern "system" fn CreateStream<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStream(&*(&pwcsname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), grfmode, reserved1, reserved2, ::core::mem::transmute_copy(&ppstm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenStream(&*(&pwcsname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&reserved1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), grfmode, reserved2, ::core::mem::transmute_copy(&ppstm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorage<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStorage(&*(&pwcsname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), grfmode, reserved1, reserved2, ::core::mem::transmute_copy(&ppstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStorage<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::core::RawPtr, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenStorage(&*(&pwcsname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pstgpriority as *const <IStorage as ::windows::core::Abi>::Abi as *const <IStorage as ::windows::core::DefaultType>::DefaultType), grfmode, snbexclude, reserved, ::core::mem::transmute_copy(&ppstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyTo(ciidexclude, &*(&rgiidexclude as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), snbexclude, &*(&pstgdest as *const <IStorage as ::windows::core::Abi>::Abi as *const <IStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveElementTo<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgdest: ::windows::core::RawPtr, pwcsnewname: super::super::super::Foundation::PWSTR, grfflags: STGMOVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveElementTo(
                &*(&pwcsname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pstgdest as *const <IStorage as ::windows::core::Abi>::Abi as *const <IStorage as ::windows::core::DefaultType>::DefaultType),
                &*(&pwcsnewname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                grfflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfcommitflags: STGC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit(grfcommitflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revert<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Revert() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumElements<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumElements(reserved1, &*(&reserved2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), reserved3, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyElement<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DestroyElement(&*(&pwcsname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameElement<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsoldname: super::super::super::Foundation::PWSTR, pwcsnewname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenameElement(&*(&pwcsoldname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwcsnewname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementTimes<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetElementTimes(
                &*(&pwcsname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pctime as *const <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&patime as *const <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&pmtime as *const <super::super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClass(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStateBits<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStateBits(grfstatebits, grfmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stat<Impl: IStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stat(::core::mem::transmute_copy(&pstatstg), grfstatflag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IStorage>,
            base.5,
            CreateStream::<Impl, OFFSET>,
            OpenStream::<Impl, OFFSET>,
            CreateStorage::<Impl, OFFSET>,
            OpenStorage::<Impl, OFFSET>,
            CopyTo::<Impl, OFFSET>,
            MoveElementTo::<Impl, OFFSET>,
            Commit::<Impl, OFFSET>,
            Revert::<Impl, OFFSET>,
            EnumElements::<Impl, OFFSET>,
            DestroyElement::<Impl, OFFSET>,
            RenameElement::<Impl, OFFSET>,
            SetElementTimes::<Impl, OFFSET>,
            SetClass::<Impl, OFFSET>,
            SetStateBits::<Impl, OFFSET>,
            Stat::<Impl, OFFSET>,
        )
    }
}
