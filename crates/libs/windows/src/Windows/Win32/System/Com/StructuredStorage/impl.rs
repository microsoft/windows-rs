pub trait IDirectWriterLockImpl: Sized {
    fn WaitForWriteAccess();
    fn ReleaseWriteAccess();
    fn HaveWriteAccess();
}
impl IDirectWriterLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectWriterLockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectWriterLockVtbl {
        unsafe extern "system" fn WaitForWriteAccess<Impl: IDirectWriterLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseWriteAccess<Impl: IDirectWriterLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HaveWriteAccess<Impl: IDirectWriterLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IEnumSTATPROPSETSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATPROPSETSTGVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSETSTGImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSTATPROPSETSTGVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATPROPSETSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IEnumSTATPROPSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATPROPSTGVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATPROPSTGImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSTATPROPSTGVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATPROPSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATPROPSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATPROPSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATPROPSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IEnumSTATSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumSTATSTGVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATSTGImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSTATSTGVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATSTGImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IFillLockBytesImpl: Sized {
    fn FillAppend();
    fn FillAt();
    fn SetFillSize();
    fn Terminate();
}
#[cfg(feature = "Win32_Foundation")]
impl IFillLockBytesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFillLockBytesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFillLockBytesVtbl {
        unsafe extern "system" fn FillAppend<Impl: IFillLockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillAt<Impl: IFillLockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFillSize<Impl: IFillLockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminate<Impl: IFillLockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ILayoutStorageImpl: Sized {
    fn LayoutScript();
    fn BeginMonitor();
    fn EndMonitor();
    fn ReLayoutDocfile();
    fn ReLayoutDocfileOnILockBytes();
}
#[cfg(feature = "Win32_Foundation")]
impl ILayoutStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutStorageVtbl {
        unsafe extern "system" fn LayoutScript<Impl: ILayoutStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginMonitor<Impl: ILayoutStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndMonitor<Impl: ILayoutStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReLayoutDocfile<Impl: ILayoutStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsnewdfname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Impl: ILayoutStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pilockbytes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ILockBytesImpl: Sized {
    fn ReadAt();
    fn WriteAt();
    fn Flush();
    fn SetSize();
    fn LockRegion();
    fn UnlockRegion();
    fn Stat();
}
#[cfg(feature = "Win32_Foundation")]
impl ILockBytesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockBytesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockBytesVtbl {
        unsafe extern "system" fn ReadAt<Impl: ILockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteAt<Impl: ILockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: ILockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: ILockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockRegion<Impl: ILockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockRegion<Impl: ILockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stat<Impl: ILockBytesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IPersistStorageImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn InitNew();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn HandsOffStorage();
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistStorageVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitNew<Impl: IPersistStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IPersistStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IPersistStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstgsave: ::windows::core::RawPtr, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstgnew: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandsOffStorage<Impl: IPersistStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IPersistVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IPropertyBagImpl: Sized {
    fn Read();
    fn Write();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IPropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBagImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyBagVtbl {
        unsafe extern "system" fn Read<Impl: IPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *mut super::VARIANT, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Read: Read::<Impl, IMPL_OFFSET>, Write: Write::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IPropertyBag2Impl: Sized {
    fn Read();
    fn Write();
    fn CountProperties();
    fn GetPropertyInfo();
    fn LoadObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IPropertyBag2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyBag2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyBag2Vtbl {
        unsafe extern "system" fn Read<Impl: IPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::windows::core::RawPtr, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CountProperties<Impl: IPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyInfo<Impl: IPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadObject<Impl: IPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: super::super::super::Foundation::PWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IPropertySetStorageImpl: Sized {
    fn Create();
    fn Open();
    fn Delete();
    fn Enum();
}
impl IPropertySetStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySetStorageVtbl {
        unsafe extern "system" fn Create<Impl: IPropertySetStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IPropertySetStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPropertySetStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enum<Impl: IPropertySetStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IPropertyStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStorageVtbl {
        unsafe extern "system" fn ReadMultiple<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteMultiple<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteMultiple<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadPropertyNames<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WritePropertyNames<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeletePropertyNames<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Revert<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enum<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimes<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClass<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stat<Impl: IPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRootStorageImpl: Sized {
    fn SwitchToFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IRootStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRootStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRootStorageVtbl {
        unsafe extern "system" fn SwitchToFile<Impl: IRootStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SwitchToFile: SwitchToFile::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRootStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageVtbl {
        unsafe extern "system" fn CreateStream<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenStream<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStorage<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenStorage<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::core::RawPtr, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTo<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveElementTo<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgdest: ::windows::core::RawPtr, pwcsnewname: super::super::super::Foundation::PWSTR, grfflags: STGMOVE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: STGC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Revert<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumElements<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyElement<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenameElement<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsoldname: super::super::super::Foundation::PWSTR, pwcsnewname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetElementTimes<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClass<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStateBits<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stat<Impl: IStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
