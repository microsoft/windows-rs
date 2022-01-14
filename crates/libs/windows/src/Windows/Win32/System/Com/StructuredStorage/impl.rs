pub trait IDirectWriterLock_Impl: Sized {
    fn WaitForWriteAccess(&mut self, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn ReleaseWriteAccess(&mut self) -> ::windows::core::Result<()>;
    fn HaveWriteAccess(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectWriterLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectWriterLock_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectWriterLock_Vtbl {
        unsafe extern "system" fn WaitForWriteAccess<Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForWriteAccess(::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn ReleaseWriteAccess<Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseWriteAccess().into()
        }
        unsafe extern "system" fn HaveWriteAccess<Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HaveWriteAccess().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            WaitForWriteAccess: WaitForWriteAccess::<Impl, IMPL_OFFSET>,
            ReleaseWriteAccess: ReleaseWriteAccess::<Impl, IMPL_OFFSET>,
            HaveWriteAccess: HaveWriteAccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectWriterLock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATPROPSETSTG_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&mut self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSTATPROPSETSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATPROPSETSTG_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSETSTG_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSTATPROPSETSTG_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATPROPSETSTG as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATPROPSTG_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&mut self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSTATPROPSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATPROPSTG_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSTG_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSTATPROPSTG_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATPROPSTG as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATSTG_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSTATSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATSTG_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATSTG_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSTATSTG_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATSTG as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFillLockBytes_Impl: Sized {
    fn FillAppend(&mut self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32>;
    fn FillAt(&mut self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32>;
    fn SetFillSize(&mut self, ulsize: u64) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFillLockBytes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFillLockBytes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFillLockBytes_Vtbl {
        unsafe extern "system" fn FillAppend<Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillAppend(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillAt<Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillAt(::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillSize<Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillSize(::core::mem::transmute_copy(&ulsize)).into()
        }
        unsafe extern "system" fn Terminate<Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&bcanceled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FillAppend: FillAppend::<Impl, IMPL_OFFSET>,
            FillAt: FillAt::<Impl, IMPL_OFFSET>,
            SetFillSize: SetFillSize::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFillLockBytes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ILayoutStorage_Impl: Sized {
    fn LayoutScript(&mut self, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::Result<()>;
    fn BeginMonitor(&mut self) -> ::windows::core::Result<()>;
    fn EndMonitor(&mut self) -> ::windows::core::Result<()>;
    fn ReLayoutDocfile(&mut self, pwcsnewdfname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ReLayoutDocfileOnILockBytes(&mut self, pilockbytes: ::core::option::Option<ILockBytes>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ILayoutStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutStorage_Vtbl {
        unsafe extern "system" fn LayoutScript<Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LayoutScript(::core::mem::transmute_copy(&pstoragelayout), ::core::mem::transmute_copy(&nentries), ::core::mem::transmute_copy(&glfinterleavedflag)).into()
        }
        unsafe extern "system" fn BeginMonitor<Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginMonitor().into()
        }
        unsafe extern "system" fn EndMonitor<Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndMonitor().into()
        }
        unsafe extern "system" fn ReLayoutDocfile<Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsnewdfname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReLayoutDocfile(::core::mem::transmute_copy(&pwcsnewdfname)).into()
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pilockbytes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReLayoutDocfileOnILockBytes(::core::mem::transmute(&pilockbytes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LayoutScript: LayoutScript::<Impl, IMPL_OFFSET>,
            BeginMonitor: BeginMonitor::<Impl, IMPL_OFFSET>,
            EndMonitor: EndMonitor::<Impl, IMPL_OFFSET>,
            ReLayoutDocfile: ReLayoutDocfile::<Impl, IMPL_OFFSET>,
            ReLayoutDocfileOnILockBytes: ReLayoutDocfileOnILockBytes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayoutStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ILockBytes_Impl: Sized {
    fn ReadAt(&mut self, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()>;
    fn WriteAt(&mut self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
    fn SetSize(&mut self, cb: u64) -> ::windows::core::Result<()>;
    fn LockRegion(&mut self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()>;
    fn UnlockRegion(&mut self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()>;
    fn Stat(&mut self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ILockBytes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockBytes_Vtbl {
        unsafe extern "system" fn ReadAt<Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadAt(::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn WriteAt<Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteAt(::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn SetSize<Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn LockRegion<Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockRegion(::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn UnlockRegion<Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockRegion(::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn Stat<Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stat(::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute_copy(&grfstatflag)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReadAt: ReadAt::<Impl, IMPL_OFFSET>,
            WriteAt: WriteAt::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            LockRegion: LockRegion::<Impl, IMPL_OFFSET>,
            UnlockRegion: UnlockRegion::<Impl, IMPL_OFFSET>,
            Stat: Stat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockBytes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStorage_Impl: Sized + super::IPersist_Impl {
    fn IsDirty(&mut self) -> ::windows::core::Result<()>;
    fn InitNew(&mut self, pstg: ::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn Load(&mut self, pstg: ::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn Save(&mut self, pstgsave: ::core::option::Option<IStorage>, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveCompleted(&mut self, pstgnew: ::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn HandsOffStorage(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistStorage_Vtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDirty().into()
        }
        unsafe extern "system" fn InitNew<Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitNew(::core::mem::transmute(&pstg)).into()
        }
        unsafe extern "system" fn Load<Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute(&pstg)).into()
        }
        unsafe extern "system" fn Save<Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstgsave: ::windows::core::RawPtr, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute(&pstgsave), ::core::mem::transmute_copy(&fsameasload)).into()
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstgnew: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveCompleted(::core::mem::transmute(&pstgnew)).into()
        }
        unsafe extern "system" fn HandsOffStorage<Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandsOffStorage().into()
        }
        Self {
            base: super::IPersist_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsDirty: IsDirty::<Impl, IMPL_OFFSET>,
            InitNew: InitNew::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            SaveCompleted: SaveCompleted::<Impl, IMPL_OFFSET>,
            HandsOffStorage: HandsOffStorage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IPropertyBag_Impl: Sized {
    fn Read(&mut self, pszpropname: super::super::super::Foundation::PWSTR, pvar: *mut super::VARIANT, perrorlog: ::core::option::Option<super::IErrorLog>) -> ::windows::core::Result<()>;
    fn Write(&mut self, pszpropname: super::super::super::Foundation::PWSTR, pvar: *const super::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyBag_Vtbl {
        unsafe extern "system" fn Read<Impl: IPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *mut super::VARIANT, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pszpropname), ::core::mem::transmute_copy(&pvar), ::core::mem::transmute(&perrorlog)).into()
        }
        unsafe extern "system" fn Write<Impl: IPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&pszpropname), ::core::mem::transmute_copy(&pvar)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Read: Read::<Impl, IMPL_OFFSET>, Write: Write::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IPropertyBag2_Impl: Sized {
    fn Read(&mut self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::core::option::Option<super::IErrorLog>, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Write(&mut self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn CountProperties(&mut self) -> ::windows::core::Result<u32>;
    fn GetPropertyInfo(&mut self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::Result<()>;
    fn LoadObject(&mut self, pstrname: super::super::super::Foundation::PWSTR, dwhint: u32, punkobject: ::core::option::Option<::windows::core::IUnknown>, perrlog: ::core::option::Option<super::IErrorLog>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IPropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyBag2_Vtbl {
        unsafe extern "system" fn Read<Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::windows::core::RawPtr, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute(&perrlog), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&phrerror)).into()
        }
        unsafe extern "system" fn Write<Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn CountProperties<Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pcproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyInfo(::core::mem::transmute_copy(&iproperty), ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute_copy(&pcproperties)).into()
        }
        unsafe extern "system" fn LoadObject<Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: super::super::super::Foundation::PWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadObject(::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&dwhint), ::core::mem::transmute(&punkobject), ::core::mem::transmute(&perrlog)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Read: Read::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            CountProperties: CountProperties::<Impl, IMPL_OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Impl, IMPL_OFFSET>,
            LoadObject: LoadObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyBag2 as ::windows::core::Interface>::IID
    }
}
pub trait IPropertySetStorage_Impl: Sized {
    fn Create(&mut self, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32) -> ::windows::core::Result<IPropertyStorage>;
    fn Open(&mut self, rfmtid: *const ::windows::core::GUID, grfmode: u32) -> ::windows::core::Result<IPropertyStorage>;
    fn Delete(&mut self, rfmtid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Enum(&mut self) -> ::windows::core::Result<IEnumSTATPROPSETSTG>;
}
impl IPropertySetStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetStorage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySetStorage_Vtbl {
        unsafe extern "system" fn Create<Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&rfmtid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&grfmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&rfmtid), ::core::mem::transmute_copy(&grfmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&rfmtid)).into()
        }
        unsafe extern "system" fn Enum<Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Enum: Enum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySetStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPropertyStorage_Impl: Sized {
    fn ReadMultiple(&mut self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::Result<()>;
    fn WriteMultiple(&mut self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()>;
    fn DeleteMultiple(&mut self, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::Result<()>;
    fn ReadPropertyNames(&mut self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WritePropertyNames(&mut self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeletePropertyNames(&mut self, cpropid: u32, rgpropid: *const u32) -> ::windows::core::Result<()>;
    fn Commit(&mut self, grfcommitflags: u32) -> ::windows::core::Result<()>;
    fn Revert(&mut self) -> ::windows::core::Result<()>;
    fn Enum(&mut self) -> ::windows::core::Result<IEnumSTATPROPSTG>;
    fn SetTimes(&mut self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetClass(&mut self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Stat(&mut self) -> ::windows::core::Result<STATPROPSETSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPropertyStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStorage_Vtbl {
        unsafe extern "system" fn ReadMultiple<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar)).into()
        }
        unsafe extern "system" fn WriteMultiple<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar), ::core::mem::transmute_copy(&propidnamefirst)).into()
        }
        unsafe extern "system" fn DeleteMultiple<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec)).into()
        }
        unsafe extern "system" fn ReadPropertyNames<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadPropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn WritePropertyNames<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WritePropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn DeletePropertyNames<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid)).into()
        }
        unsafe extern "system" fn Commit<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Revert().into()
        }
        unsafe extern "system" fn Enum<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimes<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimes(::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClass(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn Stat<Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stat() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatpsstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReadMultiple: ReadMultiple::<Impl, IMPL_OFFSET>,
            WriteMultiple: WriteMultiple::<Impl, IMPL_OFFSET>,
            DeleteMultiple: DeleteMultiple::<Impl, IMPL_OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Impl, IMPL_OFFSET>,
            WritePropertyNames: WritePropertyNames::<Impl, IMPL_OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Revert: Revert::<Impl, IMPL_OFFSET>,
            Enum: Enum::<Impl, IMPL_OFFSET>,
            SetTimes: SetTimes::<Impl, IMPL_OFFSET>,
            SetClass: SetClass::<Impl, IMPL_OFFSET>,
            Stat: Stat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRootStorage_Impl: Sized {
    fn SwitchToFile(&mut self, pszfile: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRootStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRootStorage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRootStorage_Vtbl {
        unsafe extern "system" fn SwitchToFile<Impl: IRootStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchToFile(::core::mem::transmute_copy(&pszfile)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SwitchToFile: SwitchToFile::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRootStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorage_Impl: Sized {
    fn CreateStream(&mut self, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<super::IStream>;
    fn OpenStream(&mut self, pwcsname: super::super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut ::core::option::Option<super::IStream>) -> ::windows::core::Result<()>;
    fn CreateStorage(&mut self, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<IStorage>;
    fn OpenStorage(&mut self, pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::core::option::Option<IStorage>, grfmode: STGM, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage>;
    fn CopyTo(&mut self, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: ::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn MoveElementTo(&mut self, pwcsname: super::super::super::Foundation::PWSTR, pstgdest: ::core::option::Option<IStorage>, pwcsnewname: super::super::super::Foundation::PWSTR, grfflags: STGMOVE) -> ::windows::core::Result<()>;
    fn Commit(&mut self, grfcommitflags: STGC) -> ::windows::core::Result<()>;
    fn Revert(&mut self) -> ::windows::core::Result<()>;
    fn EnumElements(&mut self, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::core::option::Option<IEnumSTATSTG>) -> ::windows::core::Result<()>;
    fn DestroyElement(&mut self, pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RenameElement(&mut self, pwcsoldname: super::super::super::Foundation::PWSTR, pwcsnewname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetElementTimes(&mut self, pwcsname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetClass(&mut self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetStateBits(&mut self, grfstatebits: u32, grfmask: u32) -> ::windows::core::Result<()>;
    fn Stat(&mut self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorage_Vtbl {
        unsafe extern "system" fn CreateStream<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStream(::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenStream(::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved2), ::core::mem::transmute_copy(&ppstm)).into()
        }
        unsafe extern "system" fn CreateStorage<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorage(::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStorage<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::core::RawPtr, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenStorage(::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute(&pstgpriority), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&snbexclude), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTo(::core::mem::transmute_copy(&ciidexclude), ::core::mem::transmute_copy(&rgiidexclude), ::core::mem::transmute_copy(&snbexclude), ::core::mem::transmute(&pstgdest)).into()
        }
        unsafe extern "system" fn MoveElementTo<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgdest: ::windows::core::RawPtr, pwcsnewname: super::super::super::Foundation::PWSTR, grfflags: STGMOVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveElementTo(::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute(&pstgdest), ::core::mem::transmute_copy(&pwcsnewname), ::core::mem::transmute_copy(&grfflags)).into()
        }
        unsafe extern "system" fn Commit<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: STGC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Revert().into()
        }
        unsafe extern "system" fn EnumElements<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumElements(::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2), ::core::mem::transmute_copy(&reserved3), ::core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn DestroyElement<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyElement(::core::mem::transmute_copy(&pwcsname)).into()
        }
        unsafe extern "system" fn RenameElement<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsoldname: super::super::super::Foundation::PWSTR, pwcsnewname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameElement(::core::mem::transmute_copy(&pwcsoldname), ::core::mem::transmute_copy(&pwcsnewname)).into()
        }
        unsafe extern "system" fn SetElementTimes<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElementTimes(::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClass(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn SetStateBits<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStateBits(::core::mem::transmute_copy(&grfstatebits), ::core::mem::transmute_copy(&grfmask)).into()
        }
        unsafe extern "system" fn Stat<Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stat(::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute_copy(&grfstatflag)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateStream: CreateStream::<Impl, IMPL_OFFSET>,
            OpenStream: OpenStream::<Impl, IMPL_OFFSET>,
            CreateStorage: CreateStorage::<Impl, IMPL_OFFSET>,
            OpenStorage: OpenStorage::<Impl, IMPL_OFFSET>,
            CopyTo: CopyTo::<Impl, IMPL_OFFSET>,
            MoveElementTo: MoveElementTo::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Revert: Revert::<Impl, IMPL_OFFSET>,
            EnumElements: EnumElements::<Impl, IMPL_OFFSET>,
            DestroyElement: DestroyElement::<Impl, IMPL_OFFSET>,
            RenameElement: RenameElement::<Impl, IMPL_OFFSET>,
            SetElementTimes: SetElementTimes::<Impl, IMPL_OFFSET>,
            SetClass: SetClass::<Impl, IMPL_OFFSET>,
            SetStateBits: SetStateBits::<Impl, IMPL_OFFSET>,
            Stat: Stat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorage as ::windows::core::Interface>::IID
    }
}
