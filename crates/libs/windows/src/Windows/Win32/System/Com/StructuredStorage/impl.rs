pub trait IDirectWriterLock_Impl: Sized {
    fn WaitForWriteAccess(&self, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn ReleaseWriteAccess(&self) -> ::windows::core::Result<()>;
    fn HaveWriteAccess(&self) -> ::windows::core::Result<()>;
}
impl IDirectWriterLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectWriterLock_Impl, const OFFSET: isize>() -> IDirectWriterLock_Vtbl {
        unsafe extern "system" fn WaitForWriteAccess<Identity: ::windows::core::IUnknownImpl, Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForWriteAccess(::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn ReleaseWriteAccess<Identity: ::windows::core::IUnknownImpl, Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseWriteAccess().into()
        }
        unsafe extern "system" fn HaveWriteAccess<Identity: ::windows::core::IUnknownImpl, Impl: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HaveWriteAccess().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            WaitForWriteAccess: WaitForWriteAccess::<Identity, Impl, OFFSET>,
            ReleaseWriteAccess: ReleaseWriteAccess::<Identity, Impl, OFFSET>,
            HaveWriteAccess: HaveWriteAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectWriterLock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATPROPSETSTG_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATPROPSETSTG_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>() -> IEnumSTATPROPSETSTG_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATPROPSETSTG as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSTATPROPSTG_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSTG>;
}
impl IEnumSTATPROPSTG_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>() -> IEnumSTATPROPSTG_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATPROPSTG as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATSTG_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSTATSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATSTG_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>() -> IEnumSTATSTG_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATSTG as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFillLockBytes_Impl: Sized {
    fn FillAppend(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32>;
    fn FillAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32>;
    fn SetFillSize(&self, ulsize: u64) -> ::windows::core::Result<()>;
    fn Terminate(&self, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFillLockBytes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFillLockBytes_Impl, const OFFSET: isize>() -> IFillLockBytes_Vtbl {
        unsafe extern "system" fn FillAppend<Identity: ::windows::core::IUnknownImpl, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillAppend(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillAt<Identity: ::windows::core::IUnknownImpl, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillAt(::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillSize<Identity: ::windows::core::IUnknownImpl, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFillSize(::core::mem::transmute_copy(&ulsize)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl, Impl: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&bcanceled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FillAppend: FillAppend::<Identity, Impl, OFFSET>,
            FillAt: FillAt::<Identity, Impl, OFFSET>,
            SetFillSize: SetFillSize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFillLockBytes as ::windows::core::Interface>::IID
    }
}
pub trait ILayoutStorage_Impl: Sized {
    fn LayoutScript(&self, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::Result<()>;
    fn BeginMonitor(&self) -> ::windows::core::Result<()>;
    fn EndMonitor(&self) -> ::windows::core::Result<()>;
    fn ReLayoutDocfile(&self, pwcsnewdfname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ReLayoutDocfileOnILockBytes(&self, pilockbytes: &::core::option::Option<ILockBytes>) -> ::windows::core::Result<()>;
}
impl ILayoutStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorage_Impl, const OFFSET: isize>() -> ILayoutStorage_Vtbl {
        unsafe extern "system" fn LayoutScript<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LayoutScript(::core::mem::transmute_copy(&pstoragelayout), ::core::mem::transmute_copy(&nentries), ::core::mem::transmute_copy(&glfinterleavedflag)).into()
        }
        unsafe extern "system" fn BeginMonitor<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginMonitor().into()
        }
        unsafe extern "system" fn EndMonitor<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndMonitor().into()
        }
        unsafe extern "system" fn ReLayoutDocfile<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsnewdfname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReLayoutDocfile(::core::mem::transmute(&pwcsnewdfname)).into()
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pilockbytes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReLayoutDocfileOnILockBytes(::core::mem::transmute(&pilockbytes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LayoutScript: LayoutScript::<Identity, Impl, OFFSET>,
            BeginMonitor: BeginMonitor::<Identity, Impl, OFFSET>,
            EndMonitor: EndMonitor::<Identity, Impl, OFFSET>,
            ReLayoutDocfile: ReLayoutDocfile::<Identity, Impl, OFFSET>,
            ReLayoutDocfileOnILockBytes: ReLayoutDocfileOnILockBytes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayoutStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ILockBytes_Impl: Sized {
    fn ReadAt(&self, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()>;
    fn WriteAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32>;
    fn Flush(&self) -> ::windows::core::Result<()>;
    fn SetSize(&self, cb: u64) -> ::windows::core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ILockBytes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>() -> ILockBytes_Vtbl {
        unsafe extern "system" fn ReadAt<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadAt(::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn WriteAt<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteAt(::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn LockRegion<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockRegion(::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn UnlockRegion<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockRegion(::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn Stat<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stat(::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute_copy(&grfstatflag)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReadAt: ReadAt::<Identity, Impl, OFFSET>,
            WriteAt: WriteAt::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            LockRegion: LockRegion::<Identity, Impl, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockBytes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStorage_Impl: Sized + super::IPersist_Impl {
    fn IsDirty(&self) -> ::windows::core::Result<()>;
    fn InitNew(&self, pstg: &::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn Load(&self, pstg: &::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn Save(&self, pstgsave: &::core::option::Option<IStorage>, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveCompleted(&self, pstgnew: &::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn HandsOffStorage(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const OFFSET: isize>() -> IPersistStorage_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDirty().into()
        }
        unsafe extern "system" fn InitNew<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitNew(::core::mem::transmute(&pstg)).into()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(::core::mem::transmute(&pstg)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstgsave: ::windows::core::RawPtr, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save(::core::mem::transmute(&pstgsave), ::core::mem::transmute_copy(&fsameasload)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstgnew: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveCompleted(::core::mem::transmute(&pstgnew)).into()
        }
        unsafe extern "system" fn HandsOffStorage<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandsOffStorage().into()
        }
        Self {
            base: super::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistStorage as ::windows::core::Interface>::IID || iid == &<super::IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IPropertyBag_Impl: Sized {
    fn Read(&self, pszpropname: &::windows::core::PCWSTR, pvar: *mut super::VARIANT, perrorlog: &::core::option::Option<super::IErrorLog>) -> ::windows::core::Result<()>;
    fn Write(&self, pszpropname: &::windows::core::PCWSTR, pvar: *const super::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag_Impl, const OFFSET: isize>() -> IPropertyBag_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pvar: *mut super::VARIANT, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&pvar), ::core::mem::transmute(&perrorlog)).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Write(::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&pvar)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, Impl, OFFSET>, Write: Write::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IPropertyBag2_Impl: Sized {
    fn Read(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: &::core::option::Option<super::IErrorLog>, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn CountProperties(&self) -> ::windows::core::Result<u32>;
    fn GetPropertyInfo(&self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::Result<()>;
    fn LoadObject(&self, pstrname: &::windows::core::PCWSTR, dwhint: u32, punkobject: &::core::option::Option<::windows::core::IUnknown>, perrlog: &::core::option::Option<super::IErrorLog>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IPropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2_Impl, const OFFSET: isize>() -> IPropertyBag2_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::windows::core::RawPtr, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute(&perrlog), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&phrerror)).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn CountProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CountProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pcproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyInfo(::core::mem::transmute_copy(&iproperty), ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute_copy(&pcproperties)).into()
        }
        unsafe extern "system" fn LoadObject<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadObject(::core::mem::transmute(&pstrname), ::core::mem::transmute_copy(&dwhint), ::core::mem::transmute(&punkobject), ::core::mem::transmute(&perrlog)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            CountProperties: CountProperties::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            LoadObject: LoadObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyBag2 as ::windows::core::Interface>::IID
    }
}
pub trait IPropertySetStorage_Impl: Sized {
    fn Create(&self, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32) -> ::windows::core::Result<IPropertyStorage>;
    fn Open(&self, rfmtid: *const ::windows::core::GUID, grfmode: u32) -> ::windows::core::Result<IPropertyStorage>;
    fn Delete(&self, rfmtid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG>;
}
impl IPropertySetStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetStorage_Impl, const OFFSET: isize>() -> IPropertySetStorage_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&rfmtid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&grfmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&rfmtid), ::core::mem::transmute_copy(&grfmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&rfmtid)).into()
        }
        unsafe extern "system" fn Enum<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySetStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPropertyStorage_Impl: Sized {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::Result<()>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> ::windows::core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()>;
    fn Revert(&self) -> ::windows::core::Result<()>;
    fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Stat(&self) -> ::windows::core::Result<STATPROPSETSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPropertyStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>() -> IPropertyStorage_Vtbl {
        unsafe extern "system" fn ReadMultiple<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar)).into()
        }
        unsafe extern "system" fn WriteMultiple<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar), ::core::mem::transmute_copy(&propidnamefirst)).into()
        }
        unsafe extern "system" fn DeleteMultiple<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec)).into()
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadPropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn WritePropertyNames<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WritePropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Revert().into()
        }
        unsafe extern "system" fn Enum<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimes<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTimes(::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClass(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn Stat<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Stat() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatpsstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, Impl, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, Impl, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, Impl, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, Impl, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, Impl, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            SetTimes: SetTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStorage as ::windows::core::Interface>::IID
    }
}
pub trait IRootStorage_Impl: Sized {
    fn SwitchToFile(&self, pszfile: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl IRootStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRootStorage_Impl, const OFFSET: isize>() -> IRootStorage_Vtbl {
        unsafe extern "system" fn SwitchToFile<Identity: ::windows::core::IUnknownImpl, Impl: IRootStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SwitchToFile(::core::mem::transmute(&pszfile)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SwitchToFile: SwitchToFile::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRootStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorage_Impl: Sized {
    fn CreateStream(&self, pwcsname: &::windows::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<super::IStream>;
    fn OpenStream(&self, pwcsname: &::windows::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut ::core::option::Option<super::IStream>) -> ::windows::core::Result<()>;
    fn CreateStorage(&self, pwcsname: &::windows::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<IStorage>;
    fn OpenStorage(&self, pwcsname: &::windows::core::PCWSTR, pstgpriority: &::core::option::Option<IStorage>, grfmode: STGM, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage>;
    fn CopyTo(&self, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: &::core::option::Option<IStorage>) -> ::windows::core::Result<()>;
    fn MoveElementTo(&self, pwcsname: &::windows::core::PCWSTR, pstgdest: &::core::option::Option<IStorage>, pwcsnewname: &::windows::core::PCWSTR, grfflags: STGMOVE) -> ::windows::core::Result<()>;
    fn Commit(&self, grfcommitflags: STGC) -> ::windows::core::Result<()>;
    fn Revert(&self) -> ::windows::core::Result<()>;
    fn EnumElements(&self, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::core::option::Option<IEnumSTATSTG>) -> ::windows::core::Result<()>;
    fn DestroyElement(&self, pwcsname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RenameElement(&self, pwcsoldname: &::windows::core::PCWSTR, pwcsnewname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetElementTimes(&self, pwcsname: &::windows::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> ::windows::core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>() -> IStorage_Vtbl {
        unsafe extern "system" fn CreateStream<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStream(::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenStream(::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved2), ::core::mem::transmute_copy(&ppstm)).into()
        }
        unsafe extern "system" fn CreateStorage<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStorage(::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStorage<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pstgpriority: ::windows::core::RawPtr, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenStorage(::core::mem::transmute(&pwcsname), ::core::mem::transmute(&pstgpriority), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&snbexclude), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTo(::core::mem::transmute_copy(&ciidexclude), ::core::mem::transmute_copy(&rgiidexclude), ::core::mem::transmute_copy(&snbexclude), ::core::mem::transmute(&pstgdest)).into()
        }
        unsafe extern "system" fn MoveElementTo<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pstgdest: ::windows::core::RawPtr, pwcsnewname: ::windows::core::PCWSTR, grfflags: STGMOVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveElementTo(::core::mem::transmute(&pwcsname), ::core::mem::transmute(&pstgdest), ::core::mem::transmute(&pwcsnewname), ::core::mem::transmute_copy(&grfflags)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: STGC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Revert().into()
        }
        unsafe extern "system" fn EnumElements<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumElements(::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2), ::core::mem::transmute_copy(&reserved3), ::core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn DestroyElement<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyElement(::core::mem::transmute(&pwcsname)).into()
        }
        unsafe extern "system" fn RenameElement<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsoldname: ::windows::core::PCWSTR, pwcsnewname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenameElement(::core::mem::transmute(&pwcsoldname), ::core::mem::transmute(&pwcsnewname)).into()
        }
        unsafe extern "system" fn SetElementTimes<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetElementTimes(::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClass(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn SetStateBits<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStateBits(::core::mem::transmute_copy(&grfstatebits), ::core::mem::transmute_copy(&grfmask)).into()
        }
        unsafe extern "system" fn Stat<Identity: ::windows::core::IUnknownImpl, Impl: IStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stat(::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute_copy(&grfstatflag)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            OpenStream: OpenStream::<Identity, Impl, OFFSET>,
            CreateStorage: CreateStorage::<Identity, Impl, OFFSET>,
            OpenStorage: OpenStorage::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            MoveElementTo: MoveElementTo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            EnumElements: EnumElements::<Identity, Impl, OFFSET>,
            DestroyElement: DestroyElement::<Identity, Impl, OFFSET>,
            RenameElement: RenameElement::<Identity, Impl, OFFSET>,
            SetElementTimes: SetElementTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            SetStateBits: SetStateBits::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorage as ::windows::core::Interface>::IID
    }
}
