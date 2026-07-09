#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CIMTYPE(pub i32);
pub type CIMTYPE_ENUMERATION = i32;
pub const CIM_BOOLEAN: CIMTYPE_ENUMERATION = 11;
pub const CIM_CHAR16: CIMTYPE_ENUMERATION = 103;
pub const CIM_DATETIME: CIMTYPE_ENUMERATION = 101;
pub const CIM_EMPTY: CIMTYPE_ENUMERATION = 0;
pub const CIM_FLAG_ARRAY: CIMTYPE_ENUMERATION = 8192;
pub const CIM_ILLEGAL: CIMTYPE_ENUMERATION = 4095;
pub const CIM_OBJECT: CIMTYPE_ENUMERATION = 13;
pub const CIM_REAL32: CIMTYPE_ENUMERATION = 4;
pub const CIM_REAL64: CIMTYPE_ENUMERATION = 5;
pub const CIM_REFERENCE: CIMTYPE_ENUMERATION = 102;
pub const CIM_SINT16: CIMTYPE_ENUMERATION = 2;
pub const CIM_SINT32: CIMTYPE_ENUMERATION = 3;
pub const CIM_SINT64: CIMTYPE_ENUMERATION = 20;
pub const CIM_SINT8: CIMTYPE_ENUMERATION = 16;
pub const CIM_STRING: CIMTYPE_ENUMERATION = 8;
pub const CIM_UINT16: CIMTYPE_ENUMERATION = 18;
pub const CIM_UINT32: CIMTYPE_ENUMERATION = 19;
pub const CIM_UINT64: CIMTYPE_ENUMERATION = 21;
pub const CIM_UINT8: CIMTYPE_ENUMERATION = 17;
windows_core::imp::define_interface!(IEnumWbemClassObject, IEnumWbemClassObject_Vtbl, 0x027947e1_d731_11ce_a357_000000000001);
windows_core::imp::interface_hierarchy!(IEnumWbemClassObject, windows_core::IUnknown);
impl IEnumWbemClassObject {
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Next(&self, ltimeout: i32, ucount: u32, apobjects: *mut Option<IWbemClassObject>, pureturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ltimeout, ucount, core::mem::transmute(apobjects), pureturned as _) }
    }
    pub unsafe fn NextAsync<P1>(&self, ucount: u32, psink: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).NextAsync)(windows_core::Interface::as_raw(self), ucount, psink.param().abi()) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Skip(&self, ltimeout: i32, ncount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ltimeout, ncount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWbemClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
}
pub trait IEnumWbemClassObject_Impl: windows_core::IUnknownImpl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self, ltimeout: i32, ucount: u32, apobjects: windows_core::OutRef<IWbemClassObject>, pureturned: *mut u32) -> windows_core::Result<()>;
    fn NextAsync(&self, ucount: u32, psink: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWbemClassObject>;
    fn Skip(&self, ltimeout: i32, ncount: u32) -> windows_core::Result<()>;
}
impl IEnumWbemClassObject_Vtbl {
    pub const fn new<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ucount), core::mem::transmute_copy(&apobjects), core::mem::transmute_copy(&pureturned)).into()
            }
        }
        unsafe extern "system" fn NextAsync<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucount: u32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::NextAsync(this, core::mem::transmute_copy(&ucount), core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWbemClassObject_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ncount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::Skip(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ncount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            NextAsync: NextAsync::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWbemClassObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWbemClassObject {}
windows_core::imp::define_interface!(IMofCompiler, IMofCompiler_Vtbl, 0x6daf974e_2e37_11d2_aec9_00c04fb68820);
windows_core::imp::interface_hierarchy!(IMofCompiler, windows_core::IUnknown);
impl IMofCompiler {
    pub unsafe fn CompileFile<P0, P1, P2, P3, P4>(&self, filename: P0, serverandnamespace: P1, user: P2, authority: P3, password: P4, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompileFile)(windows_core::Interface::as_raw(self), filename.param().abi(), serverandnamespace.param().abi(), user.param().abi(), authority.param().abi(), password.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo as _) }
    }
    pub unsafe fn CompileBuffer<P2, P3, P4, P5>(&self, pbuffer: &[u8], serverandnamespace: P2, user: P3, authority: P4, password: P5, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompileBuffer)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), serverandnamespace.param().abi(), user.param().abi(), authority.param().abi(), password.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo as _) }
    }
    pub unsafe fn CreateBMOF<P0, P1, P2>(&self, textfilename: P0, bmoffilename: P1, serverandnamespace: P2, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateBMOF)(windows_core::Interface::as_raw(self), textfilename.param().abi(), bmoffilename.param().abi(), serverandnamespace.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMofCompiler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompileFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
    pub CompileBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
    pub CreateBMOF: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
}
pub trait IMofCompiler_Impl: windows_core::IUnknownImpl {
    fn CompileFile(&self, filename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CompileBuffer(&self, buffsize: i32, pbuffer: *const u8, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CreateBMOF(&self, textfilename: &windows_core::PCWSTR, bmoffilename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
}
impl IMofCompiler_Vtbl {
    pub const fn new<Identity: IMofCompiler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompileFile<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMofCompiler_Impl::CompileFile(this, core::mem::transmute(&filename), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn CompileBuffer<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMofCompiler_Impl::CompileBuffer(this, core::mem::transmute_copy(&buffsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn CreateBMOF<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textfilename: windows_core::PCWSTR, bmoffilename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMofCompiler_Impl::CreateBMOF(this, core::mem::transmute(&textfilename), core::mem::transmute(&bmoffilename), core::mem::transmute(&serverandnamespace), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompileFile: CompileFile::<Identity, OFFSET>,
            CompileBuffer: CompileBuffer::<Identity, OFFSET>,
            CreateBMOF: CreateBMOF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMofCompiler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMofCompiler {}
windows_core::imp::define_interface!(IUnsecuredApartment, IUnsecuredApartment_Vtbl, 0x1cfaba8c_1523_11d1_ad79_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IUnsecuredApartment, windows_core::IUnknown);
impl IUnsecuredApartment {
    pub unsafe fn CreateObjectStub<P0>(&self, pobject: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateObjectStub)(windows_core::Interface::as_raw(self), pobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsecuredApartment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateObjectStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUnsecuredApartment_Impl: windows_core::IUnknownImpl {
    fn CreateObjectStub(&self, pobject: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl IUnsecuredApartment_Vtbl {
    pub const fn new<Identity: IUnsecuredApartment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateObjectStub<Identity: IUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUnsecuredApartment_Impl::CreateObjectStub(this, core::mem::transmute_copy(&pobject)) {
                    Ok(ok__) => {
                        ppstub.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObjectStub: CreateObjectStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUnsecuredApartment {}
windows_core::imp::define_interface!(IWbemBackupRestore, IWbemBackupRestore_Vtbl, 0xc49e32c7_bc8b_11d2_85d4_00105a1f8304);
windows_core::imp::interface_hierarchy!(IWbemBackupRestore, windows_core::IUnknown);
impl IWbemBackupRestore {
    pub unsafe fn Backup<P0>(&self, strbackuptofile: P0, lflags: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), strbackuptofile.param().abi(), lflags) }
    }
    pub unsafe fn Restore<P0>(&self, strrestorefromfile: P0, lflags: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self), strrestorefromfile.param().abi(), lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Backup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
}
pub trait IWbemBackupRestore_Impl: windows_core::IUnknownImpl {
    fn Backup(&self, strbackuptofile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn Restore(&self, strrestorefromfile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemBackupRestore_Vtbl {
    pub const fn new<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Backup<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbackuptofile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestore_Impl::Backup(this, core::mem::transmute(&strbackuptofile), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrestorefromfile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestore_Impl::Restore(this, core::mem::transmute(&strrestorefromfile), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Backup: Backup::<Identity, OFFSET>, Restore: Restore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemBackupRestore {}
windows_core::imp::define_interface!(IWbemBackupRestoreEx, IWbemBackupRestoreEx_Vtbl, 0xa359dec5_e813_4834_8a2a_ba7f1d777d76);
impl core::ops::Deref for IWbemBackupRestoreEx {
    type Target = IWbemBackupRestore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemBackupRestoreEx, windows_core::IUnknown, IWbemBackupRestore);
impl IWbemBackupRestoreEx {
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestoreEx_Vtbl {
    pub base__: IWbemBackupRestore_Vtbl,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemBackupRestoreEx_Impl: IWbemBackupRestore_Impl {
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl IWbemBackupRestoreEx_Vtbl {
    pub const fn new<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pause<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestoreEx_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestoreEx_Impl::Resume(this).into()
            }
        }
        Self { base__: IWbemBackupRestore_Vtbl::new::<Identity, OFFSET>(), Pause: Pause::<Identity, OFFSET>, Resume: Resume::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestoreEx as windows_core::Interface>::IID || iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemBackupRestoreEx {}
windows_core::imp::define_interface!(IWbemCallResult, IWbemCallResult_Vtbl, 0x44aca675_e8fc_11d0_a07c_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemCallResult, windows_core::IUnknown);
impl IWbemCallResult {
    pub unsafe fn GetResultObject(&self, ltimeout: i32) -> windows_core::Result<IWbemClassObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResultObject)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetResultString(&self, ltimeout: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResultString)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetResultServices(&self, ltimeout: i32) -> windows_core::Result<IWbemServices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResultServices)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCallStatus(&self, ltimeout: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCallStatus)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemCallResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResultObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResultString: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResultServices: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCallStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IWbemCallResult_Impl: windows_core::IUnknownImpl {
    fn GetResultObject(&self, ltimeout: i32) -> windows_core::Result<IWbemClassObject>;
    fn GetResultString(&self, ltimeout: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetResultServices(&self, ltimeout: i32) -> windows_core::Result<IWbemServices>;
    fn GetCallStatus(&self, ltimeout: i32) -> windows_core::Result<i32>;
}
impl IWbemCallResult_Vtbl {
    pub const fn new<Identity: IWbemCallResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResultObject<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppresultobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetResultObject(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        ppresultobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResultString<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetResultString(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        pstrresultstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResultServices<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetResultServices(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        ppservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCallStatus<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetCallStatus(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        plstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResultObject: GetResultObject::<Identity, OFFSET>,
            GetResultString: GetResultString::<Identity, OFFSET>,
            GetResultServices: GetResultServices::<Identity, OFFSET>,
            GetCallStatus: GetCallStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemCallResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemCallResult {}
windows_core::imp::define_interface!(IWbemClassObject, IWbemClassObject_Vtbl, 0xdc12a681_737f_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemClassObject, windows_core::IUnknown);
impl IWbemClassObject {
    pub unsafe fn GetQualifierSet(&self) -> windows_core::Result<IWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQualifierSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::oaidl::VARIANT, ptype: *mut CIMTYPE, plflavor: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), ptype as _, plflavor as _) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, lflags: i32, pval: *const super::oaidl::VARIANT, r#type: CIMTYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), r#type) }
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), wszname.param().abi()) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetNames<P0>(&self, wszqualifiername: P0, lflags: i32, pqualifierval: *const super::oaidl::VARIANT) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), wszqualifiername.param().abi(), lflags, core::mem::transmute(pqualifierval), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lenumflags) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut windows_core::BSTR, pval: *mut super::oaidl::VARIANT, ptype: *mut CIMTYPE, plflavor: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(strname), core::mem::transmute(pval), ptype as _, plflavor as _) }
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPropertyQualifierSet<P0>(&self, wszproperty: P0) -> windows_core::Result<IWbemQualifierSet>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyQualifierSet)(windows_core::Interface::as_raw(self), wszproperty.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectText)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnDerivedClass)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnInstance)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CompareTo<P1>(&self, lflags: i32, pcompareto: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompareTo)(windows_core::Interface::as_raw(self), lflags, pcompareto.param().abi()) }
    }
    pub unsafe fn GetPropertyOrigin<P0>(&self, wszname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyOrigin)(windows_core::Interface::as_raw(self), wszname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InheritsFrom<P0>(&self, strancestor: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InheritsFrom)(windows_core::Interface::as_raw(self), strancestor.param().abi()) }
    }
    pub unsafe fn GetMethod<P0>(&self, wszname: P0, lflags: i32, ppinsignature: *mut Option<Self>, ppoutsignature: *mut Option<Self>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMethod)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(ppinsignature), core::mem::transmute(ppoutsignature)) }
    }
    pub unsafe fn PutMethod<P0, P2, P3>(&self, wszname: P0, lflags: i32, pinsignature: P2, poutsignature: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<Self>,
        P3: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutMethod)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, pinsignature.param().abi(), poutsignature.param().abi()) }
    }
    pub unsafe fn DeleteMethod<P0>(&self, wszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteMethod)(windows_core::Interface::as_raw(self), wszname.param().abi()) }
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginMethodEnumeration)(windows_core::Interface::as_raw(self), lenumflags) }
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut windows_core::BSTR, ppinsignature: *mut Option<Self>, ppoutsignature: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NextMethod)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(ppinsignature), core::mem::transmute(ppoutsignature)) }
    }
    pub unsafe fn EndMethodEnumeration(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndMethodEnumeration)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMethodQualifierSet<P0>(&self, wszmethod: P0) -> windows_core::Result<IWbemQualifierSet>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodQualifierSet)(windows_core::Interface::as_raw(self), wszmethod.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMethodOrigin<P0>(&self, wszmethodname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodOrigin)(windows_core::Interface::as_raw(self), wszmethodname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut super::oaidl::VARIANT, *mut CIMTYPE, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Get: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *const super::oaidl::VARIANT, CIMTYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *const super::oaidl::VARIANT, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut super::oaidl::VARIANT, *mut CIMTYPE, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnDerivedClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnInstance: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompareTo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InheritsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub BeginMethodEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NextMethod: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndMethodEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IWbemClassObject_Impl: windows_core::IUnknownImpl {
    fn GetQualifierSet(&self) -> windows_core::Result<IWbemQualifierSet>;
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut super::oaidl::VARIANT, ptype: *mut CIMTYPE, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *const super::oaidl::VARIANT, r#type: CIMTYPE) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, wszqualifiername: &windows_core::PCWSTR, lflags: i32, pqualifierval: *const super::oaidl::VARIANT) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn BeginEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, strname: *mut windows_core::BSTR, pval: *mut super::oaidl::VARIANT, ptype: *mut CIMTYPE, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn GetPropertyQualifierSet(&self, wszproperty: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn Clone(&self) -> windows_core::Result<IWbemClassObject>;
    fn GetObjectText(&self, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn SpawnInstance(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn CompareTo(&self, lflags: i32, pcompareto: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<()>;
    fn GetPropertyOrigin(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn InheritsFrom(&self, strancestor: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, ppinsignature: windows_core::OutRef<IWbemClassObject>, ppoutsignature: windows_core::OutRef<IWbemClassObject>) -> windows_core::Result<()>;
    fn PutMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, pinsignature: windows_core::Ref<IWbemClassObject>, poutsignature: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<()>;
    fn DeleteMethod(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BeginMethodEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn NextMethod(&self, lflags: i32, pstrname: *mut windows_core::BSTR, ppinsignature: windows_core::OutRef<IWbemClassObject>, ppoutsignature: windows_core::OutRef<IWbemClassObject>) -> windows_core::Result<()>;
    fn EndMethodEnumeration(&self) -> windows_core::Result<()>;
    fn GetMethodQualifierSet(&self, wszmethod: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn GetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IWbemClassObject_Vtbl {
    pub const fn new<Identity: IWbemClassObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetQualifierSet(this) {
                    Ok(ok__) => {
                        ppqualset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Get<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut super::oaidl::VARIANT, ptype: *mut CIMTYPE, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn Put<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *const super::oaidl::VARIANT, r#type: CIMTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Delete(this, core::mem::transmute(&wszname)).into()
            }
        }
        unsafe extern "system" fn GetNames<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszqualifiername: windows_core::PCWSTR, lflags: i32, pqualifierval: *const super::oaidl::VARIANT, pnames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetNames(this, core::mem::transmute(&wszqualifiername), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pqualifierval)) {
                    Ok(ok__) => {
                        pnames.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strname: *mut *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT, ptype: *mut CIMTYPE, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&strname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::EndEnumeration(this).into()
            }
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszproperty: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetPropertyQualifierSet(this, core::mem::transmute(&wszproperty)) {
                    Ok(ok__) => {
                        ppqualset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppcopy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectText<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrobjecttext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetObjectText(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pstrobjecttext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnDerivedClass<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::SpawnDerivedClass(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppnewclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnInstance<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::SpawnInstance(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppnewinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareTo<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pcompareto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::CompareTo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcompareto)).into()
            }
        }
        unsafe extern "system" fn GetPropertyOrigin<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pstrclassname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetPropertyOrigin(this, core::mem::transmute(&wszname)) {
                    Ok(ok__) => {
                        pstrclassname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InheritsFrom<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strancestor: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::InheritsFrom(this, core::mem::transmute(&strancestor)).into()
            }
        }
        unsafe extern "system" fn GetMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::GetMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
            }
        }
        unsafe extern "system" fn PutMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pinsignature: *mut core::ffi::c_void, poutsignature: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::PutMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pinsignature), core::mem::transmute_copy(&poutsignature)).into()
            }
        }
        unsafe extern "system" fn DeleteMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::DeleteMethod(this, core::mem::transmute(&wszname)).into()
            }
        }
        unsafe extern "system" fn BeginMethodEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::BeginMethodEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
            }
        }
        unsafe extern "system" fn NextMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut *mut core::ffi::c_void, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::NextMethod(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
            }
        }
        unsafe extern "system" fn EndMethodEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::EndMethodEnumeration(this).into()
            }
        }
        unsafe extern "system" fn GetMethodQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethod: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetMethodQualifierSet(this, core::mem::transmute(&wszmethod)) {
                    Ok(ok__) => {
                        ppqualset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMethodOrigin<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, pstrclassname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetMethodOrigin(this, core::mem::transmute(&wszmethodname)) {
                    Ok(ok__) => {
                        pstrclassname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetQualifierSet: GetQualifierSet::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
            GetPropertyQualifierSet: GetPropertyQualifierSet::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetObjectText: GetObjectText::<Identity, OFFSET>,
            SpawnDerivedClass: SpawnDerivedClass::<Identity, OFFSET>,
            SpawnInstance: SpawnInstance::<Identity, OFFSET>,
            CompareTo: CompareTo::<Identity, OFFSET>,
            GetPropertyOrigin: GetPropertyOrigin::<Identity, OFFSET>,
            InheritsFrom: InheritsFrom::<Identity, OFFSET>,
            GetMethod: GetMethod::<Identity, OFFSET>,
            PutMethod: PutMethod::<Identity, OFFSET>,
            DeleteMethod: DeleteMethod::<Identity, OFFSET>,
            BeginMethodEnumeration: BeginMethodEnumeration::<Identity, OFFSET>,
            NextMethod: NextMethod::<Identity, OFFSET>,
            EndMethodEnumeration: EndMethodEnumeration::<Identity, OFFSET>,
            GetMethodQualifierSet: GetMethodQualifierSet::<Identity, OFFSET>,
            GetMethodOrigin: GetMethodOrigin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IWbemClassObject {}
windows_core::imp::define_interface!(IWbemConfigureRefresher, IWbemConfigureRefresher_Vtbl, 0x49353c92_516b_11d1_aea6_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemConfigureRefresher, windows_core::IUnknown);
impl IWbemConfigureRefresher {
    pub unsafe fn AddObjectByPath<P0, P1, P3>(&self, pnamespace: P0, wszpath: P1, lflags: i32, pcontext: P3, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddObjectByPath)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszpath.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid as _) }
    }
    pub unsafe fn AddObjectByTemplate<P0, P1, P3>(&self, pnamespace: P0, ptemplate: P1, lflags: i32, pcontext: P3, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<IWbemClassObject>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddObjectByTemplate)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), ptemplate.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid as _) }
    }
    pub unsafe fn AddRefresher<P0>(&self, prefresher: P0, lflags: i32, plid: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemRefresher>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRefresher)(windows_core::Interface::as_raw(self), prefresher.param().abi(), lflags, plid as _) }
    }
    pub unsafe fn Remove(&self, lid: i32, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lid, lflags) }
    }
    pub unsafe fn AddEnum<P0, P1, P3>(&self, pnamespace: P0, wszclassname: P1, lflags: i32, pcontext: P3, ppenum: *mut Option<IWbemHiPerfEnum>, plid: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEnum)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclassname.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(ppenum), plid as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConfigureRefresher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddObjectByPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddObjectByTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddRefresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IWbemConfigureRefresher_Impl: windows_core::IUnknownImpl {
    fn AddObjectByPath(&self, pnamespace: windows_core::Ref<IWbemServices>, wszpath: &windows_core::PCWSTR, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, pprefreshable: windows_core::OutRef<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddObjectByTemplate(&self, pnamespace: windows_core::Ref<IWbemServices>, ptemplate: windows_core::Ref<IWbemClassObject>, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, pprefreshable: windows_core::OutRef<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddRefresher(&self, prefresher: windows_core::Ref<IWbemRefresher>, lflags: i32, plid: *mut i32) -> windows_core::Result<()>;
    fn Remove(&self, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn AddEnum(&self, pnamespace: windows_core::Ref<IWbemServices>, wszclassname: &windows_core::PCWSTR, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, ppenum: windows_core::OutRef<IWbemHiPerfEnum>, plid: *mut i32) -> windows_core::Result<()>;
}
impl IWbemConfigureRefresher_Vtbl {
    pub const fn new<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddObjectByPath<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddObjectByPath(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute(&wszpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn AddObjectByTemplate<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddObjectByTemplate(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&ptemplate), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn AddRefresher<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddRefresher(this, core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::Remove(this, core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn AddEnum<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclassname: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddEnum(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute(&wszclassname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&ppenum), core::mem::transmute_copy(&plid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjectByPath: AddObjectByPath::<Identity, OFFSET>,
            AddObjectByTemplate: AddObjectByTemplate::<Identity, OFFSET>,
            AddRefresher: AddRefresher::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            AddEnum: AddEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConfigureRefresher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemConfigureRefresher {}
windows_core::imp::define_interface!(IWbemContext, IWbemContext_Vtbl, 0x44aca674_e8fc_11d0_a07c_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemContext, windows_core::IUnknown);
impl IWbemContext {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lflags) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(pvalue)) }
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetValue<P0>(&self, wszname: P0, lflags: i32, pvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pvalue)) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetValue<P0>(&self, wszname: P0, lflags: i32) -> windows_core::Result<super::oaidl::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DeleteValue<P0>(&self, wszname: P0, lflags: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags) }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_oaidl")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetValue: usize,
    pub DeleteValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IWbemContext_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IWbemContext>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pvalue: *mut super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn SetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32, pvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn DeleteValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IWbemContext_Vtbl {
    pub const fn new<Identity: IWbemContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemContext_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppnewcopy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNames<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemContext_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pnames.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut *mut core::ffi::c_void, pvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::EndEnumeration(this).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::SetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemContext_Impl::GetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::DeleteValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            DeleteValue: DeleteValue::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IWbemContext {}
windows_core::imp::define_interface!(IWbemHiPerfEnum, IWbemHiPerfEnum_Vtbl, 0x2705c288_79ae_11d2_b348_00105a1f8177);
windows_core::imp::interface_hierarchy!(IWbemHiPerfEnum, windows_core::IUnknown);
impl IWbemHiPerfEnum {
    pub unsafe fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const Option<IWbemObjectAccess>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddObjects)(windows_core::Interface::as_raw(self), lflags, unumobjects, apids, core::mem::transmute(apobj)) }
    }
    pub unsafe fn RemoveObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveObjects)(windows_core::Interface::as_raw(self), lflags, unumobjects, apids) }
    }
    pub unsafe fn GetObjects(&self, lflags: i32, unumobjects: u32, apobj: *mut Option<IWbemObjectAccess>, pureturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjects)(windows_core::Interface::as_raw(self), lflags, unumobjects, core::mem::transmute(apobj), pureturned as _) }
    }
    pub unsafe fn RemoveAll(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self), lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const i32) -> windows_core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IWbemHiPerfEnum_Impl: windows_core::IUnknownImpl {
    fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const Option<IWbemObjectAccess>) -> windows_core::Result<()>;
    fn RemoveObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::Result<()>;
    fn GetObjects(&self, lflags: i32, unumobjects: u32, apobj: windows_core::OutRef<IWbemObjectAccess>, pureturned: *mut u32) -> windows_core::Result<()>;
    fn RemoveAll(&self, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemHiPerfEnum_Vtbl {
    pub const fn new<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::AddObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids), core::mem::transmute_copy(&apobj)).into()
            }
        }
        unsafe extern "system" fn RemoveObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::RemoveObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids)).into()
            }
        }
        unsafe extern "system" fn GetObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::GetObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&pureturned)).into()
            }
        }
        unsafe extern "system" fn RemoveAll<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::RemoveAll(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjects: AddObjects::<Identity, OFFSET>,
            RemoveObjects: RemoveObjects::<Identity, OFFSET>,
            GetObjects: GetObjects::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemHiPerfEnum {}
windows_core::imp::define_interface!(IWbemLocator, IWbemLocator_Vtbl, 0xdc12a687_737f_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemLocator, windows_core::IUnknown);
impl IWbemLocator {
    pub unsafe fn ConnectServer<P6>(&self, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: P6) -> windows_core::Result<IWbemServices>
    where
        P6: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strnetworkresource), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lsecurityflags, core::mem::transmute_copy(strauthority), pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemLocator_Impl: windows_core::IUnknownImpl {
    fn ConnectServer(&self, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl IWbemLocator_Vtbl {
    pub const fn new<Identity: IWbemLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectServer<Identity: IWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnetworkresource: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lsecurityflags: i32, strauthority: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLocator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemLocator {}
windows_core::imp::define_interface!(IWbemObjectAccess, IWbemObjectAccess_Vtbl, 0x49353c9a_516b_11d1_aea6_00c04fb68820);
impl core::ops::Deref for IWbemObjectAccess {
    type Target = IWbemClassObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemObjectAccess, windows_core::IUnknown, IWbemClassObject);
impl IWbemObjectAccess {
    pub unsafe fn GetPropertyHandle<P0>(&self, wszpropertyname: P0, ptype: *mut CIMTYPE, plhandle: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyHandle)(windows_core::Interface::as_raw(self), wszpropertyname.param().abi(), ptype as _, plhandle as _) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn WritePropertyValue(&self, lhandle: i32, lnumbytes: i32, adata: *const super::rpcndr::byte) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyValue)(windows_core::Interface::as_raw(self), lhandle, lnumbytes, adata) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn ReadPropertyValue(&self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut super::rpcndr::byte) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadPropertyValue)(windows_core::Interface::as_raw(self), lhandle, lbuffersize, plnumbytes as _, adata as _) }
    }
    pub unsafe fn ReadDWORD(&self, lhandle: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDWORD)(windows_core::Interface::as_raw(self), lhandle, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteDWORD(&self, lhandle: i32, dw: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteDWORD)(windows_core::Interface::as_raw(self), lhandle, dw) }
    }
    pub unsafe fn ReadQWORD(&self, lhandle: i32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadQWORD)(windows_core::Interface::as_raw(self), lhandle, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteQWORD(&self, lhandle: i32, pw: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteQWORD)(windows_core::Interface::as_raw(self), lhandle, pw) }
    }
    pub unsafe fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut windows_core::BSTR, ptype: *mut CIMTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfoByHandle)(windows_core::Interface::as_raw(self), lhandle, core::mem::transmute(pstrname), ptype as _) }
    }
    pub unsafe fn Lock(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn Unlock(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectAccess_Vtbl {
    pub base__: IWbemClassObject_Vtbl,
    pub GetPropertyHandle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut CIMTYPE, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpcndr")]
    pub WritePropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const super::rpcndr::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    WritePropertyValue: usize,
    #[cfg(feature = "Win32_rpcndr")]
    pub ReadPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32, *mut super::rpcndr::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    ReadPropertyValue: usize,
    pub ReadDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub WriteDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub ReadQWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u64) -> windows_core::HRESULT,
    pub WriteQWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u64) -> windows_core::HRESULT,
    pub GetPropertyInfoByHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut CIMTYPE) -> windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IWbemObjectAccess_Impl: IWbemClassObject_Impl {
    fn GetPropertyHandle(&self, wszpropertyname: &windows_core::PCWSTR, ptype: *mut CIMTYPE, plhandle: *mut i32) -> windows_core::Result<()>;
    fn WritePropertyValue(&self, lhandle: i32, lnumbytes: i32, adata: *const super::rpcndr::byte) -> windows_core::Result<()>;
    fn ReadPropertyValue(&self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut super::rpcndr::byte) -> windows_core::Result<()>;
    fn ReadDWORD(&self, lhandle: i32) -> windows_core::Result<u32>;
    fn WriteDWORD(&self, lhandle: i32, dw: u32) -> windows_core::Result<()>;
    fn ReadQWORD(&self, lhandle: i32) -> windows_core::Result<u64>;
    fn WriteQWORD(&self, lhandle: i32, pw: u64) -> windows_core::Result<()>;
    fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut windows_core::BSTR, ptype: *mut CIMTYPE) -> windows_core::Result<()>;
    fn Lock(&self, lflags: i32) -> windows_core::Result<()>;
    fn Unlock(&self, lflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IWbemObjectAccess_Vtbl {
    pub const fn new<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyHandle<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, ptype: *mut CIMTYPE, plhandle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::GetPropertyHandle(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plhandle)).into()
            }
        }
        unsafe extern "system" fn WritePropertyValue<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const super::rpcndr::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::WritePropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lnumbytes), core::mem::transmute_copy(&adata)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyValue<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut super::rpcndr::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::ReadPropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lbuffersize), core::mem::transmute_copy(&plnumbytes), core::mem::transmute_copy(&adata)).into()
            }
        }
        unsafe extern "system" fn ReadDWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectAccess_Impl::ReadDWORD(this, core::mem::transmute_copy(&lhandle)) {
                    Ok(ok__) => {
                        pdw.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteDWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, dw: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::WriteDWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&dw)).into()
            }
        }
        unsafe extern "system" fn ReadQWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectAccess_Impl::ReadQWORD(this, core::mem::transmute_copy(&lhandle)) {
                    Ok(ok__) => {
                        pqw.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteQWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pw: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::WriteQWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pw)).into()
            }
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pstrname: *mut *mut core::ffi::c_void, ptype: *mut CIMTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::GetPropertyInfoByHandle(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::Lock(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::Unlock(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self {
            base__: IWbemClassObject_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyHandle: GetPropertyHandle::<Identity, OFFSET>,
            WritePropertyValue: WritePropertyValue::<Identity, OFFSET>,
            ReadPropertyValue: ReadPropertyValue::<Identity, OFFSET>,
            ReadDWORD: ReadDWORD::<Identity, OFFSET>,
            WriteDWORD: WriteDWORD::<Identity, OFFSET>,
            ReadQWORD: ReadQWORD::<Identity, OFFSET>,
            WriteQWORD: WriteQWORD::<Identity, OFFSET>,
            GetPropertyInfoByHandle: GetPropertyInfoByHandle::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectAccess as windows_core::Interface>::IID || iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IWbemObjectAccess {}
windows_core::imp::define_interface!(IWbemObjectSink, IWbemObjectSink_Vtbl, 0x7c857801_7381_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemObjectSink, windows_core::IUnknown);
impl IWbemObjectSink {
    pub unsafe fn Indicate(&self, lobjectcount: i32, apobjarray: *const Option<IWbemClassObject>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Indicate)(windows_core::Interface::as_raw(self), lobjectcount, core::mem::transmute(apobjarray)) }
    }
    pub unsafe fn SetStatus<P3>(&self, lflags: i32, hresult: windows_core::HRESULT, strparam: &windows_core::BSTR, pobjparam: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), lflags, hresult, core::mem::transmute_copy(strparam), pobjparam.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Indicate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::HRESULT, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemObjectSink_Impl: windows_core::IUnknownImpl {
    fn Indicate(&self, lobjectcount: i32, apobjarray: *const Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn SetStatus(&self, lflags: i32, hresult: windows_core::HRESULT, strparam: &windows_core::BSTR, pobjparam: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<()>;
}
impl IWbemObjectSink_Vtbl {
    pub const fn new<Identity: IWbemObjectSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Indicate<Identity: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjectcount: i32, apobjarray: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSink_Impl::Indicate(this, core::mem::transmute_copy(&lobjectcount), core::mem::transmute_copy(&apobjarray)).into()
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hresult: windows_core::HRESULT, strparam: *mut core::ffi::c_void, pobjparam: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSink_Impl::SetStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hresult), core::mem::transmute(&strparam), core::mem::transmute_copy(&pobjparam)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Indicate: Indicate::<Identity, OFFSET>, SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemObjectSink {}
windows_core::imp::define_interface!(IWbemObjectSinkEx, IWbemObjectSinkEx_Vtbl, 0xe7d35cfa_348b_485e_b524_252725d697ca);
impl core::ops::Deref for IWbemObjectSinkEx {
    type Target = IWbemObjectSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemObjectSinkEx, windows_core::IUnknown, IWbemObjectSink);
impl IWbemObjectSinkEx {
    pub unsafe fn WriteMessage(&self, uchannel: u32, strmessage: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteMessage)(windows_core::Interface::as_raw(self), uchannel, core::mem::transmute_copy(strmessage)) }
    }
    pub unsafe fn WriteError<P0>(&self, pobjerror: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteError)(windows_core::Interface::as_raw(self), pobjerror.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PromptUser(&self, strmessage: &windows_core::BSTR, uprompttype: u8) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PromptUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strmessage), uprompttype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteProgress(&self, stractivity: &windows_core::BSTR, strcurrentoperation: &windows_core::BSTR, strstatusdescription: &windows_core::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteProgress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(stractivity), core::mem::transmute_copy(strcurrentoperation), core::mem::transmute_copy(strstatusdescription), upercentcomplete, usecondsremaining) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn WriteStreamParameter(&self, strname: &windows_core::BSTR, vtvalue: *const super::oaidl::VARIANT, ultype: u32, ulflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteStreamParameter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), core::mem::transmute(vtvalue), ultype, ulflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSinkEx_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub WriteMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub PromptUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u8, *mut u8) -> windows_core::HRESULT,
    pub WriteProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub WriteStreamParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    WriteStreamParameter: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IWbemObjectSinkEx_Impl: IWbemObjectSink_Impl {
    fn WriteMessage(&self, uchannel: u32, strmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WriteError(&self, pobjerror: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<u8>;
    fn PromptUser(&self, strmessage: &windows_core::BSTR, uprompttype: u8) -> windows_core::Result<u8>;
    fn WriteProgress(&self, stractivity: &windows_core::BSTR, strcurrentoperation: &windows_core::BSTR, strstatusdescription: &windows_core::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::Result<()>;
    fn WriteStreamParameter(&self, strname: &windows_core::BSTR, vtvalue: *const super::oaidl::VARIANT, ultype: u32, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IWbemObjectSinkEx_Vtbl {
    pub const fn new<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteMessage<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uchannel: u32, strmessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSinkEx_Impl::WriteMessage(this, core::mem::transmute_copy(&uchannel), core::mem::transmute(&strmessage)).into()
            }
        }
        unsafe extern "system" fn WriteError<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjerror: *mut core::ffi::c_void, pureturned: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectSinkEx_Impl::WriteError(this, core::mem::transmute_copy(&pobjerror)) {
                    Ok(ok__) => {
                        pureturned.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PromptUser<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmessage: *mut core::ffi::c_void, uprompttype: u8, pureturned: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectSinkEx_Impl::PromptUser(this, core::mem::transmute(&strmessage), core::mem::transmute_copy(&uprompttype)) {
                    Ok(ok__) => {
                        pureturned.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteProgress<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stractivity: *mut core::ffi::c_void, strcurrentoperation: *mut core::ffi::c_void, strstatusdescription: *mut core::ffi::c_void, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSinkEx_Impl::WriteProgress(this, core::mem::transmute(&stractivity), core::mem::transmute(&strcurrentoperation), core::mem::transmute(&strstatusdescription), core::mem::transmute_copy(&upercentcomplete), core::mem::transmute_copy(&usecondsremaining)).into()
            }
        }
        unsafe extern "system" fn WriteStreamParameter<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, vtvalue: *const super::oaidl::VARIANT, ultype: u32, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSinkEx_Impl::WriteStreamParameter(this, core::mem::transmute(&strname), core::mem::transmute_copy(&vtvalue), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, OFFSET>(),
            WriteMessage: WriteMessage::<Identity, OFFSET>,
            WriteError: WriteError::<Identity, OFFSET>,
            PromptUser: PromptUser::<Identity, OFFSET>,
            WriteProgress: WriteProgress::<Identity, OFFSET>,
            WriteStreamParameter: WriteStreamParameter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSinkEx as windows_core::Interface>::IID || iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IWbemObjectSinkEx {}
windows_core::imp::define_interface!(IWbemObjectTextSrc, IWbemObjectTextSrc_Vtbl, 0xbfbf883a_cad7_11d3_a11b_00105a1f515a);
windows_core::imp::interface_hierarchy!(IWbemObjectTextSrc, windows_core::IUnknown);
impl IWbemObjectTextSrc {
    pub unsafe fn GetText<P1, P3>(&self, lflags: i32, pobj: P1, uobjtextformat: u32, pctx: P3) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<IWbemClassObject>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pobj.param().abi(), uobjtextformat, pctx.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateFromText<P3>(&self, lflags: i32, strtext: &windows_core::BSTR, uobjtextformat: u32, pctx: P3) -> windows_core::Result<IWbemClassObject>
    where
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFromText)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(strtext), uobjtextformat, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectTextSrc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemObjectTextSrc_Impl: windows_core::IUnknownImpl {
    fn GetText(&self, lflags: i32, pobj: windows_core::Ref<IWbemClassObject>, uobjtextformat: u32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<windows_core::BSTR>;
    fn CreateFromText(&self, lflags: i32, strtext: &windows_core::BSTR, uobjtextformat: u32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemClassObject>;
}
impl IWbemObjectTextSrc_Vtbl {
    pub const fn new<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pobj: *mut core::ffi::c_void, uobjtextformat: u32, pctx: *mut core::ffi::c_void, strtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectTextSrc_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pobj), core::mem::transmute_copy(&uobjtextformat), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        strtext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromText<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strtext: *mut core::ffi::c_void, uobjtextformat: u32, pctx: *mut core::ffi::c_void, pnewobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectTextSrc_Impl::CreateFromText(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strtext), core::mem::transmute_copy(&uobjtextformat), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        pnewobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            CreateFromText: CreateFromText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectTextSrc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemObjectTextSrc {}
windows_core::imp::define_interface!(IWbemQualifierSet, IWbemQualifierSet_Vtbl, 0xdc12a680_737f_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemQualifierSet, windows_core::IUnknown);
impl IWbemQualifierSet {
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::oaidl::VARIANT, plflavor: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), plflavor as _) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, pval: *const super::oaidl::VARIANT, lflavor: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), wszname.param().abi(), core::mem::transmute(pval), lflavor) }
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), wszname.param().abi()) }
    }
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lflags) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pval: *mut super::oaidl::VARIANT, plflavor: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(pval), plflavor as _) }
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQualifierSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut super::oaidl::VARIANT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Get: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_oaidl")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut super::oaidl::VARIANT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IWbemQualifierSet_Impl: windows_core::IUnknownImpl {
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut super::oaidl::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, pval: *const super::oaidl::VARIANT, lflavor: i32) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pval: *mut super::oaidl::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IWbemQualifierSet_Vtbl {
    pub const fn new<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut super::oaidl::VARIANT, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn Put<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pval: *const super::oaidl::VARIANT, lflavor: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&lflavor)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Delete(this, core::mem::transmute(&wszname)).into()
            }
        }
        unsafe extern "system" fn GetNames<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemQualifierSet_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pnames.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::EndEnumeration(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQualifierSet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IWbemQualifierSet {}
windows_core::imp::define_interface!(IWbemRefresher, IWbemRefresher_Vtbl, 0x49353c99_516b_11d1_aea6_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemRefresher, windows_core::IUnknown);
impl IWbemRefresher {
    pub unsafe fn Refresh(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemRefresher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IWbemRefresher_Impl: windows_core::IUnknownImpl {
    fn Refresh(&self, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemRefresher_Vtbl {
    pub const fn new<Identity: IWbemRefresher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Refresh<Identity: IWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Refresh: Refresh::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemRefresher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemRefresher {}
windows_core::imp::define_interface!(IWbemServices, IWbemServices_Vtbl, 0x9556dc99_828c_11cf_a37e_00aa003240c7);
windows_core::imp::interface_hierarchy!(IWbemServices, windows_core::IUnknown);
impl IWbemServices {
    pub unsafe fn OpenNamespace<P2>(&self, strnamespace: &windows_core::BSTR, lflags: i32, pctx: P2, ppworkingnamespace: *mut Option<Self>, ppresult: *mut Option<IWbemCallResult>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenNamespace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strnamespace), lflags, pctx.param().abi(), core::mem::transmute(ppworkingnamespace), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn CancelAsyncCall<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CancelAsyncCall)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    pub unsafe fn QueryObjectSink(&self, lflags: i32) -> windows_core::Result<IWbemObjectSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryObjectSink)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObject<P2>(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: P2, ppobject: *mut Option<IWbemClassObject>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), core::mem::transmute(ppobject), core::mem::transmute(ppcallresult)) }
    }
    pub unsafe fn GetObjectAsync<P2, P3>(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: P2, presponsehandler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObjectAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn PutClass<P0, P2>(&self, pobject: P0, lflags: i32, pctx: P2, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutClass)(windows_core::Interface::as_raw(self), pobject.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult)) }
    }
    pub unsafe fn PutClassAsync<P0, P2, P3>(&self, pobject: P0, lflags: i32, pctx: P2, presponsehandler: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutClassAsync)(windows_core::Interface::as_raw(self), pobject.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn DeleteClass<P2>(&self, strclass: &windows_core::BSTR, lflags: i32, pctx: P2, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteClass)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult)) }
    }
    pub unsafe fn DeleteClassAsync<P2, P3>(&self, strclass: &windows_core::BSTR, lflags: i32, pctx: P2, presponsehandler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteClassAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn CreateClassEnum<P2>(&self, strsuperclass: &windows_core::BSTR, lflags: i32, pctx: P2) -> windows_core::Result<IEnumWbemClassObject>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateClassEnum)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsuperclass), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateClassEnumAsync<P2, P3>(&self, strsuperclass: &windows_core::BSTR, lflags: i32, pctx: P2, presponsehandler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClassEnumAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsuperclass), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn PutInstance<P0, P2>(&self, pinst: P0, lflags: i32, pctx: P2, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutInstance)(windows_core::Interface::as_raw(self), pinst.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult)) }
    }
    pub unsafe fn PutInstanceAsync<P0, P2, P3>(&self, pinst: P0, lflags: i32, pctx: P2, presponsehandler: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutInstanceAsync)(windows_core::Interface::as_raw(self), pinst.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn DeleteInstance<P2>(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: P2, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult)) }
    }
    pub unsafe fn DeleteInstanceAsync<P2, P3>(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: P2, presponsehandler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteInstanceAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn CreateInstanceEnum<P2>(&self, strfilter: &windows_core::BSTR, lflags: i32, pctx: P2) -> windows_core::Result<IEnumWbemClassObject>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceEnum)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilter), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateInstanceEnumAsync<P2, P3>(&self, strfilter: &windows_core::BSTR, lflags: i32, pctx: P2, presponsehandler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceEnumAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilter), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn ExecQuery<P3>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: P3) -> windows_core::Result<IEnumWbemClassObject>
    where
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecQueryAsync<P3, P4>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: P3, presponsehandler: P4) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecQueryAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn ExecNotificationQuery<P3>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: P3) -> windows_core::Result<IEnumWbemClassObject>
    where
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecNotificationQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecNotificationQueryAsync<P3, P4>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: P3, presponsehandler: P4) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), presponsehandler.param().abi()) }
    }
    pub unsafe fn ExecMethod<P3, P4>(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: i32, pctx: P3, pinparams: P4, ppoutparams: *mut Option<IWbemClassObject>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethod)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), lflags, pctx.param().abi(), pinparams.param().abi(), core::mem::transmute(ppoutparams), core::mem::transmute(ppcallresult)) }
    }
    pub unsafe fn ExecMethodAsync<P3, P4, P5>(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: i32, pctx: P3, pinparams: P4, presponsehandler: P5) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemClassObject>,
        P5: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethodAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), lflags, pctx.param().abi(), pinparams.param().abi(), presponsehandler.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryObjectSink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutClassAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteClassAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassEnumAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceEnumAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethodAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemServices_Impl: windows_core::IUnknownImpl {
    fn OpenNamespace(&self, strnamespace: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, ppworkingnamespace: windows_core::OutRef<IWbemServices>, ppresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, psink: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn QueryObjectSink(&self, lflags: i32) -> windows_core::Result<IWbemObjectSink>;
    fn GetObject(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, ppobject: windows_core::OutRef<IWbemClassObject>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn GetObjectAsync(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutClass(&self, pobject: windows_core::Ref<IWbemClassObject>, lflags: i32, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutClassAsync(&self, pobject: windows_core::Ref<IWbemClassObject>, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteClass(&self, strclass: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteClassAsync(&self, strclass: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateClassEnum(&self, strsuperclass: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateClassEnumAsync(&self, strsuperclass: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutInstance(&self, pinst: windows_core::Ref<IWbemClassObject>, lflags: i32, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutInstanceAsync(&self, pinst: windows_core::Ref<IWbemClassObject>, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteInstance(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteInstanceAsync(&self, strobjectpath: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateInstanceEnum(&self, strfilter: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateInstanceEnumAsync(&self, strfilter: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecNotificationQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, pinparams: windows_core::Ref<IWbemClassObject>, ppoutparams: windows_core::OutRef<IWbemClassObject>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn ExecMethodAsync(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, pinparams: windows_core::Ref<IWbemClassObject>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
}
impl IWbemServices_Vtbl {
    pub const fn new<Identity: IWbemServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenNamespace<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppworkingnamespace: *mut *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::OpenNamespace(this, core::mem::transmute(&strnamespace), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppworkingnamespace), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::CancelAsyncCall(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn QueryObjectSink<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppresponsehandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::QueryObjectSink(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppresponsehandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObject<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::GetObject(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppobject), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn GetObjectAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::GetObjectAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn PutClass<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutClass(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn PutClassAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutClassAsync(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn DeleteClass<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteClass(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn DeleteClassAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteClassAsync(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn CreateClassEnum<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::CreateClassEnum(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateClassEnumAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::CreateClassEnumAsync(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn PutInstance<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutInstance(this, core::mem::transmute_copy(&pinst), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn PutInstanceAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutInstanceAsync(this, core::mem::transmute_copy(&pinst), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn DeleteInstance<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteInstance(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn DeleteInstanceAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteInstanceAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn CreateInstanceEnum<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::CreateInstanceEnum(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::CreateInstanceEnumAsync(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn ExecQuery<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecNotificationQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn ExecMethod<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, ppoutparams: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&pinparams), core::mem::transmute_copy(&ppoutparams), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecMethodAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&pinparams), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenNamespace: OpenNamespace::<Identity, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, OFFSET>,
            QueryObjectSink: QueryObjectSink::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetObjectAsync: GetObjectAsync::<Identity, OFFSET>,
            PutClass: PutClass::<Identity, OFFSET>,
            PutClassAsync: PutClassAsync::<Identity, OFFSET>,
            DeleteClass: DeleteClass::<Identity, OFFSET>,
            DeleteClassAsync: DeleteClassAsync::<Identity, OFFSET>,
            CreateClassEnum: CreateClassEnum::<Identity, OFFSET>,
            CreateClassEnumAsync: CreateClassEnumAsync::<Identity, OFFSET>,
            PutInstance: PutInstance::<Identity, OFFSET>,
            PutInstanceAsync: PutInstanceAsync::<Identity, OFFSET>,
            DeleteInstance: DeleteInstance::<Identity, OFFSET>,
            DeleteInstanceAsync: DeleteInstanceAsync::<Identity, OFFSET>,
            CreateInstanceEnum: CreateInstanceEnum::<Identity, OFFSET>,
            CreateInstanceEnumAsync: CreateInstanceEnumAsync::<Identity, OFFSET>,
            ExecQuery: ExecQuery::<Identity, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, OFFSET>,
            ExecMethod: ExecMethod::<Identity, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemServices {}
windows_core::imp::define_interface!(IWbemShutdown, IWbemShutdown_Vtbl, 0xb7b31df9_d515_11d3_a11c_00105a1f515a);
windows_core::imp::interface_hierarchy!(IWbemShutdown, windows_core::IUnknown);
impl IWbemShutdown {
    pub unsafe fn Shutdown<P2>(&self, ureason: i32, umaxmilliseconds: u32, pctx: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self), ureason, umaxmilliseconds, pctx.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemShutdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemShutdown_Impl: windows_core::IUnknownImpl {
    fn Shutdown(&self, ureason: i32, umaxmilliseconds: u32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<()>;
}
impl IWbemShutdown_Vtbl {
    pub const fn new<Identity: IWbemShutdown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Shutdown<Identity: IWbemShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemShutdown_Impl::Shutdown(this, core::mem::transmute_copy(&ureason), core::mem::transmute_copy(&umaxmilliseconds), core::mem::transmute_copy(&pctx)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Shutdown: Shutdown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemShutdown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemShutdown {}
windows_core::imp::define_interface!(IWbemStatusCodeText, IWbemStatusCodeText_Vtbl, 0xeb87e1bc_3233_11d2_aec9_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemStatusCodeText, windows_core::IUnknown);
impl IWbemStatusCodeText {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetErrorCodeText(&self, hres: windows_core::HRESULT, localeid: super::winnt::LCID, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorCodeText)(windows_core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetFacilityCodeText(&self, hres: windows_core::HRESULT, localeid: super::winnt::LCID, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFacilityCodeText)(windows_core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemStatusCodeText_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetErrorCodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, super::winnt::LCID, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetErrorCodeText: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetFacilityCodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, super::winnt::LCID, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetFacilityCodeText: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IWbemStatusCodeText_Impl: windows_core::IUnknownImpl {
    fn GetErrorCodeText(&self, hres: windows_core::HRESULT, localeid: super::winnt::LCID, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFacilityCodeText(&self, hres: windows_core::HRESULT, localeid: super::winnt::LCID, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_winnt")]
impl IWbemStatusCodeText_Vtbl {
    pub const fn new<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorCodeText<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: super::winnt::LCID, lflags: i32, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemStatusCodeText_Impl::GetErrorCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFacilityCodeText<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: super::winnt::LCID, lflags: i32, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemStatusCodeText_Impl::GetFacilityCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorCodeText: GetErrorCodeText::<Identity, OFFSET>,
            GetFacilityCodeText: GetFacilityCodeText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemStatusCodeText as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IWbemStatusCodeText {}
windows_core::imp::define_interface!(IWbemUnsecuredApartment, IWbemUnsecuredApartment_Vtbl, 0x31739d04_3471_4cf4_9a7c_57a44ae71956);
impl core::ops::Deref for IWbemUnsecuredApartment {
    type Target = IUnsecuredApartment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemUnsecuredApartment, windows_core::IUnknown, IUnsecuredApartment);
impl IWbemUnsecuredApartment {
    pub unsafe fn CreateSinkStub<P0, P2>(&self, psink: P0, dwflags: u32, wszreserved: P2) -> windows_core::Result<IWbemObjectSink>
    where
        P0: windows_core::Param<IWbemObjectSink>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSinkStub)(windows_core::Interface::as_raw(self), psink.param().abi(), dwflags, wszreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnsecuredApartment_Vtbl {
    pub base__: IUnsecuredApartment_Vtbl,
    pub CreateSinkStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemUnsecuredApartment_Impl: IUnsecuredApartment_Impl {
    fn CreateSinkStub(&self, psink: windows_core::Ref<IWbemObjectSink>, dwflags: u32, wszreserved: &windows_core::PCWSTR) -> windows_core::Result<IWbemObjectSink>;
}
impl IWbemUnsecuredApartment_Vtbl {
    pub const fn new<Identity: IWbemUnsecuredApartment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSinkStub<Identity: IWbemUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, dwflags: u32, wszreserved: windows_core::PCWSTR, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemUnsecuredApartment_Impl::CreateSinkStub(this, core::mem::transmute_copy(&psink), core::mem::transmute_copy(&dwflags), core::mem::transmute(&wszreserved)) {
                    Ok(ok__) => {
                        ppstub.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUnsecuredApartment_Vtbl::new::<Identity, OFFSET>(), CreateSinkStub: CreateSinkStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnsecuredApartment as windows_core::Interface>::IID || iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemUnsecuredApartment {}
pub const MofCompiler: windows_core::GUID = windows_core::GUID::from_u128(0x6daf9757_2e37_11d2_aec9_00c04fb68820);
pub const UnsecuredApartment: windows_core::GUID = windows_core::GUID::from_u128(0x49bd2028_1523_11d1_ad79_00c04fd8fdff);
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: WBEMSTATUS = -2147213309;
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: WBEMSTATUS = -2147213311;
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: WBEMSTATUS = -2147213310;
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: WBEMSTATUS = -2147205089;
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: WBEMSTATUS = -2147205094;
pub const WBEMMOF_E_DUPLICATE_PROPERTY: WBEMSTATUS = -2147205093;
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: WBEMSTATUS = -2147205087;
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: WBEMSTATUS = -2147205073;
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: WBEMSTATUS = -2147205072;
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: WBEMSTATUS = -2147205098;
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: WBEMSTATUS = -2147205079;
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: WBEMSTATUS = -2147205100;
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: WBEMSTATUS = -2147205116;
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: WBEMSTATUS = -2147205115;
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: WBEMSTATUS = -2147205114;
pub const WBEMMOF_E_EXPECTED_DOLLAR: WBEMSTATUS = -2147205095;
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: WBEMSTATUS = -2147205086;
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: WBEMSTATUS = -2147205117;
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: WBEMSTATUS = -2147205111;
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: WBEMSTATUS = -2147205108;
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: WBEMSTATUS = -2147205119;
pub const WBEMMOF_E_EXPECTED_SEMI: WBEMSTATUS = -2147205118;
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: WBEMSTATUS = -2147205112;
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: WBEMSTATUS = -2147205113;
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: WBEMSTATUS = -2147205085;
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: WBEMSTATUS = -2147205083;
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: WBEMSTATUS = -2147205104;
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: WBEMSTATUS = -2147205097;
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: WBEMSTATUS = -2147205071;
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: WBEMSTATUS = -2147205076;
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: WBEMSTATUS = -2147205103;
pub const WBEMMOF_E_INVALID_FILE: WBEMSTATUS = -2147205090;
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: WBEMSTATUS = -2147205080;
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: WBEMSTATUS = -2147205096;
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: WBEMSTATUS = -2147205092;
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: WBEMSTATUS = -2147205101;
pub const WBEMMOF_E_INVALID_PRAGMA: WBEMSTATUS = -2147205102;
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: WBEMSTATUS = -2147205075;
pub const WBEMMOF_E_MULTIPLE_ALIASES: WBEMSTATUS = -2147205084;
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: WBEMSTATUS = -2147205081;
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: WBEMSTATUS = -2147205082;
pub const WBEMMOF_E_NULL_ARRAY_ELEM: WBEMSTATUS = -2147205088;
pub const WBEMMOF_E_OUT_OF_RANGE: WBEMSTATUS = -2147205091;
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: WBEMSTATUS = -2147205074;
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: WBEMSTATUS = -2147205107;
pub const WBEMMOF_E_TYPE_MISMATCH: WBEMSTATUS = -2147205099;
pub const WBEMMOF_E_UNEXPECTED_ALIAS: WBEMSTATUS = -2147205106;
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: WBEMSTATUS = -2147205105;
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: WBEMSTATUS = -2147205110;
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: WBEMSTATUS = -2147205109;
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: WBEMSTATUS = -2147205077;
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: WBEMSTATUS = -2147205078;
pub type WBEMSTATUS = i32;
pub type WBEMSTATUS_FORMAT = i32;
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = 0;
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = 1;
pub type WBEM_BACKUP_RESTORE_FLAGS = i32;
pub type WBEM_CHANGE_FLAG_TYPE = i32;
pub type WBEM_COMPARISON_FLAG = i32;
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = 0;
pub type WBEM_COMPILER_OPTIONS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WBEM_COMPILE_STATUS_INFO {
    pub lPhaseError: i32,
    pub hRes: windows_core::HRESULT,
    pub ObjectNum: i32,
    pub FirstLine: i32,
    pub LastLine: i32,
    pub dwOutFlags: u32,
}
pub type WBEM_CONDITION_FLAG_TYPE = i32;
pub type WBEM_CONNECT_OPTIONS = i32;
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = 1;
pub const WBEM_E_ACCESS_DENIED: WBEMSTATUS = -2147217405;
pub const WBEM_E_AGGREGATING_BY_OBJECT: WBEMSTATUS = -2147217315;
pub const WBEM_E_ALREADY_EXISTS: WBEMSTATUS = -2147217383;
pub const WBEM_E_AMBIGUOUS_OPERATION: WBEMSTATUS = -2147217301;
pub const WBEM_E_AMENDED_OBJECT: WBEMSTATUS = -2147217306;
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: WBEMSTATUS = -2147217312;
pub const WBEM_E_BUFFER_TOO_SMALL: WBEMSTATUS = -2147217348;
pub const WBEM_E_CALL_CANCELLED: WBEMSTATUS = -2147217358;
pub const WBEM_E_CANNOT_BE_ABSTRACT: WBEMSTATUS = -2147217307;
pub const WBEM_E_CANNOT_BE_KEY: WBEMSTATUS = -2147217377;
pub const WBEM_E_CANNOT_BE_SINGLETON: WBEMSTATUS = -2147217364;
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: WBEMSTATUS = -2147217328;
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: WBEMSTATUS = -2147217335;
pub const WBEM_E_CIRCULAR_REFERENCE: WBEMSTATUS = -2147217337;
pub const WBEM_E_CLASS_HAS_CHILDREN: WBEMSTATUS = -2147217371;
pub const WBEM_E_CLASS_HAS_INSTANCES: WBEMSTATUS = -2147217370;
pub const WBEM_E_CLASS_NAME_TOO_WIDE: WBEMSTATUS = -2147217292;
pub const WBEM_E_CLIENT_TOO_SLOW: WBEMSTATUS = -2147217305;
pub const WBEM_E_CONNECTION_FAILED: WBEMSTATUS = -2147217295;
pub const WBEM_E_CRITICAL_ERROR: WBEMSTATUS = -2147217398;
pub const WBEM_E_DATABASE_VER_MISMATCH: WBEMSTATUS = -2147217288;
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: WBEMSTATUS = -2147217273;
pub const WBEM_E_FAILED: WBEMSTATUS = -2147217407;
pub const WBEM_E_FATAL_TRANSPORT_ERROR: WBEMSTATUS = -2147217274;
pub const WBEM_E_HANDLE_OUT_OF_DATE: WBEMSTATUS = -2147217296;
pub const WBEM_E_ILLEGAL_NULL: WBEMSTATUS = -2147217368;
pub const WBEM_E_ILLEGAL_OPERATION: WBEMSTATUS = -2147217378;
pub const WBEM_E_INCOMPLETE_CLASS: WBEMSTATUS = -2147217376;
pub const WBEM_E_INITIALIZATION_FAILURE: WBEMSTATUS = -2147217388;
pub const WBEM_E_INVALID_ASSOCIATION: WBEMSTATUS = -2147217302;
pub const WBEM_E_INVALID_CIM_TYPE: WBEMSTATUS = -2147217363;
pub const WBEM_E_INVALID_CLASS: WBEMSTATUS = -2147217392;
pub const WBEM_E_INVALID_CONTEXT: WBEMSTATUS = -2147217401;
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: WBEMSTATUS = -2147217341;
pub const WBEM_E_INVALID_FLAVOR: WBEMSTATUS = -2147217338;
pub const WBEM_E_INVALID_HANDLE_REQUEST: WBEMSTATUS = -2147217294;
pub const WBEM_E_INVALID_LOCALE: WBEMSTATUS = -2147217280;
pub const WBEM_E_INVALID_METHOD: WBEMSTATUS = -2147217362;
pub const WBEM_E_INVALID_METHOD_PARAMETERS: WBEMSTATUS = -2147217361;
pub const WBEM_E_INVALID_NAMESPACE: WBEMSTATUS = -2147217394;
pub const WBEM_E_INVALID_OBJECT: WBEMSTATUS = -2147217393;
pub const WBEM_E_INVALID_OBJECT_PATH: WBEMSTATUS = -2147217350;
pub const WBEM_E_INVALID_OPERATION: WBEMSTATUS = -2147217386;
pub const WBEM_E_INVALID_OPERATOR: WBEMSTATUS = -2147217309;
pub const WBEM_E_INVALID_PARAMETER: WBEMSTATUS = -2147217400;
pub const WBEM_E_INVALID_PARAMETER_ID: WBEMSTATUS = -2147217353;
pub const WBEM_E_INVALID_PROPERTY: WBEMSTATUS = -2147217359;
pub const WBEM_E_INVALID_PROPERTY_TYPE: WBEMSTATUS = -2147217366;
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: WBEMSTATUS = -2147217390;
pub const WBEM_E_INVALID_QUALIFIER: WBEMSTATUS = -2147217342;
pub const WBEM_E_INVALID_QUALIFIER_TYPE: WBEMSTATUS = -2147217367;
pub const WBEM_E_INVALID_QUERY: WBEMSTATUS = -2147217385;
pub const WBEM_E_INVALID_QUERY_TYPE: WBEMSTATUS = -2147217384;
pub const WBEM_E_INVALID_STREAM: WBEMSTATUS = -2147217397;
pub const WBEM_E_INVALID_SUPERCLASS: WBEMSTATUS = -2147217395;
pub const WBEM_E_INVALID_SYNTAX: WBEMSTATUS = -2147217375;
pub const WBEM_E_LOCAL_CREDENTIALS: WBEMSTATUS = -2147217308;
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: WBEMSTATUS = -2147217343;
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: WBEMSTATUS = -2147217344;
pub const WBEM_E_METHOD_DISABLED: WBEMSTATUS = -2147217322;
pub const WBEM_E_METHOD_NAME_TOO_WIDE: WBEMSTATUS = -2147217291;
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: WBEMSTATUS = -2147217323;
pub const WBEM_E_MISSING_AGGREGATION_LIST: WBEMSTATUS = -2147217317;
pub const WBEM_E_MISSING_GROUP_WITHIN: WBEMSTATUS = -2147217318;
pub const WBEM_E_MISSING_PARAMETER_ID: WBEMSTATUS = -2147217354;
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: WBEMSTATUS = -2147217352;
pub const WBEM_E_NONDECORATED_OBJECT: WBEMSTATUS = -2147217374;
pub const WBEM_E_NOT_AVAILABLE: WBEMSTATUS = -2147217399;
pub const WBEM_E_NOT_EVENT_CLASS: WBEMSTATUS = -2147217319;
pub const WBEM_E_NOT_FOUND: WBEMSTATUS = -2147217406;
pub const WBEM_E_NOT_SUPPORTED: WBEMSTATUS = -2147217396;
pub const WBEM_E_NO_KEY: WBEMSTATUS = -2147217271;
pub const WBEM_E_NO_SCHEMA: WBEMSTATUS = -2147217277;
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: WBEMSTATUS = -2147217304;
pub const WBEM_E_OUT_OF_DISK_SPACE: WBEMSTATUS = -2147217349;
pub const WBEM_E_OUT_OF_MEMORY: WBEMSTATUS = -2147217402;
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = -2147217382;
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: WBEMSTATUS = -2147217351;
pub const WBEM_E_PRIVILEGE_NOT_HELD: WBEMSTATUS = -2147217310;
pub const WBEM_E_PROPAGATED_METHOD: WBEMSTATUS = -2147217356;
pub const WBEM_E_PROPAGATED_PROPERTY: WBEMSTATUS = -2147217380;
pub const WBEM_E_PROPAGATED_QUALIFIER: WBEMSTATUS = -2147217381;
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: WBEMSTATUS = -2147217293;
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: WBEMSTATUS = -2147217316;
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: WBEMSTATUS = -2147217276;
pub const WBEM_E_PROVIDER_DISABLED: WBEMSTATUS = -2147217270;
pub const WBEM_E_PROVIDER_FAILURE: WBEMSTATUS = -2147217404;
pub const WBEM_E_PROVIDER_LOAD_FAILURE: WBEMSTATUS = -2147217389;
pub const WBEM_E_PROVIDER_NOT_CAPABLE: WBEMSTATUS = -2147217372;
pub const WBEM_E_PROVIDER_NOT_FOUND: WBEMSTATUS = -2147217391;
pub const WBEM_E_PROVIDER_NOT_REGISTERED: WBEMSTATUS = -2147217275;
pub const WBEM_E_PROVIDER_SUSPENDED: WBEMSTATUS = -2147217279;
pub const WBEM_E_PROVIDER_TIMED_OUT: WBEMSTATUS = -2147217272;
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: WBEMSTATUS = -2147217290;
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: WBEMSTATUS = -2147217369;
pub const WBEM_E_QUEUE_OVERFLOW: WBEMSTATUS = -2147217311;
pub const WBEM_E_QUOTA_VIOLATION: WBEMSTATUS = -2147217300;
pub const WBEM_E_READ_ONLY: WBEMSTATUS = -2147217373;
pub const WBEM_E_REFRESHER_BUSY: WBEMSTATUS = -2147217321;
pub const WBEM_E_RERUN_COMMAND: WBEMSTATUS = -2147217289;
pub const WBEM_E_RESERVED_001: WBEMSTATUS = -2147217299;
pub const WBEM_E_RESERVED_002: WBEMSTATUS = -2147217298;
pub const WBEM_E_SERVER_TOO_BUSY: WBEMSTATUS = -2147217339;
pub const WBEM_E_SHUTTING_DOWN: WBEMSTATUS = -2147217357;
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: WBEMSTATUS = -2147217278;
pub const WBEM_E_SYSTEM_PROPERTY: WBEMSTATUS = -2147217360;
pub const WBEM_E_TIMED_OUT: WBEMSTATUS = -2147217303;
pub const WBEM_E_TOO_MANY_PROPERTIES: WBEMSTATUS = -2147217327;
pub const WBEM_E_TOO_MUCH_DATA: WBEMSTATUS = -2147217340;
pub const WBEM_E_TRANSPORT_FAILURE: WBEMSTATUS = -2147217387;
pub const WBEM_E_TYPE_MISMATCH: WBEMSTATUS = -2147217403;
pub const WBEM_E_UNEXPECTED: WBEMSTATUS = -2147217379;
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: WBEMSTATUS = -2147217313;
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: WBEMSTATUS = -2147217346;
pub const WBEM_E_UNKNOWN_PACKET_TYPE: WBEMSTATUS = -2147217345;
pub const WBEM_E_UNPARSABLE_QUERY: WBEMSTATUS = -2147217320;
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: WBEMSTATUS = -2147217336;
pub const WBEM_E_UNSUPPORTED_LOCALE: WBEMSTATUS = -2147217297;
pub const WBEM_E_UNSUPPORTED_PARAMETER: WBEMSTATUS = -2147217355;
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: WBEMSTATUS = -2147217347;
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = -2147217325;
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: WBEMSTATUS = -2147217324;
pub const WBEM_E_UPDATE_TYPE_MISMATCH: WBEMSTATUS = -2147217326;
pub const WBEM_E_VALUE_OUT_OF_RANGE: WBEMSTATUS = -2147217365;
pub const WBEM_E_VETO_DELETE: WBEMSTATUS = -2147217287;
pub const WBEM_E_VETO_PUT: WBEMSTATUS = -2147217286;
pub const WBEM_FLAG_ADVISORY: WBEM_CHANGE_FLAG_TYPE = 65536;
pub const WBEM_FLAG_ALLOW_READ: WBEM_LOCKING_FLAG_TYPE = 1;
pub const WBEM_FLAG_ALWAYS: WBEM_CONDITION_FLAG_TYPE = 0;
pub const WBEM_FLAG_AUTORECOVER: WBEM_COMPILER_OPTIONS = 2;
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: WBEM_BACKUP_RESTORE_FLAGS = 0;
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: WBEM_BACKUP_RESTORE_FLAGS = 1;
pub const WBEM_FLAG_BIDIRECTIONAL: WBEM_GENERIC_FLAG_TYPE = 0;
pub const WBEM_FLAG_CHECK_ONLY: WBEM_COMPILER_OPTIONS = 1;
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: WBEM_CONDITION_FLAG_TYPE = 512;
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: WBEM_CONDITION_FLAG_TYPE = 256;
pub const WBEM_FLAG_CONNECT_PROVIDERS: WBEM_CONNECT_OPTIONS = 256;
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: WBEM_CONNECT_OPTIONS = 64;
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: WBEM_CONNECT_OPTIONS = 128;
pub const WBEM_FLAG_CONSOLE_PRINT: WBEM_COMPILER_OPTIONS = 8;
pub const WBEM_FLAG_CREATE_ONLY: WBEM_CHANGE_FLAG_TYPE = 2;
pub const WBEM_FLAG_CREATE_OR_UPDATE: WBEM_CHANGE_FLAG_TYPE = 0;
pub const WBEM_FLAG_DEEP: WBEM_QUERY_FLAG_TYPE = 0;
pub const WBEM_FLAG_DIRECT_READ: WBEM_GENERIC_FLAG_TYPE = 512;
pub const WBEM_FLAG_DONT_ADD_TO_LIST: WBEM_COMPILER_OPTIONS = 16;
pub const WBEM_FLAG_DONT_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = 0;
pub const WBEM_FLAG_ENSURE_LOCATABLE: WBEM_GENERIC_FLAG_TYPE = 256;
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = 16;
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = 32;
pub const WBEM_FLAG_FORWARD_ONLY: WBEM_GENERIC_FLAG_TYPE = 32;
pub const WBEM_FLAG_IGNORE_CASE: WBEM_COMPARISON_FLAG = 16;
pub const WBEM_FLAG_IGNORE_CLASS: WBEM_COMPARISON_FLAG = 8;
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: WBEM_COMPARISON_FLAG = 4;
pub const WBEM_FLAG_IGNORE_FLAVOR: WBEM_COMPARISON_FLAG = 32;
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: WBEM_COMPARISON_FLAG = 2;
pub const WBEM_FLAG_IGNORE_QUALIFIERS: WBEM_COMPARISON_FLAG = 1;
pub const WBEM_FLAG_KEYS_ONLY: WBEM_CONDITION_FLAG_TYPE = 4;
pub const WBEM_FLAG_LOCAL_ONLY: WBEM_CONDITION_FLAG_TYPE = 16;
pub const WBEM_FLAG_LONG_NAME: WBEM_INFORMATION_FLAG_TYPE = 2;
pub const WBEM_FLAG_NONSYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = 64;
pub const WBEM_FLAG_NO_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = 64;
pub const WBEM_FLAG_NO_FLAVORS: WBEM_TEXT_FLAG_TYPE = 1;
pub const WBEM_FLAG_ONLY_IF_FALSE: WBEM_CONDITION_FLAG_TYPE = 2;
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: WBEM_CONDITION_FLAG_TYPE = 3;
pub const WBEM_FLAG_ONLY_IF_TRUE: WBEM_CONDITION_FLAG_TYPE = 1;
pub const WBEM_FLAG_PROPAGATED_ONLY: WBEM_CONDITION_FLAG_TYPE = 32;
pub const WBEM_FLAG_PROTOTYPE: WBEM_QUERY_FLAG_TYPE = 2;
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = 0;
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = 1;
pub const WBEM_FLAG_REFS_ONLY: WBEM_CONDITION_FLAG_TYPE = 8;
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = 0;
pub const WBEM_FLAG_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = 16;
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: WBEM_GENERIC_FLAG_TYPE = 0;
pub const WBEM_FLAG_SEND_ONLY_SELECTED: WBEM_GENERIC_FLAG_TYPE = 0;
pub const WBEM_FLAG_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = 128;
pub const WBEM_FLAG_SHALLOW: WBEM_QUERY_FLAG_TYPE = 1;
pub const WBEM_FLAG_SHORT_NAME: WBEM_INFORMATION_FLAG_TYPE = 1;
pub const WBEM_FLAG_SPLIT_FILES: WBEM_COMPILER_OPTIONS = 32;
pub const WBEM_FLAG_STORE_FILE: WBEM_COMPILER_OPTIONS = 256;
pub const WBEM_FLAG_STRONG_VALIDATION: WBEM_GENERIC_FLAG_TYPE = 1048576;
pub const WBEM_FLAG_SYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = 48;
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 1;
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 0;
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 2;
pub const WBEM_FLAG_UPDATE_COMPATIBLE: WBEM_CHANGE_FLAG_TYPE = 0;
pub const WBEM_FLAG_UPDATE_FORCE_MODE: WBEM_CHANGE_FLAG_TYPE = 64;
pub const WBEM_FLAG_UPDATE_ONLY: WBEM_CHANGE_FLAG_TYPE = 1;
pub const WBEM_FLAG_UPDATE_SAFE_MODE: WBEM_CHANGE_FLAG_TYPE = 32;
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: WBEM_GENERIC_FLAG_TYPE = 131072;
pub const WBEM_FLAG_WMI_CHECK: WBEM_COMPILER_OPTIONS = 4;
pub const WBEM_FLAVOR_AMENDED: WBEM_FLAVOR_TYPE = 128;
pub const WBEM_FLAVOR_DONT_PROPAGATE: WBEM_FLAVOR_TYPE = 0;
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: WBEM_FLAVOR_TYPE = 2;
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: WBEM_FLAVOR_TYPE = 1;
pub const WBEM_FLAVOR_MASK_AMENDED: WBEM_FLAVOR_TYPE = 128;
pub const WBEM_FLAVOR_MASK_ORIGIN: WBEM_FLAVOR_TYPE = 96;
pub const WBEM_FLAVOR_MASK_PERMISSIONS: WBEM_FLAVOR_TYPE = 16;
pub const WBEM_FLAVOR_MASK_PROPAGATION: WBEM_FLAVOR_TYPE = 15;
pub const WBEM_FLAVOR_NOT_AMENDED: WBEM_FLAVOR_TYPE = 0;
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: WBEM_FLAVOR_TYPE = 16;
pub const WBEM_FLAVOR_ORIGIN_LOCAL: WBEM_FLAVOR_TYPE = 0;
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: WBEM_FLAVOR_TYPE = 32;
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: WBEM_FLAVOR_TYPE = 64;
pub const WBEM_FLAVOR_OVERRIDABLE: WBEM_FLAVOR_TYPE = 0;
pub type WBEM_FLAVOR_TYPE = i32;
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = 4;
pub type WBEM_GENERIC_FLAG_TYPE = i32;
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = 1;
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = 2;
pub type WBEM_GENUS_TYPE = i32;
pub const WBEM_INFINITE: WBEM_TIMEOUT_TYPE = -1;
pub type WBEM_INFORMATION_FLAG_TYPE = i32;
pub type WBEM_LIMITATION_FLAG_TYPE = i32;
pub type WBEM_LIMITS = i32;
pub type WBEM_LOCKING_FLAG_TYPE = i32;
pub const WBEM_MASK_CLASS_CONDITION: WBEM_CONDITION_FLAG_TYPE = 768;
pub const WBEM_MASK_CONDITION_ORIGIN: WBEM_CONDITION_FLAG_TYPE = 112;
pub const WBEM_MASK_PRIMARY_CONDITION: WBEM_CONDITION_FLAG_TYPE = 3;
pub const WBEM_MASK_RESERVED_FLAGS: WBEM_GENERIC_FLAG_TYPE = 126976;
pub const WBEM_MASK_UPDATE_MODE: WBEM_CHANGE_FLAG_TYPE = 96;
pub const WBEM_MAX_IDENTIFIER: WBEM_LIMITS = 4096;
pub const WBEM_MAX_OBJECT_NESTING: WBEM_LIMITS = 64;
pub const WBEM_MAX_PATH: WBEM_LIMITS = 8192;
pub const WBEM_MAX_QUERY: WBEM_LIMITS = 16384;
pub const WBEM_MAX_USER_PROPERTIES: WBEM_LIMITS = 1024;
pub const WBEM_METHOD_EXECUTE: WBEM_SECURITY_FLAGS = 2;
pub const WBEM_NO_ERROR: WBEMSTATUS = 0;
pub const WBEM_NO_WAIT: WBEM_TIMEOUT_TYPE = 0;
pub const WBEM_PARTIAL_WRITE_REP: WBEM_SECURITY_FLAGS = 8;
pub type WBEM_QUERY_FLAG_TYPE = i32;
pub type WBEM_REFRESHER_FLAGS = i32;
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = 32;
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = 16;
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = 0;
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = 128;
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = 64;
pub type WBEM_SECURITY_FLAGS = i32;
pub type WBEM_SHUTDOWN_FLAGS = i32;
pub const WBEM_SHUTDOWN_OS: WBEM_SHUTDOWN_FLAGS = 3;
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: WBEM_SHUTDOWN_FLAGS = 1;
pub const WBEM_SHUTDOWN_WMI: WBEM_SHUTDOWN_FLAGS = 2;
pub const WBEM_STATUS_COMPLETE: WBEM_STATUS_TYPE = 0;
pub const WBEM_STATUS_LOGGING_INFORMATION: WBEM_STATUS_TYPE = 256;
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: WBEM_STATUS_TYPE = 4096;
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: WBEM_STATUS_TYPE = 1024;
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: WBEM_STATUS_TYPE = 512;
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: WBEM_STATUS_TYPE = 2048;
pub const WBEM_STATUS_PROGRESS: WBEM_STATUS_TYPE = 2;
pub const WBEM_STATUS_REQUIREMENTS: WBEM_STATUS_TYPE = 1;
pub type WBEM_STATUS_TYPE = i32;
pub const WBEM_S_ACCESS_DENIED: WBEMSTATUS = 262153;
pub const WBEM_S_ALREADY_EXISTS: WBEMSTATUS = 262145;
pub const WBEM_S_DIFFERENT: WBEMSTATUS = 262147;
pub const WBEM_S_DUPLICATE_OBJECTS: WBEMSTATUS = 262152;
pub const WBEM_S_FALSE: WBEMSTATUS = 1;
pub const WBEM_S_NO_ERROR: WBEMSTATUS = 0;
pub const WBEM_S_NO_MORE_DATA: WBEMSTATUS = 262149;
pub const WBEM_S_OPERATION_CANCELLED: WBEMSTATUS = 262150;
pub const WBEM_S_PARTIAL_RESULTS: WBEMSTATUS = 262160;
pub const WBEM_S_PENDING: WBEMSTATUS = 262151;
pub const WBEM_S_RESET_TO_DEFAULT: WBEMSTATUS = 262146;
pub const WBEM_S_SAME: WBEMSTATUS = 0;
pub const WBEM_S_SOURCE_NOT_AVAILABLE: WBEMSTATUS = 262167;
pub const WBEM_S_TIMEDOUT: WBEMSTATUS = 262148;
pub type WBEM_TEXT_FLAG_TYPE = i32;
pub type WBEM_TIMEOUT_TYPE = i32;
pub type WBEM_UNSECAPP_FLAG_TYPE = i32;
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = 16;
pub type WMI_OBJ_TEXT = i32;
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: WMI_OBJ_TEXT = 1;
pub const WMI_OBJ_TEXT_LAST: WMI_OBJ_TEXT = 13;
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: WMI_OBJ_TEXT = 2;
pub const WMI_OBJ_TEXT_WMI_EXT1: WMI_OBJ_TEXT = 3;
pub const WMI_OBJ_TEXT_WMI_EXT10: WMI_OBJ_TEXT = 12;
pub const WMI_OBJ_TEXT_WMI_EXT2: WMI_OBJ_TEXT = 4;
pub const WMI_OBJ_TEXT_WMI_EXT3: WMI_OBJ_TEXT = 5;
pub const WMI_OBJ_TEXT_WMI_EXT4: WMI_OBJ_TEXT = 6;
pub const WMI_OBJ_TEXT_WMI_EXT5: WMI_OBJ_TEXT = 7;
pub const WMI_OBJ_TEXT_WMI_EXT6: WMI_OBJ_TEXT = 8;
pub const WMI_OBJ_TEXT_WMI_EXT7: WMI_OBJ_TEXT = 9;
pub const WMI_OBJ_TEXT_WMI_EXT8: WMI_OBJ_TEXT = 10;
pub const WMI_OBJ_TEXT_WMI_EXT9: WMI_OBJ_TEXT = 11;
pub const WbemBackupRestore: windows_core::GUID = windows_core::GUID::from_u128(0xc49e32c6_bc8b_11d2_85d4_00105a1f8304);
pub const WbemClassObject: windows_core::GUID = windows_core::GUID::from_u128(0x9a653086_174f_11d2_b5f9_00104b703efd);
pub const WbemContext: windows_core::GUID = windows_core::GUID::from_u128(0x674b6698_ee92_11d0_ad71_00c04fd8fdff);
pub const WbemLocator: windows_core::GUID = windows_core::GUID::from_u128(0x4590f811_1d3a_11d0_891f_00aa004b2e24);
pub const WbemObjectTextSrc: windows_core::GUID = windows_core::GUID::from_u128(0x8d1c559d_84f0_4bb3_a7d5_56a7435a9ba6);
pub const WbemRefresher: windows_core::GUID = windows_core::GUID::from_u128(0xc71566f2_561e_11d1_ad87_00c04fd8fdff);
pub const WbemStatusCodeText: windows_core::GUID = windows_core::GUID::from_u128(0xeb87e1bd_3233_11d2_aec9_00c04fb68820);
