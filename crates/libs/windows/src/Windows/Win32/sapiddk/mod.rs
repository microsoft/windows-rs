windows_core::imp::define_interface!(ISpCFGInterpreter, ISpCFGInterpreter_Vtbl, 0xf3d3f926_11fc_11d3_bb97_00c04f8ee6c0);
windows_core::imp::interface_hierarchy!(ISpCFGInterpreter, windows_core::IUnknown);
impl ISpCFGInterpreter {
    pub unsafe fn InitGrammar<P0>(&self, pszgrammarname: P0, pvgrammardata: *const *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitGrammar)(windows_core::Interface::as_raw(self), pszgrammarname.param().abi(), pvgrammardata) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn Interpret<P0, P3>(&self, pphrase: P0, ulfirstelement: super::ULONG, ulcountofelements: super::ULONG, psite: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpPhraseBuilder>,
        P3: windows_core::Param<ISpCFGInterpreterSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).Interpret)(windows_core::Interface::as_raw(self), pphrase.param().abi(), ulfirstelement, ulcountofelements, psite.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpCFGInterpreter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub Interpret: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, super::ULONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    Interpret: usize,
}
#[cfg(all(feature = "sapi", feature = "shared"))]
pub trait ISpCFGInterpreter_Impl: windows_core::IUnknownImpl {
    fn InitGrammar(&self, pszgrammarname: &windows_core::PCWSTR, pvgrammardata: *const *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Interpret(&self, pphrase: windows_core::Ref<ISpPhraseBuilder>, ulfirstelement: super::ULONG, ulcountofelements: super::ULONG, psite: windows_core::Ref<ISpCFGInterpreterSite>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl ISpCFGInterpreter_Vtbl {
    pub const fn new<Identity: ISpCFGInterpreter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitGrammar<Identity: ISpCFGInterpreter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgrammarname: windows_core::PCWSTR, pvgrammardata: *const *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpCFGInterpreter_Impl::InitGrammar(this, core::mem::transmute(&pszgrammarname), core::mem::transmute_copy(&pvgrammardata)).into()
            }
        }
        unsafe extern "system" fn Interpret<Identity: ISpCFGInterpreter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void, ulfirstelement: super::ULONG, ulcountofelements: super::ULONG, psite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpCFGInterpreter_Impl::Interpret(this, core::mem::transmute_copy(&pphrase), core::mem::transmute_copy(&ulfirstelement), core::mem::transmute_copy(&ulcountofelements), core::mem::transmute_copy(&psite)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitGrammar: InitGrammar::<Identity, OFFSET>,
            Interpret: Interpret::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpCFGInterpreter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl windows_core::RuntimeName for ISpCFGInterpreter {}
windows_core::imp::define_interface!(ISpCFGInterpreterSite, ISpCFGInterpreterSite_Vtbl, 0x6a6ffad8_78b6_473d_b844_98152e4fb16b);
windows_core::imp::interface_hierarchy!(ISpCFGInterpreterSite, windows_core::IUnknown);
impl ISpCFGInterpreterSite {
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn AddTextReplacement(&self, preplace: *const super::SPPHRASEREPLACEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTextReplacement)(windows_core::Interface::as_raw(self), preplace) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddProperty(&self, pproperty: *const super::SPPHRASEPROPERTY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), pproperty) }
    }
    pub unsafe fn GetResourceValue<P0>(&self, pszresourcename: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceValue)(windows_core::Interface::as_raw(self), pszresourcename.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpCFGInterpreterSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub AddTextReplacement: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SPPHRASEREPLACEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    AddTextReplacement: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SPPHRASEPROPERTY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    AddProperty: usize,
    pub GetResourceValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISpCFGInterpreterSite_Impl: windows_core::IUnknownImpl {
    fn AddTextReplacement(&self, preplace: *const super::SPPHRASEREPLACEMENT) -> windows_core::Result<()>;
    fn AddProperty(&self, pproperty: *const super::SPPHRASEPROPERTY) -> windows_core::Result<()>;
    fn GetResourceValue(&self, pszresourcename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISpCFGInterpreterSite_Vtbl {
    pub const fn new<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTextReplacement<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplace: *const super::SPPHRASEREPLACEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpCFGInterpreterSite_Impl::AddTextReplacement(this, core::mem::transmute_copy(&preplace)).into()
            }
        }
        unsafe extern "system" fn AddProperty<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperty: *const super::SPPHRASEPROPERTY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpCFGInterpreterSite_Impl::AddProperty(this, core::mem::transmute_copy(&pproperty)).into()
            }
        }
        unsafe extern "system" fn GetResourceValue<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszresourcename: windows_core::PCWSTR, ppcomemresource: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpCFGInterpreterSite_Impl::GetResourceValue(this, core::mem::transmute(&pszresourcename)) {
                    Ok(ok__) => {
                        ppcomemresource.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTextReplacement: AddTextReplacement::<Identity, OFFSET>,
            AddProperty: AddProperty::<Identity, OFFSET>,
            GetResourceValue: GetResourceValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpCFGInterpreterSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISpCFGInterpreterSite {}
windows_core::imp::define_interface!(ISpErrorLog, ISpErrorLog_Vtbl, 0xf4711347_e608_11d2_a086_00c04f8ef9b5);
windows_core::imp::interface_hierarchy!(ISpErrorLog, windows_core::IUnknown);
impl ISpErrorLog {
    #[cfg(feature = "shared")]
    pub unsafe fn AddError<P2, P3>(&self, llinenumber: i32, hr: windows_core::HRESULT, pszdescription: P2, pszhelpfile: P3, dwhelpcontext: super::DWORD) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddError)(windows_core::Interface::as_raw(self), llinenumber, hr, pszdescription.param().abi(), pszhelpfile.param().abi(), dwhelpcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpErrorLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub AddError: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::HRESULT, windows_core::PCWSTR, windows_core::PCWSTR, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddError: usize,
}
#[cfg(feature = "shared")]
pub trait ISpErrorLog_Impl: windows_core::IUnknownImpl {
    fn AddError(&self, llinenumber: i32, hr: windows_core::HRESULT, pszdescription: &windows_core::PCWSTR, pszhelpfile: &windows_core::PCWSTR, dwhelpcontext: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ISpErrorLog_Vtbl {
    pub const fn new<Identity: ISpErrorLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddError<Identity: ISpErrorLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llinenumber: i32, hr: windows_core::HRESULT, pszdescription: windows_core::PCWSTR, pszhelpfile: windows_core::PCWSTR, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpErrorLog_Impl::AddError(this, core::mem::transmute_copy(&llinenumber), core::mem::transmute_copy(&hr), core::mem::transmute(&pszdescription), core::mem::transmute(&pszhelpfile), core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddError: AddError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpErrorLog as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ISpErrorLog {}
#[cfg(feature = "sapi")]
windows_core::imp::define_interface!(ISpGramCompBackend, ISpGramCompBackend_Vtbl, 0x3ddca27c_665c_4786_9f97_8c90c3488b61);
#[cfg(feature = "sapi")]
impl core::ops::Deref for ISpGramCompBackend {
    type Target = super::ISpGrammarBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "sapi")]
windows_core::imp::interface_hierarchy!(ISpGramCompBackend, windows_core::IUnknown, super::ISpGrammarBuilder);
#[cfg(feature = "sapi")]
impl ISpGramCompBackend {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SetSaveObjects<P0, P1>(&self, pstream: P0, perrorlog: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
        P1: windows_core::Param<ISpErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSaveObjects)(windows_core::Interface::as_raw(self), pstream.param().abi(), perrorlog.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn InitFromBinaryGrammar(&self, pbinarydata: *const super::SPBINARYGRAMMAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitFromBinaryGrammar)(windows_core::Interface::as_raw(self), pbinarydata) }
    }
}
#[cfg(feature = "sapi")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpGramCompBackend_Vtbl {
    pub base__: super::ISpGrammarBuilder_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub SetSaveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SetSaveObjects: usize,
    #[cfg(feature = "shared")]
    pub InitFromBinaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SPBINARYGRAMMAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    InitFromBinaryGrammar: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISpGramCompBackend_Impl: super::ISpGrammarBuilder_Impl {
    fn SetSaveObjects(&self, pstream: windows_core::Ref<super::IStream>, perrorlog: windows_core::Ref<ISpErrorLog>) -> windows_core::Result<()>;
    fn InitFromBinaryGrammar(&self, pbinarydata: *const super::SPBINARYGRAMMAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISpGramCompBackend_Vtbl {
    pub const fn new<Identity: ISpGramCompBackend_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSaveObjects<Identity: ISpGramCompBackend_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpGramCompBackend_Impl::SetSaveObjects(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&perrorlog)).into()
            }
        }
        unsafe extern "system" fn InitFromBinaryGrammar<Identity: ISpGramCompBackend_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinarydata: *const super::SPBINARYGRAMMAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpGramCompBackend_Impl::InitFromBinaryGrammar(this, core::mem::transmute_copy(&pbinarydata)).into()
            }
        }
        Self {
            base__: super::ISpGrammarBuilder_Vtbl::new::<Identity, OFFSET>(),
            SetSaveObjects: SetSaveObjects::<Identity, OFFSET>,
            InitFromBinaryGrammar: InitFromBinaryGrammar::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpGramCompBackend as windows_core::Interface>::IID || iid == &<super::ISpGrammarBuilder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISpGramCompBackend {}
windows_core::imp::define_interface!(ISpGrammarCompiler, ISpGrammarCompiler_Vtbl, 0xb1e29d58_a675_11d2_8302_00c04f8ee6c0);
windows_core::imp::interface_hierarchy!(ISpGrammarCompiler, windows_core::IUnknown);
impl ISpGrammarCompiler {
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn CompileStream<P0, P1, P2, P3, P4>(&self, psource: P0, pdest: P1, pheader: P2, preserved: P3, perrorlog: P4, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
        P1: windows_core::Param<super::IStream>,
        P2: windows_core::Param<super::IStream>,
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<ISpErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompileStream)(windows_core::Interface::as_raw(self), psource.param().abi(), pdest.param().abi(), pheader.param().abi(), preserved.param().abi(), perrorlog.param().abi(), dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpGrammarCompiler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub CompileStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    CompileStream: usize,
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
pub trait ISpGrammarCompiler_Impl: windows_core::IUnknownImpl {
    fn CompileStream(&self, psource: windows_core::Ref<super::IStream>, pdest: windows_core::Ref<super::IStream>, pheader: windows_core::Ref<super::IStream>, preserved: windows_core::Ref<windows_core::IUnknown>, perrorlog: windows_core::Ref<ISpErrorLog>, dwflags: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl ISpGrammarCompiler_Vtbl {
    pub const fn new<Identity: ISpGrammarCompiler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompileStream<Identity: ISpGrammarCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, pdest: *mut core::ffi::c_void, pheader: *mut core::ffi::c_void, preserved: *mut core::ffi::c_void, perrorlog: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpGrammarCompiler_Impl::CompileStream(this, core::mem::transmute_copy(&psource), core::mem::transmute_copy(&pdest), core::mem::transmute_copy(&pheader), core::mem::transmute_copy(&preserved), core::mem::transmute_copy(&perrorlog), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CompileStream: CompileStream::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpGrammarCompiler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl windows_core::RuntimeName for ISpGrammarCompiler {}
windows_core::imp::define_interface!(ISpITNProcessor, ISpITNProcessor_Vtbl, 0x12d7360f_a1c9_11d3_bc90_00c04f72df9f);
windows_core::imp::interface_hierarchy!(ISpITNProcessor, windows_core::IUnknown);
impl ISpITNProcessor {
    pub unsafe fn LoadITNGrammar<P0>(&self, pszclsid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadITNGrammar)(windows_core::Interface::as_raw(self), pszclsid.param().abi()) }
    }
    #[cfg(feature = "sapi")]
    pub unsafe fn ITNPhrase<P0>(&self, pphrase: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpPhraseBuilder>,
    {
        unsafe { (windows_core::Interface::vtable(self).ITNPhrase)(windows_core::Interface::as_raw(self), pphrase.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpITNProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadITNGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "sapi")]
    pub ITNPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "sapi"))]
    ITNPhrase: usize,
}
#[cfg(feature = "sapi")]
pub trait ISpITNProcessor_Impl: windows_core::IUnknownImpl {
    fn LoadITNGrammar(&self, pszclsid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ITNPhrase(&self, pphrase: windows_core::Ref<ISpPhraseBuilder>) -> windows_core::Result<()>;
}
#[cfg(feature = "sapi")]
impl ISpITNProcessor_Vtbl {
    pub const fn new<Identity: ISpITNProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadITNGrammar<Identity: ISpITNProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclsid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpITNProcessor_Impl::LoadITNGrammar(this, core::mem::transmute(&pszclsid)).into()
            }
        }
        unsafe extern "system" fn ITNPhrase<Identity: ISpITNProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpITNProcessor_Impl::ITNPhrase(this, core::mem::transmute_copy(&pphrase)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadITNGrammar: LoadITNGrammar::<Identity, OFFSET>,
            ITNPhrase: ITNPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpITNProcessor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "sapi")]
impl windows_core::RuntimeName for ISpITNProcessor {}
#[cfg(feature = "sapi")]
windows_core::imp::define_interface!(ISpObjectTokenEnumBuilder, ISpObjectTokenEnumBuilder_Vtbl, 0x06b64f9f_7fda_11d2_b4f2_00c04f797396);
#[cfg(feature = "sapi")]
impl core::ops::Deref for ISpObjectTokenEnumBuilder {
    type Target = super::IEnumSpObjectTokens;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "sapi")]
windows_core::imp::interface_hierarchy!(ISpObjectTokenEnumBuilder, windows_core::IUnknown, super::IEnumSpObjectTokens);
#[cfg(feature = "sapi")]
impl ISpObjectTokenEnumBuilder {
    pub unsafe fn SetAttribs<P0, P1>(&self, pszreqattribs: P0, pszoptattribs: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAttribs)(windows_core::Interface::as_raw(self), pszreqattribs.param().abi(), pszoptattribs.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddTokens(&self, ctokens: super::ULONG, ptoken: *const Option<super::ISpObjectToken>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTokens)(windows_core::Interface::as_raw(self), ctokens, core::mem::transmute(ptoken)) }
    }
    pub unsafe fn AddTokensFromDataKey<P0, P1, P2>(&self, pdatakey: P0, pszsubkey: P1, pszcategoryid: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ISpDataKey>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTokensFromDataKey)(windows_core::Interface::as_raw(self), pdatakey.param().abi(), pszsubkey.param().abi(), pszcategoryid.param().abi()) }
    }
    pub unsafe fn AddTokensFromTokenEnum<P0>(&self, ptokenenum: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IEnumSpObjectTokens>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTokensFromTokenEnum)(windows_core::Interface::as_raw(self), ptokenenum.param().abi()) }
    }
    pub unsafe fn Sort<P0>(&self, psztokenidtolistfirst: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Sort)(windows_core::Interface::as_raw(self), psztokenidtolistfirst.param().abi()) }
    }
}
#[cfg(feature = "sapi")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpObjectTokenEnumBuilder_Vtbl {
    pub base__: super::IEnumSpObjectTokens_Vtbl,
    pub SetAttribs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddTokens: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddTokens: usize,
    pub AddTokensFromDataKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddTokensFromTokenEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "sapi", feature = "shared"))]
pub trait ISpObjectTokenEnumBuilder_Impl: super::IEnumSpObjectTokens_Impl {
    fn SetAttribs(&self, pszreqattribs: &windows_core::PCWSTR, pszoptattribs: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddTokens(&self, ctokens: super::ULONG, ptoken: *const Option<super::ISpObjectToken>) -> windows_core::Result<()>;
    fn AddTokensFromDataKey(&self, pdatakey: windows_core::Ref<super::ISpDataKey>, pszsubkey: &windows_core::PCWSTR, pszcategoryid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddTokensFromTokenEnum(&self, ptokenenum: windows_core::Ref<super::IEnumSpObjectTokens>) -> windows_core::Result<()>;
    fn Sort(&self, psztokenidtolistfirst: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl ISpObjectTokenEnumBuilder_Vtbl {
    pub const fn new<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAttribs<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszreqattribs: windows_core::PCWSTR, pszoptattribs: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpObjectTokenEnumBuilder_Impl::SetAttribs(this, core::mem::transmute(&pszreqattribs), core::mem::transmute(&pszoptattribs)).into()
            }
        }
        unsafe extern "system" fn AddTokens<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ctokens: super::ULONG, ptoken: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpObjectTokenEnumBuilder_Impl::AddTokens(this, core::mem::transmute_copy(&ctokens), core::mem::transmute_copy(&ptoken)).into()
            }
        }
        unsafe extern "system" fn AddTokensFromDataKey<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatakey: *mut core::ffi::c_void, pszsubkey: windows_core::PCWSTR, pszcategoryid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpObjectTokenEnumBuilder_Impl::AddTokensFromDataKey(this, core::mem::transmute_copy(&pdatakey), core::mem::transmute(&pszsubkey), core::mem::transmute(&pszcategoryid)).into()
            }
        }
        unsafe extern "system" fn AddTokensFromTokenEnum<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptokenenum: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpObjectTokenEnumBuilder_Impl::AddTokensFromTokenEnum(this, core::mem::transmute_copy(&ptokenenum)).into()
            }
        }
        unsafe extern "system" fn Sort<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztokenidtolistfirst: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpObjectTokenEnumBuilder_Impl::Sort(this, core::mem::transmute(&psztokenidtolistfirst)).into()
            }
        }
        Self {
            base__: super::IEnumSpObjectTokens_Vtbl::new::<Identity, OFFSET>(),
            SetAttribs: SetAttribs::<Identity, OFFSET>,
            AddTokens: AddTokens::<Identity, OFFSET>,
            AddTokensFromDataKey: AddTokensFromDataKey::<Identity, OFFSET>,
            AddTokensFromTokenEnum: AddTokensFromTokenEnum::<Identity, OFFSET>,
            Sort: Sort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpObjectTokenEnumBuilder as windows_core::Interface>::IID || iid == &<super::IEnumSpObjectTokens as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl windows_core::RuntimeName for ISpObjectTokenEnumBuilder {}
#[cfg(feature = "sapi")]
windows_core::imp::define_interface!(ISpPhraseBuilder, ISpPhraseBuilder_Vtbl, 0x88a3342a_0bed_4834_922b_88d43173162f);
#[cfg(feature = "sapi")]
impl core::ops::Deref for ISpPhraseBuilder {
    type Target = super::ISpPhrase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "sapi")]
windows_core::imp::interface_hierarchy!(ISpPhraseBuilder, windows_core::IUnknown, super::ISpPhrase);
#[cfg(feature = "sapi")]
impl ISpPhraseBuilder {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InitFromPhrase(&self, pphrase: *const super::SPPHRASE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitFromPhrase)(windows_core::Interface::as_raw(self), pphrase) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn InitFromSerializedPhrase(&self, pphrase: *const super::SPSERIALIZEDPHRASE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitFromSerializedPhrase)(windows_core::Interface::as_raw(self), pphrase) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddElements(&self, celements: super::ULONG, pelement: *const super::SPPHRASEELEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddElements)(windows_core::Interface::as_raw(self), celements, pelement) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddRules(&self, hparent: SPPHRASERULEHANDLE, prule: *const super::SPPHRASERULE) -> windows_core::Result<SPPHRASERULEHANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddRules)(windows_core::Interface::as_raw(self), hparent, prule, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddProperties(&self, hparent: SPPHRASEPROPERTYHANDLE, pproperty: *const super::SPPHRASEPROPERTY) -> windows_core::Result<SPPHRASEPROPERTYHANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddProperties)(windows_core::Interface::as_raw(self), hparent, pproperty, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddReplacements(&self, creplacements: super::ULONG, preplacements: *const super::SPPHRASEREPLACEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddReplacements)(windows_core::Interface::as_raw(self), creplacements, preplacements) }
    }
}
#[cfg(feature = "sapi")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpPhraseBuilder_Vtbl {
    pub base__: super::ISpPhrase_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub InitFromPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SPPHRASE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    InitFromPhrase: usize,
    #[cfg(feature = "shared")]
    pub InitFromSerializedPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SPSERIALIZEDPHRASE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    InitFromSerializedPhrase: usize,
    #[cfg(feature = "shared")]
    pub AddElements: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::SPPHRASEELEMENT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddElements: usize,
    #[cfg(feature = "shared")]
    pub AddRules: unsafe extern "system" fn(*mut core::ffi::c_void, SPPHRASERULEHANDLE, *const super::SPPHRASERULE, *mut SPPHRASERULEHANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddRules: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub AddProperties: unsafe extern "system" fn(*mut core::ffi::c_void, SPPHRASEPROPERTYHANDLE, *const super::SPPHRASEPROPERTY, *mut SPPHRASEPROPERTYHANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    AddProperties: usize,
    #[cfg(feature = "shared")]
    pub AddReplacements: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::SPPHRASEREPLACEMENT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddReplacements: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISpPhraseBuilder_Impl: super::ISpPhrase_Impl {
    fn InitFromPhrase(&self, pphrase: *const super::SPPHRASE) -> windows_core::Result<()>;
    fn InitFromSerializedPhrase(&self, pphrase: *const super::SPSERIALIZEDPHRASE) -> windows_core::Result<()>;
    fn AddElements(&self, celements: super::ULONG, pelement: *const super::SPPHRASEELEMENT) -> windows_core::Result<()>;
    fn AddRules(&self, hparent: SPPHRASERULEHANDLE, prule: *const super::SPPHRASERULE) -> windows_core::Result<SPPHRASERULEHANDLE>;
    fn AddProperties(&self, hparent: SPPHRASEPROPERTYHANDLE, pproperty: *const super::SPPHRASEPROPERTY) -> windows_core::Result<SPPHRASEPROPERTYHANDLE>;
    fn AddReplacements(&self, creplacements: super::ULONG, preplacements: *const super::SPPHRASEREPLACEMENT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISpPhraseBuilder_Vtbl {
    pub const fn new<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitFromPhrase<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *const super::SPPHRASE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpPhraseBuilder_Impl::InitFromPhrase(this, core::mem::transmute_copy(&pphrase)).into()
            }
        }
        unsafe extern "system" fn InitFromSerializedPhrase<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *const super::SPSERIALIZEDPHRASE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpPhraseBuilder_Impl::InitFromSerializedPhrase(this, core::mem::transmute_copy(&pphrase)).into()
            }
        }
        unsafe extern "system" fn AddElements<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: super::ULONG, pelement: *const super::SPPHRASEELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpPhraseBuilder_Impl::AddElements(this, core::mem::transmute_copy(&celements), core::mem::transmute_copy(&pelement)).into()
            }
        }
        unsafe extern "system" fn AddRules<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: SPPHRASERULEHANDLE, prule: *const super::SPPHRASERULE, phnewrule: *mut SPPHRASERULEHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpPhraseBuilder_Impl::AddRules(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&prule)) {
                    Ok(ok__) => {
                        phnewrule.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddProperties<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: SPPHRASEPROPERTYHANDLE, pproperty: *const super::SPPHRASEPROPERTY, phnewproperty: *mut SPPHRASEPROPERTYHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpPhraseBuilder_Impl::AddProperties(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&pproperty)) {
                    Ok(ok__) => {
                        phnewproperty.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddReplacements<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creplacements: super::ULONG, preplacements: *const super::SPPHRASEREPLACEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpPhraseBuilder_Impl::AddReplacements(this, core::mem::transmute_copy(&creplacements), core::mem::transmute_copy(&preplacements)).into()
            }
        }
        Self {
            base__: super::ISpPhrase_Vtbl::new::<Identity, OFFSET>(),
            InitFromPhrase: InitFromPhrase::<Identity, OFFSET>,
            InitFromSerializedPhrase: InitFromSerializedPhrase::<Identity, OFFSET>,
            AddElements: AddElements::<Identity, OFFSET>,
            AddRules: AddRules::<Identity, OFFSET>,
            AddProperties: AddProperties::<Identity, OFFSET>,
            AddReplacements: AddReplacements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhraseBuilder as windows_core::Interface>::IID || iid == &<super::ISpPhrase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "rpc", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISpPhraseBuilder {}
windows_core::imp::define_interface!(ISpPrivateEngineCallEx, ISpPrivateEngineCallEx_Vtbl, 0xdefd682a_fe0a_42b9_bfa1_56d3d6cecfaf);
windows_core::imp::interface_hierarchy!(ISpPrivateEngineCallEx, windows_core::IUnknown);
impl ISpPrivateEngineCallEx {
    #[cfg(feature = "shared")]
    pub unsafe fn CallEngineSynchronize(&self, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CallEngineSynchronize)(windows_core::Interface::as_raw(self), pinframe, ulinframesize, ppcomemoutframe as _, puloutframesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CallEngineImmediate(&self, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CallEngineImmediate)(windows_core::Interface::as_raw(self), pinframe, ulinframesize, ppcomemoutframe as _, puloutframesize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpPrivateEngineCallEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub CallEngineSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CallEngineSynchronize: usize,
    #[cfg(feature = "shared")]
    pub CallEngineImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CallEngineImmediate: usize,
}
#[cfg(feature = "shared")]
pub trait ISpPrivateEngineCallEx_Impl: windows_core::IUnknownImpl {
    fn CallEngineSynchronize(&self, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::Result<()>;
    fn CallEngineImmediate(&self, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ISpPrivateEngineCallEx_Vtbl {
    pub const fn new<Identity: ISpPrivateEngineCallEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallEngineSynchronize<Identity: ISpPrivateEngineCallEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpPrivateEngineCallEx_Impl::CallEngineSynchronize(this, core::mem::transmute_copy(&pinframe), core::mem::transmute_copy(&ulinframesize), core::mem::transmute_copy(&ppcomemoutframe), core::mem::transmute_copy(&puloutframesize)).into()
            }
        }
        unsafe extern "system" fn CallEngineImmediate<Identity: ISpPrivateEngineCallEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpPrivateEngineCallEx_Impl::CallEngineImmediate(this, core::mem::transmute_copy(&pinframe), core::mem::transmute_copy(&ulinframesize), core::mem::transmute_copy(&ppcomemoutframe), core::mem::transmute_copy(&puloutframesize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CallEngineSynchronize: CallEngineSynchronize::<Identity, OFFSET>,
            CallEngineImmediate: CallEngineImmediate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPrivateEngineCallEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ISpPrivateEngineCallEx {}
windows_core::imp::define_interface!(ISpSRAlternates, ISpSRAlternates_Vtbl, 0xfece8294_2be1_408f_8e68_2de377092f0e);
windows_core::imp::interface_hierarchy!(ISpSRAlternates, windows_core::IUnknown);
impl ISpSRAlternates {
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn GetAlternates(&self, paltrequest: *const SPPHRASEALTREQUEST, ppalts: *mut *mut SPPHRASEALT, pcalts: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAlternates)(windows_core::Interface::as_raw(self), paltrequest, ppalts as _, pcalts as _) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn Commit(&self, paltrequest: *const SPPHRASEALTREQUEST, palt: *const SPPHRASEALT, ppvresultextra: *mut *mut core::ffi::c_void, pcbresultextra: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), paltrequest, palt, ppvresultextra as _, pcbresultextra as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpSRAlternates_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub GetAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEALTREQUEST, *mut *mut SPPHRASEALT, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    GetAlternates: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEALTREQUEST, *const SPPHRASEALT, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    Commit: usize,
}
#[cfg(all(feature = "sapi", feature = "shared"))]
pub trait ISpSRAlternates_Impl: windows_core::IUnknownImpl {
    fn GetAlternates(&self, paltrequest: *const SPPHRASEALTREQUEST, ppalts: *mut *mut SPPHRASEALT, pcalts: *mut super::ULONG) -> windows_core::Result<()>;
    fn Commit(&self, paltrequest: *const SPPHRASEALTREQUEST, palt: *const SPPHRASEALT, ppvresultextra: *mut *mut core::ffi::c_void, pcbresultextra: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl ISpSRAlternates_Vtbl {
    pub const fn new<Identity: ISpSRAlternates_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAlternates<Identity: ISpSRAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltrequest: *const SPPHRASEALTREQUEST, ppalts: *mut *mut SPPHRASEALT, pcalts: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSRAlternates_Impl::GetAlternates(this, core::mem::transmute_copy(&paltrequest), core::mem::transmute_copy(&ppalts), core::mem::transmute_copy(&pcalts)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: ISpSRAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltrequest: *const SPPHRASEALTREQUEST, palt: *const SPPHRASEALT, ppvresultextra: *mut *mut core::ffi::c_void, pcbresultextra: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSRAlternates_Impl::Commit(this, core::mem::transmute_copy(&paltrequest), core::mem::transmute_copy(&palt), core::mem::transmute_copy(&ppvresultextra), core::mem::transmute_copy(&pcbresultextra)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAlternates: GetAlternates::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSRAlternates as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl windows_core::RuntimeName for ISpSRAlternates {}
windows_core::imp::define_interface!(ISpSRAlternates2, ISpSRAlternates2_Vtbl, 0xf338f437_cb33_4020_9cab_c71ff9ce12d3);
impl core::ops::Deref for ISpSRAlternates2 {
    type Target = ISpSRAlternates;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSRAlternates2, windows_core::IUnknown, ISpSRAlternates);
impl ISpSRAlternates2 {
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn CommitText<P1>(&self, paltrequest: *const SPPHRASEALTREQUEST, pcsznewtext: P1, commitflags: super::SPCOMMITFLAGS) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CommitText)(windows_core::Interface::as_raw(self), paltrequest, pcsznewtext.param().abi(), commitflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpSRAlternates2_Vtbl {
    pub base__: ISpSRAlternates_Vtbl,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub CommitText: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEALTREQUEST, windows_core::PCWSTR, super::SPCOMMITFLAGS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    CommitText: usize,
}
#[cfg(all(feature = "sapi", feature = "shared"))]
pub trait ISpSRAlternates2_Impl: ISpSRAlternates_Impl {
    fn CommitText(&self, paltrequest: *const SPPHRASEALTREQUEST, pcsznewtext: &windows_core::PCWSTR, commitflags: super::SPCOMMITFLAGS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl ISpSRAlternates2_Vtbl {
    pub const fn new<Identity: ISpSRAlternates2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommitText<Identity: ISpSRAlternates2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltrequest: *const SPPHRASEALTREQUEST, pcsznewtext: windows_core::PCWSTR, commitflags: super::SPCOMMITFLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSRAlternates2_Impl::CommitText(this, core::mem::transmute_copy(&paltrequest), core::mem::transmute(&pcsznewtext), core::mem::transmute_copy(&commitflags)).into()
            }
        }
        Self { base__: ISpSRAlternates_Vtbl::new::<Identity, OFFSET>(), CommitText: CommitText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSRAlternates2 as windows_core::Interface>::IID || iid == &<ISpSRAlternates as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl windows_core::RuntimeName for ISpSRAlternates2 {}
windows_core::imp::define_interface!(ISpSREngine, ISpSREngine_Vtbl, 0x2f472991_854b_4465_b613_fbafb3ad8ed8);
windows_core::imp::interface_hierarchy!(ISpSREngine, windows_core::IUnknown);
impl ISpSREngine {
    pub unsafe fn SetSite<P0>(&self, psite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpSREngineSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSite)(windows_core::Interface::as_raw(self), psite.param().abi()) }
    }
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub unsafe fn GetInputAudioFormat(&self, pguidsourceformatid: *const windows_core::GUID, psourcewaveformatex: *const super::WAVEFORMATEX, pguiddesiredformatid: *mut windows_core::GUID, ppcomemdesiredwaveformatex: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputAudioFormat)(windows_core::Interface::as_raw(self), pguidsourceformatid, psourcewaveformatex, pguiddesiredformatid as _, ppcomemdesiredwaveformatex as _) }
    }
    #[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
    pub unsafe fn RecognizeStream<P7>(&self, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::WAVEFORMATEX, hrequestsync: super::HANDLE, hdataavailable: super::HANDLE, hexit: super::HANDLE, fnewaudiostream: bool, frealtimeaudio: bool, paudioobjecttoken: P7) -> windows_core::HRESULT
    where
        P7: windows_core::Param<super::ISpObjectToken>,
    {
        unsafe { (windows_core::Interface::vtable(self).RecognizeStream)(windows_core::Interface::as_raw(self), rguidfmtid, pwaveformatex, hrequestsync, hdataavailable, hexit, fnewaudiostream.into(), frealtimeaudio.into(), paudioobjecttoken.param().abi()) }
    }
    #[cfg(feature = "sapi")]
    pub unsafe fn SetRecoProfile<P0>(&self, pprofile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ISpObjectToken>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecoProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()) }
    }
    pub unsafe fn OnCreateGrammar(&self, pvenginerecocontext: *const core::ffi::c_void, hsapigrammar: SPGRAMMARHANDLE, ppvenginegrammarcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnCreateGrammar)(windows_core::Interface::as_raw(self), pvenginerecocontext, hsapigrammar, ppvenginegrammarcontext as _) }
    }
    pub unsafe fn OnDeleteGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDeleteGrammar)(windows_core::Interface::as_raw(self), pvenginegrammar) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn LoadProprietaryGrammar<P2>(&self, pvenginegrammar: *const core::ffi::c_void, rguidparam: *const windows_core::GUID, pszstringparam: P2, pvdataparam: *const core::ffi::c_void, uldatasize: super::ULONG, options: super::SPLOADOPTIONS) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadProprietaryGrammar)(windows_core::Interface::as_raw(self), pvenginegrammar, rguidparam, pszstringparam.param().abi(), pvdataparam, uldatasize, options) }
    }
    pub unsafe fn UnloadProprietaryGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnloadProprietaryGrammar)(windows_core::Interface::as_raw(self), pvenginegrammar) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn SetProprietaryRuleState<P1>(&self, pvenginegrammar: *const core::ffi::c_void, pszname: P1, preserved: *const core::ffi::c_void, newstate: super::SPRULESTATE) -> windows_core::Result<super::ULONG>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetProprietaryRuleState)(windows_core::Interface::as_raw(self), pvenginegrammar, pszname.param().abi(), preserved, newstate, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn SetProprietaryRuleIdState(&self, pvenginegrammar: *const core::ffi::c_void, dwruleid: super::DWORD, newstate: super::SPRULESTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProprietaryRuleIdState)(windows_core::Interface::as_raw(self), pvenginegrammar, dwruleid, newstate) }
    }
    pub unsafe fn LoadSLM<P1>(&self, pvenginegrammar: *const core::ffi::c_void, psztopicname: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadSLM)(windows_core::Interface::as_raw(self), pvenginegrammar, psztopicname.param().abi()) }
    }
    pub unsafe fn UnloadSLM(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnloadSLM)(windows_core::Interface::as_raw(self), pvenginegrammar) }
    }
    #[cfg(feature = "sapi")]
    pub unsafe fn SetSLMState(&self, pvenginegrammar: *const core::ffi::c_void, newstate: super::SPRULESTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSLMState)(windows_core::Interface::as_raw(self), pvenginegrammar, newstate) }
    }
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub unsafe fn SetWordSequenceData(&self, pvenginegrammar: *const core::ffi::c_void, ptext: &[super::WCHAR], pinfo: *const super::SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWordSequenceData)(windows_core::Interface::as_raw(self), pvenginegrammar, ptext.as_ptr(), ptext.len().try_into().unwrap(), pinfo) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn SetTextSelection(&self, pvenginegrammar: *const core::ffi::c_void, pinfo: *const super::SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextSelection)(windows_core::Interface::as_raw(self), pvenginegrammar, pinfo) }
    }
    #[cfg(feature = "sapi")]
    pub unsafe fn IsPronounceable<P1>(&self, pvenginegrammar: *const core::ffi::c_void, pszword: P1) -> windows_core::Result<super::SPWORDPRONOUNCEABLE>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPronounceable)(windows_core::Interface::as_raw(self), pvenginegrammar, pszword.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnCreateRecoContext(&self, hsapirecocontext: SPRECOCONTEXTHANDLE, ppvenginecontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnCreateRecoContext)(windows_core::Interface::as_raw(self), hsapirecocontext, ppvenginecontext as _) }
    }
    pub unsafe fn OnDeleteRecoContext(&self, pvenginecontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDeleteRecoContext)(windows_core::Interface::as_raw(self), pvenginecontext) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn PrivateCall(&self, pvenginecontext: *const core::ffi::c_void, pcallframe: super::PVOID, ulcallframesize: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrivateCall)(windows_core::Interface::as_raw(self), pvenginecontext, pcallframe as _, ulcallframesize) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetAdaptationData(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: &[super::WCHAR]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAdaptationData)(windows_core::Interface::as_raw(self), pvenginecontext, padaptationdata.as_ptr(), padaptationdata.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetPropertyNum(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: *const super::WCHAR, lvalue: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyNum)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname, lvalue) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetPropertyNum(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: *const super::WCHAR) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyNum)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropertyString<P2, P3>(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: P2, pvalue: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyString)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname.param().abi(), pvalue.param().abi()) }
    }
    pub unsafe fn GetPropertyString<P2>(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyString)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "sapi")]
    pub unsafe fn SetGrammarState(&self, pvenginegrammar: *const core::ffi::c_void, egrammarstate: super::SPGRAMMARSTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGrammarState)(windows_core::Interface::as_raw(self), pvenginegrammar, egrammarstate) }
    }
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub unsafe fn WordNotify(&self, action: SPCFGNOTIFY, cwords: super::ULONG, pwords: *const SPWORDENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WordNotify)(windows_core::Interface::as_raw(self), action, cwords, pwords) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn RuleNotify(&self, action: SPCFGNOTIFY, crules: super::ULONG, prules: *const SPRULEENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RuleNotify)(windows_core::Interface::as_raw(self), action, crules, prules) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn PrivateCallEx(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: super::ULONG, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrivateCallEx)(windows_core::Interface::as_raw(self), pvenginecontext, pincallframe, ulincallframesize, ppvcomemresponse as _, pulresponsesize as _) }
    }
    #[cfg(feature = "sapi")]
    pub unsafe fn SetContextState(&self, pvenginecontext: *const core::ffi::c_void, econtextstate: super::SPCONTEXTSTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContextState)(windows_core::Interface::as_raw(self), pvenginecontext, econtextstate) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpSREngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub GetInputAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::WAVEFORMATEX, *mut windows_core::GUID, *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared")))]
    GetInputAudioFormat: usize,
    #[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
    pub RecognizeStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::WAVEFORMATEX, super::HANDLE, super::HANDLE, super::HANDLE, windows_core::BOOL, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt")))]
    RecognizeStream: usize,
    #[cfg(feature = "sapi")]
    pub SetRecoProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "sapi"))]
    SetRecoProfile: usize,
    pub OnCreateGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, SPGRAMMARHANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDeleteGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub LoadProprietaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const core::ffi::c_void, super::ULONG, super::SPLOADOPTIONS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    LoadProprietaryGrammar: usize,
    pub UnloadProprietaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub SetProprietaryRuleState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, super::SPRULESTATE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    SetProprietaryRuleState: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub SetProprietaryRuleIdState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::DWORD, super::SPRULESTATE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    SetProprietaryRuleIdState: usize,
    pub LoadSLM: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnloadSLM: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "sapi")]
    pub SetSLMState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::SPRULESTATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "sapi"))]
    SetSLMState: usize,
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub SetWordSequenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::WCHAR, super::ULONG, *const super::SPTEXTSELECTIONINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared", feature = "winnt")))]
    SetWordSequenceData: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub SetTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::SPTEXTSELECTIONINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    SetTextSelection: usize,
    #[cfg(feature = "sapi")]
    pub IsPronounceable: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut super::SPWORDPRONOUNCEABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "sapi"))]
    IsPronounceable: usize,
    pub OnCreateRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, SPRECOCONTEXTHANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDeleteRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub PrivateCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::PVOID, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    PrivateCall: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetAdaptationData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::WCHAR, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetAdaptationData: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetPropertyNum: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, *const super::WCHAR, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetPropertyNum: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetPropertyNum: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, *const super::WCHAR, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetPropertyNum: usize,
    pub SetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "sapi")]
    pub SetGrammarState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::SPGRAMMARSTATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "sapi"))]
    SetGrammarState: usize,
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub WordNotify: unsafe extern "system" fn(*mut core::ffi::c_void, SPCFGNOTIFY, super::ULONG, *const SPWORDENTRY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared", feature = "winnt")))]
    WordNotify: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub RuleNotify: unsafe extern "system" fn(*mut core::ffi::c_void, SPCFGNOTIFY, super::ULONG, *const SPRULEENTRY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    RuleNotify: usize,
    #[cfg(feature = "shared")]
    pub PrivateCallEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    PrivateCallEx: usize,
    #[cfg(feature = "sapi")]
    pub SetContextState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::SPCONTEXTSTATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "sapi"))]
    SetContextState: usize,
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
pub trait ISpSREngine_Impl: windows_core::IUnknownImpl {
    fn SetSite(&self, psite: windows_core::Ref<ISpSREngineSite>) -> windows_core::Result<()>;
    fn GetInputAudioFormat(&self, pguidsourceformatid: *const windows_core::GUID, psourcewaveformatex: *const super::WAVEFORMATEX, pguiddesiredformatid: *mut windows_core::GUID, ppcomemdesiredwaveformatex: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn RecognizeStream(&self, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::WAVEFORMATEX, hrequestsync: super::HANDLE, hdataavailable: super::HANDLE, hexit: super::HANDLE, fnewaudiostream: windows_core::BOOL, frealtimeaudio: windows_core::BOOL, paudioobjecttoken: windows_core::Ref<super::ISpObjectToken>) -> windows_core::Result<()>;
    fn SetRecoProfile(&self, pprofile: windows_core::Ref<super::ISpObjectToken>) -> windows_core::Result<()>;
    fn OnCreateGrammar(&self, pvenginerecocontext: *const core::ffi::c_void, hsapigrammar: SPGRAMMARHANDLE, ppvenginegrammarcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OnDeleteGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn LoadProprietaryGrammar(&self, pvenginegrammar: *const core::ffi::c_void, rguidparam: *const windows_core::GUID, pszstringparam: &windows_core::PCWSTR, pvdataparam: *const core::ffi::c_void, uldatasize: super::ULONG, options: super::SPLOADOPTIONS) -> windows_core::Result<()>;
    fn UnloadProprietaryGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetProprietaryRuleState(&self, pvenginegrammar: *const core::ffi::c_void, pszname: &windows_core::PCWSTR, preserved: *const core::ffi::c_void, newstate: super::SPRULESTATE) -> windows_core::Result<super::ULONG>;
    fn SetProprietaryRuleIdState(&self, pvenginegrammar: *const core::ffi::c_void, dwruleid: super::DWORD, newstate: super::SPRULESTATE) -> windows_core::Result<()>;
    fn LoadSLM(&self, pvenginegrammar: *const core::ffi::c_void, psztopicname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnloadSLM(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetSLMState(&self, pvenginegrammar: *const core::ffi::c_void, newstate: super::SPRULESTATE) -> windows_core::Result<()>;
    fn SetWordSequenceData(&self, pvenginegrammar: *const core::ffi::c_void, ptext: *const super::WCHAR, cchtext: super::ULONG, pinfo: *const super::SPTEXTSELECTIONINFO) -> windows_core::Result<()>;
    fn SetTextSelection(&self, pvenginegrammar: *const core::ffi::c_void, pinfo: *const super::SPTEXTSELECTIONINFO) -> windows_core::Result<()>;
    fn IsPronounceable(&self, pvenginegrammar: *const core::ffi::c_void, pszword: &windows_core::PCWSTR) -> windows_core::Result<super::SPWORDPRONOUNCEABLE>;
    fn OnCreateRecoContext(&self, hsapirecocontext: SPRECOCONTEXTHANDLE, ppvenginecontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OnDeleteRecoContext(&self, pvenginecontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn PrivateCall(&self, pvenginecontext: *const core::ffi::c_void, pcallframe: super::PVOID, ulcallframesize: super::ULONG) -> windows_core::Result<()>;
    fn SetAdaptationData(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: *const super::WCHAR, cch: super::ULONG) -> windows_core::Result<()>;
    fn SetPropertyNum(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: *const super::WCHAR, lvalue: super::LONG) -> windows_core::Result<()>;
    fn GetPropertyNum(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: *const super::WCHAR) -> windows_core::Result<super::LONG>;
    fn SetPropertyString(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: &windows_core::PCWSTR, pvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPropertyString(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn SetGrammarState(&self, pvenginegrammar: *const core::ffi::c_void, egrammarstate: super::SPGRAMMARSTATE) -> windows_core::Result<()>;
    fn WordNotify(&self, action: SPCFGNOTIFY, cwords: super::ULONG, pwords: *const SPWORDENTRY) -> windows_core::Result<()>;
    fn RuleNotify(&self, action: SPCFGNOTIFY, crules: super::ULONG, prules: *const SPRULEENTRY) -> windows_core::Result<()>;
    fn PrivateCallEx(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: super::ULONG, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetContextState(&self, pvenginecontext: *const core::ffi::c_void, econtextstate: super::SPCONTEXTSTATE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
impl ISpSREngine_Vtbl {
    pub const fn new<Identity: ISpSREngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSite<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetSite(this, core::mem::transmute_copy(&psite)).into()
            }
        }
        unsafe extern "system" fn GetInputAudioFormat<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidsourceformatid: *const windows_core::GUID, psourcewaveformatex: *const super::WAVEFORMATEX, pguiddesiredformatid: *mut windows_core::GUID, ppcomemdesiredwaveformatex: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::GetInputAudioFormat(this, core::mem::transmute_copy(&pguidsourceformatid), core::mem::transmute_copy(&psourcewaveformatex), core::mem::transmute_copy(&pguiddesiredformatid), core::mem::transmute_copy(&ppcomemdesiredwaveformatex)).into()
            }
        }
        unsafe extern "system" fn RecognizeStream<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::WAVEFORMATEX, hrequestsync: super::HANDLE, hdataavailable: super::HANDLE, hexit: super::HANDLE, fnewaudiostream: windows_core::BOOL, frealtimeaudio: windows_core::BOOL, paudioobjecttoken: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::RecognizeStream(this, core::mem::transmute_copy(&rguidfmtid), core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&hrequestsync), core::mem::transmute_copy(&hdataavailable), core::mem::transmute_copy(&hexit), core::mem::transmute_copy(&fnewaudiostream), core::mem::transmute_copy(&frealtimeaudio), core::mem::transmute_copy(&paudioobjecttoken)).into()
            }
        }
        unsafe extern "system" fn SetRecoProfile<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetRecoProfile(this, core::mem::transmute_copy(&pprofile)).into()
            }
        }
        unsafe extern "system" fn OnCreateGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginerecocontext: *const core::ffi::c_void, hsapigrammar: SPGRAMMARHANDLE, ppvenginegrammarcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::OnCreateGrammar(this, core::mem::transmute_copy(&pvenginerecocontext), core::mem::transmute_copy(&hsapigrammar), core::mem::transmute_copy(&ppvenginegrammarcontext)).into()
            }
        }
        unsafe extern "system" fn OnDeleteGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::OnDeleteGrammar(this, core::mem::transmute_copy(&pvenginegrammar)).into()
            }
        }
        unsafe extern "system" fn LoadProprietaryGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, rguidparam: *const windows_core::GUID, pszstringparam: windows_core::PCWSTR, pvdataparam: *const core::ffi::c_void, uldatasize: super::ULONG, options: super::SPLOADOPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::LoadProprietaryGrammar(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&rguidparam), core::mem::transmute(&pszstringparam), core::mem::transmute_copy(&pvdataparam), core::mem::transmute_copy(&uldatasize), core::mem::transmute_copy(&options)).into()
            }
        }
        unsafe extern "system" fn UnloadProprietaryGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::UnloadProprietaryGrammar(this, core::mem::transmute_copy(&pvenginegrammar)).into()
            }
        }
        unsafe extern "system" fn SetProprietaryRuleState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pszname: windows_core::PCWSTR, preserved: *const core::ffi::c_void, newstate: super::SPRULESTATE, pcruleschanged: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngine_Impl::SetProprietaryRuleState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&pszname), core::mem::transmute_copy(&preserved), core::mem::transmute_copy(&newstate)) {
                    Ok(ok__) => {
                        pcruleschanged.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProprietaryRuleIdState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, dwruleid: super::DWORD, newstate: super::SPRULESTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetProprietaryRuleIdState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&dwruleid), core::mem::transmute_copy(&newstate)).into()
            }
        }
        unsafe extern "system" fn LoadSLM<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, psztopicname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::LoadSLM(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&psztopicname)).into()
            }
        }
        unsafe extern "system" fn UnloadSLM<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::UnloadSLM(this, core::mem::transmute_copy(&pvenginegrammar)).into()
            }
        }
        unsafe extern "system" fn SetSLMState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, newstate: super::SPRULESTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetSLMState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&newstate)).into()
            }
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, ptext: *const super::WCHAR, cchtext: super::ULONG, pinfo: *const super::SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetWordSequenceData(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&cchtext), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn SetTextSelection<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pinfo: *const super::SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetTextSelection(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn IsPronounceable<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pszword: windows_core::PCWSTR, pwordpronounceable: *mut super::SPWORDPRONOUNCEABLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngine_Impl::IsPronounceable(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&pszword)) {
                    Ok(ok__) => {
                        pwordpronounceable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnCreateRecoContext<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsapirecocontext: SPRECOCONTEXTHANDLE, ppvenginecontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::OnCreateRecoContext(this, core::mem::transmute_copy(&hsapirecocontext), core::mem::transmute_copy(&ppvenginecontext)).into()
            }
        }
        unsafe extern "system" fn OnDeleteRecoContext<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::OnDeleteRecoContext(this, core::mem::transmute_copy(&pvenginecontext)).into()
            }
        }
        unsafe extern "system" fn PrivateCall<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, pcallframe: super::PVOID, ulcallframesize: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::PrivateCall(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&pcallframe), core::mem::transmute_copy(&ulcallframesize)).into()
            }
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, padaptationdata: *const super::WCHAR, cch: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetAdaptationData(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&padaptationdata), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn SetPropertyNum<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: *const super::WCHAR, lvalue: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetPropertyNum(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute_copy(&pname), core::mem::transmute_copy(&lvalue)).into()
            }
        }
        unsafe extern "system" fn GetPropertyNum<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: *const super::WCHAR, lvalue: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngine_Impl::GetPropertyNum(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute_copy(&pname)) {
                    Ok(ok__) => {
                        lvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertyString<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: windows_core::PCWSTR, pvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetPropertyString(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute(&pname), core::mem::transmute(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetPropertyString<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: windows_core::PCWSTR, ppcomemvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngine_Impl::GetPropertyString(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute(&pname)) {
                    Ok(ok__) => {
                        ppcomemvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGrammarState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, egrammarstate: super::SPGRAMMARSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetGrammarState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&egrammarstate)).into()
            }
        }
        unsafe extern "system" fn WordNotify<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: SPCFGNOTIFY, cwords: super::ULONG, pwords: *const SPWORDENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::WordNotify(this, core::mem::transmute_copy(&action), core::mem::transmute_copy(&cwords), core::mem::transmute_copy(&pwords)).into()
            }
        }
        unsafe extern "system" fn RuleNotify<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: SPCFGNOTIFY, crules: super::ULONG, prules: *const SPRULEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::RuleNotify(this, core::mem::transmute_copy(&action), core::mem::transmute_copy(&crules), core::mem::transmute_copy(&prules)).into()
            }
        }
        unsafe extern "system" fn PrivateCallEx<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: super::ULONG, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::PrivateCallEx(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&pincallframe), core::mem::transmute_copy(&ulincallframesize), core::mem::transmute_copy(&ppvcomemresponse), core::mem::transmute_copy(&pulresponsesize)).into()
            }
        }
        unsafe extern "system" fn SetContextState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, econtextstate: super::SPCONTEXTSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine_Impl::SetContextState(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&econtextstate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSite: SetSite::<Identity, OFFSET>,
            GetInputAudioFormat: GetInputAudioFormat::<Identity, OFFSET>,
            RecognizeStream: RecognizeStream::<Identity, OFFSET>,
            SetRecoProfile: SetRecoProfile::<Identity, OFFSET>,
            OnCreateGrammar: OnCreateGrammar::<Identity, OFFSET>,
            OnDeleteGrammar: OnDeleteGrammar::<Identity, OFFSET>,
            LoadProprietaryGrammar: LoadProprietaryGrammar::<Identity, OFFSET>,
            UnloadProprietaryGrammar: UnloadProprietaryGrammar::<Identity, OFFSET>,
            SetProprietaryRuleState: SetProprietaryRuleState::<Identity, OFFSET>,
            SetProprietaryRuleIdState: SetProprietaryRuleIdState::<Identity, OFFSET>,
            LoadSLM: LoadSLM::<Identity, OFFSET>,
            UnloadSLM: UnloadSLM::<Identity, OFFSET>,
            SetSLMState: SetSLMState::<Identity, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, OFFSET>,
            OnCreateRecoContext: OnCreateRecoContext::<Identity, OFFSET>,
            OnDeleteRecoContext: OnDeleteRecoContext::<Identity, OFFSET>,
            PrivateCall: PrivateCall::<Identity, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, OFFSET>,
            SetPropertyNum: SetPropertyNum::<Identity, OFFSET>,
            GetPropertyNum: GetPropertyNum::<Identity, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, OFFSET>,
            SetGrammarState: SetGrammarState::<Identity, OFFSET>,
            WordNotify: WordNotify::<Identity, OFFSET>,
            RuleNotify: RuleNotify::<Identity, OFFSET>,
            PrivateCallEx: PrivateCallEx::<Identity, OFFSET>,
            SetContextState: SetContextState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngine as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ISpSREngine {}
windows_core::imp::define_interface!(ISpSREngine2, ISpSREngine2_Vtbl, 0x7ba627d8_33f9_4375_90c5_9985aee5ede5);
impl core::ops::Deref for ISpSREngine2 {
    type Target = ISpSREngine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSREngine2, windows_core::IUnknown, ISpSREngine);
impl ISpSREngine2 {
    #[cfg(feature = "shared")]
    pub unsafe fn PrivateCallImmediate(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: super::ULONG, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrivateCallImmediate)(windows_core::Interface::as_raw(self), pvenginecontext, pincallframe, ulincallframesize, ppvcomemresponse as _, pulresponsesize as _) }
    }
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub unsafe fn SetAdaptationData2<P3>(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: &[super::WCHAR], ptopicname: P3, esettings: super::SPADAPTATIONSETTINGS, erelevance: super::SPADAPTATIONRELEVANCE) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAdaptationData2)(windows_core::Interface::as_raw(self), pvenginecontext, padaptationdata.as_ptr(), padaptationdata.len().try_into().unwrap(), ptopicname.param().abi(), esettings, erelevance) }
    }
    pub unsafe fn SetGrammarPrefix<P1>(&self, pvenginegrammar: *const core::ffi::c_void, pszprefix: P1, fisprefixrequired: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGrammarPrefix)(windows_core::Interface::as_raw(self), pvenginegrammar, pszprefix.param().abi(), fisprefixrequired.into()) }
    }
    pub unsafe fn SetRulePriority(&self, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, nrulepriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRulePriority)(windows_core::Interface::as_raw(self), hrule, pvclientrulecontext, nrulepriority) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn EmulateRecognition<P0>(&self, pphrase: P0, dwcompareflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ISpPhrase>,
    {
        unsafe { (windows_core::Interface::vtable(self).EmulateRecognition)(windows_core::Interface::as_raw(self), pphrase.param().abi(), dwcompareflags) }
    }
    pub unsafe fn SetSLMWeight(&self, pvenginegrammar: *const core::ffi::c_void, flweight: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSLMWeight)(windows_core::Interface::as_raw(self), pvenginegrammar, flweight) }
    }
    pub unsafe fn SetRuleWeight(&self, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, flweight: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRuleWeight)(windows_core::Interface::as_raw(self), hrule, pvclientrulecontext, flweight) }
    }
    pub unsafe fn SetTrainingState(&self, fdoingtraining: bool, fadaptfromtrainingdata: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTrainingState)(windows_core::Interface::as_raw(self), fdoingtraining.into(), fadaptfromtrainingdata.into()) }
    }
    pub unsafe fn ResetAcousticModelAdaptation(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetAcousticModelAdaptation)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn OnLoadCFG(&self, pvenginegrammar: *const core::ffi::c_void, pgrammardata: *const super::SPBINARYGRAMMAR, ulgrammarid: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLoadCFG)(windows_core::Interface::as_raw(self), pvenginegrammar, pgrammardata, ulgrammarid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnUnloadCFG(&self, pvenginegrammar: *const core::ffi::c_void, ulgrammarid: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUnloadCFG)(windows_core::Interface::as_raw(self), pvenginegrammar, ulgrammarid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpSREngine2_Vtbl {
    pub base__: ISpSREngine_Vtbl,
    #[cfg(feature = "shared")]
    pub PrivateCallImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    PrivateCallImmediate: usize,
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub SetAdaptationData2: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::WCHAR, super::ULONG, windows_core::PCWSTR, super::SPADAPTATIONSETTINGS, super::SPADAPTATIONRELEVANCE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared", feature = "winnt")))]
    SetAdaptationData2: usize,
    pub SetGrammarPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetRulePriority: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *const core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub EmulateRecognition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    EmulateRecognition: usize,
    pub SetSLMWeight: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetRuleWeight: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *const core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTrainingState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub ResetAcousticModelAdaptation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub OnLoadCFG: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::SPBINARYGRAMMAR, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    OnLoadCFG: usize,
    #[cfg(feature = "shared")]
    pub OnUnloadCFG: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnUnloadCFG: usize,
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
pub trait ISpSREngine2_Impl: ISpSREngine_Impl {
    fn PrivateCallImmediate(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: super::ULONG, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetAdaptationData2(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: *const super::WCHAR, cch: super::ULONG, ptopicname: &windows_core::PCWSTR, esettings: super::SPADAPTATIONSETTINGS, erelevance: super::SPADAPTATIONRELEVANCE) -> windows_core::Result<()>;
    fn SetGrammarPrefix(&self, pvenginegrammar: *const core::ffi::c_void, pszprefix: &windows_core::PCWSTR, fisprefixrequired: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetRulePriority(&self, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, nrulepriority: i32) -> windows_core::Result<()>;
    fn EmulateRecognition(&self, pphrase: windows_core::Ref<super::ISpPhrase>, dwcompareflags: super::DWORD) -> windows_core::Result<()>;
    fn SetSLMWeight(&self, pvenginegrammar: *const core::ffi::c_void, flweight: f32) -> windows_core::Result<()>;
    fn SetRuleWeight(&self, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, flweight: f32) -> windows_core::Result<()>;
    fn SetTrainingState(&self, fdoingtraining: windows_core::BOOL, fadaptfromtrainingdata: windows_core::BOOL) -> windows_core::Result<()>;
    fn ResetAcousticModelAdaptation(&self) -> windows_core::Result<()>;
    fn OnLoadCFG(&self, pvenginegrammar: *const core::ffi::c_void, pgrammardata: *const super::SPBINARYGRAMMAR, ulgrammarid: super::ULONG) -> windows_core::Result<()>;
    fn OnUnloadCFG(&self, pvenginegrammar: *const core::ffi::c_void, ulgrammarid: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
impl ISpSREngine2_Vtbl {
    pub const fn new<Identity: ISpSREngine2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrivateCallImmediate<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: super::ULONG, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::PrivateCallImmediate(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&pincallframe), core::mem::transmute_copy(&ulincallframesize), core::mem::transmute_copy(&ppvcomemresponse), core::mem::transmute_copy(&pulresponsesize)).into()
            }
        }
        unsafe extern "system" fn SetAdaptationData2<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, padaptationdata: *const super::WCHAR, cch: super::ULONG, ptopicname: windows_core::PCWSTR, esettings: super::SPADAPTATIONSETTINGS, erelevance: super::SPADAPTATIONRELEVANCE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::SetAdaptationData2(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&padaptationdata), core::mem::transmute_copy(&cch), core::mem::transmute(&ptopicname), core::mem::transmute_copy(&esettings), core::mem::transmute_copy(&erelevance)).into()
            }
        }
        unsafe extern "system" fn SetGrammarPrefix<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pszprefix: windows_core::PCWSTR, fisprefixrequired: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::SetGrammarPrefix(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&pszprefix), core::mem::transmute_copy(&fisprefixrequired)).into()
            }
        }
        unsafe extern "system" fn SetRulePriority<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, nrulepriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::SetRulePriority(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&pvclientrulecontext), core::mem::transmute_copy(&nrulepriority)).into()
            }
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void, dwcompareflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::EmulateRecognition(this, core::mem::transmute_copy(&pphrase), core::mem::transmute_copy(&dwcompareflags)).into()
            }
        }
        unsafe extern "system" fn SetSLMWeight<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, flweight: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::SetSLMWeight(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&flweight)).into()
            }
        }
        unsafe extern "system" fn SetRuleWeight<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, flweight: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::SetRuleWeight(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&pvclientrulecontext), core::mem::transmute_copy(&flweight)).into()
            }
        }
        unsafe extern "system" fn SetTrainingState<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdoingtraining: windows_core::BOOL, fadaptfromtrainingdata: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::SetTrainingState(this, core::mem::transmute_copy(&fdoingtraining), core::mem::transmute_copy(&fadaptfromtrainingdata)).into()
            }
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::ResetAcousticModelAdaptation(this).into()
            }
        }
        unsafe extern "system" fn OnLoadCFG<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pgrammardata: *const super::SPBINARYGRAMMAR, ulgrammarid: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::OnLoadCFG(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&pgrammardata), core::mem::transmute_copy(&ulgrammarid)).into()
            }
        }
        unsafe extern "system" fn OnUnloadCFG<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, ulgrammarid: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngine2_Impl::OnUnloadCFG(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&ulgrammarid)).into()
            }
        }
        Self {
            base__: ISpSREngine_Vtbl::new::<Identity, OFFSET>(),
            PrivateCallImmediate: PrivateCallImmediate::<Identity, OFFSET>,
            SetAdaptationData2: SetAdaptationData2::<Identity, OFFSET>,
            SetGrammarPrefix: SetGrammarPrefix::<Identity, OFFSET>,
            SetRulePriority: SetRulePriority::<Identity, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, OFFSET>,
            SetSLMWeight: SetSLMWeight::<Identity, OFFSET>,
            SetRuleWeight: SetRuleWeight::<Identity, OFFSET>,
            SetTrainingState: SetTrainingState::<Identity, OFFSET>,
            ResetAcousticModelAdaptation: ResetAcousticModelAdaptation::<Identity, OFFSET>,
            OnLoadCFG: OnLoadCFG::<Identity, OFFSET>,
            OnUnloadCFG: OnUnloadCFG::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngine2 as windows_core::Interface>::IID || iid == &<ISpSREngine as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ISpSREngine2 {}
windows_core::imp::define_interface!(ISpSREngineSite, ISpSREngineSite_Vtbl, 0x3b414aec_720c_4883_b9ef_178cd394fb3a);
windows_core::imp::interface_hierarchy!(ISpSREngineSite, windows_core::IUnknown);
impl ISpSREngineSite {
    #[cfg(feature = "shared")]
    pub unsafe fn Read(&self, pv: *const core::ffi::c_void, cb: super::ULONG) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv, cb, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DataAvailable(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetBufferNotifySize(&self, cbsize: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferNotifySize)(windows_core::Interface::as_raw(self), cbsize) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn ParseFromTransitions(&self, pparseinfo: *const SPPARSEINFO) -> windows_core::Result<ISpPhraseBuilder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseFromTransitions)(windows_core::Interface::as_raw(self), pparseinfo, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn Recognition(&self, presultinfo: *const SPRECORESULTINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Recognition)(windows_core::Interface::as_raw(self), presultinfo) }
    }
    #[cfg(all(feature = "minwindef", feature = "sapi", feature = "shared"))]
    pub unsafe fn AddEvent(&self, pevent: *const super::SPEVENT, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddEvent)(windows_core::Interface::as_raw(self), pevent, hsapirecocontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Synchronize(&self, ullprocessedthrupos: super::ULONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Synchronize)(windows_core::Interface::as_raw(self), ullprocessedthrupos) }
    }
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetWordInfo(&self, pwordentry: *mut SPWORDENTRY, options: SPWORDINFOOPT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWordInfo)(windows_core::Interface::as_raw(self), pwordentry as _, options) }
    }
    pub unsafe fn SetWordClientContext(&self, hword: SPWORDHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWordClientContext)(windows_core::Interface::as_raw(self), hword, pvclientcontext) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn GetRuleInfo(&self, pruleentry: *mut SPRULEENTRY, options: SPRULEINFOOPT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRuleInfo)(windows_core::Interface::as_raw(self), pruleentry as _, options) }
    }
    pub unsafe fn SetRuleClientContext(&self, hrule: SPRULEHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRuleClientContext)(windows_core::Interface::as_raw(self), hrule, pvclientcontext) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn GetStateInfo(&self, hstate: super::SPSTATEHANDLE, pstateinfo: *mut SPSTATEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStateInfo)(windows_core::Interface::as_raw(self), hstate, pstateinfo as _) }
    }
    pub unsafe fn GetResource<P1>(&self, hrule: SPRULEHANDLE, pszresourcename: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), hrule, pszresourcename.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetTransitionProperty(&self, id: SPTRANSITIONID) -> windows_core::Result<*mut SPTRANSITIONPROPERTY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransitionProperty)(windows_core::Interface::as_raw(self), id, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsAlternate(&self, hrule: SPRULEHANDLE, haltrule: SPRULEHANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsAlternate)(windows_core::Interface::as_raw(self), hrule, haltrule) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxAlternates(&self, hrule: SPRULEHANDLE) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxAlternates)(windows_core::Interface::as_raw(self), hrule, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetContextMaxAlternates(&self, hcontext: SPRECOCONTEXTHANDLE) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextMaxAlternates)(windows_core::Interface::as_raw(self), hcontext, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UpdateRecoPos(&self, ullcurrentrecopos: super::ULONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateRecoPos)(windows_core::Interface::as_raw(self), ullcurrentrecopos) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpSREngineSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Read: usize,
    #[cfg(feature = "shared")]
    pub DataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DataAvailable: usize,
    #[cfg(feature = "shared")]
    pub SetBufferNotifySize: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBufferNotifySize: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub ParseFromTransitions: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPARSEINFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    ParseFromTransitions: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub Recognition: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPRECORESULTINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    Recognition: usize,
    #[cfg(all(feature = "minwindef", feature = "sapi", feature = "shared"))]
    pub AddEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SPEVENT, SPRECOCONTEXTHANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "sapi", feature = "shared")))]
    AddEvent: usize,
    #[cfg(feature = "shared")]
    pub Synchronize: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Synchronize: usize,
    #[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
    pub GetWordInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPWORDENTRY, SPWORDINFOOPT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared", feature = "winnt")))]
    GetWordInfo: usize,
    pub SetWordClientContext: unsafe extern "system" fn(*mut core::ffi::c_void, SPWORDHANDLE, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub GetRuleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPRULEENTRY, SPRULEINFOOPT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    GetRuleInfo: usize,
    pub SetRuleClientContext: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub GetStateInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::SPSTATEHANDLE, *mut SPSTATEINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    GetStateInfo: usize,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetTransitionProperty: unsafe extern "system" fn(*mut core::ffi::c_void, SPTRANSITIONID, *mut *mut SPTRANSITIONPROPERTY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetTransitionProperty: usize,
    pub IsAlternate: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, SPRULEHANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxAlternates: usize,
    #[cfg(feature = "shared")]
    pub GetContextMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, SPRECOCONTEXTHANDLE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetContextMaxAlternates: usize,
    #[cfg(feature = "shared")]
    pub UpdateRecoPos: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UpdateRecoPos: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISpSREngineSite_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pv: *const core::ffi::c_void, cb: super::ULONG) -> windows_core::Result<super::ULONG>;
    fn DataAvailable(&self) -> windows_core::Result<super::ULONG>;
    fn SetBufferNotifySize(&self, cbsize: super::ULONG) -> windows_core::Result<()>;
    fn ParseFromTransitions(&self, pparseinfo: *const SPPARSEINFO) -> windows_core::Result<ISpPhraseBuilder>;
    fn Recognition(&self, presultinfo: *const SPRECORESULTINFO) -> windows_core::Result<()>;
    fn AddEvent(&self, pevent: *const super::SPEVENT, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::Result<()>;
    fn Synchronize(&self, ullprocessedthrupos: super::ULONGLONG) -> windows_core::Result<()>;
    fn GetWordInfo(&self, pwordentry: *mut SPWORDENTRY, options: SPWORDINFOOPT) -> windows_core::Result<()>;
    fn SetWordClientContext(&self, hword: SPWORDHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetRuleInfo(&self, pruleentry: *mut SPRULEENTRY, options: SPRULEINFOOPT) -> windows_core::Result<()>;
    fn SetRuleClientContext(&self, hrule: SPRULEHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStateInfo(&self, hstate: super::SPSTATEHANDLE, pstateinfo: *mut SPSTATEINFO) -> windows_core::Result<()>;
    fn GetResource(&self, hrule: SPRULEHANDLE, pszresourcename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTransitionProperty(&self, id: SPTRANSITIONID) -> windows_core::Result<*mut SPTRANSITIONPROPERTY>;
    fn IsAlternate(&self, hrule: SPRULEHANDLE, haltrule: SPRULEHANDLE) -> windows_core::Result<()>;
    fn GetMaxAlternates(&self, hrule: SPRULEHANDLE) -> windows_core::Result<super::ULONG>;
    fn GetContextMaxAlternates(&self, hcontext: SPRECOCONTEXTHANDLE) -> windows_core::Result<super::ULONG>;
    fn UpdateRecoPos(&self, ullcurrentrecopos: super::ULONGLONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISpSREngineSite_Vtbl {
    pub const fn new<Identity: ISpSREngineSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: super::ULONG, pcbread: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngineSite_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                    Ok(ok__) => {
                        pcbread.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataAvailable<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngineSite_Impl::DataAvailable(this) {
                    Ok(ok__) => {
                        pcb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::SetBufferNotifySize(this, core::mem::transmute_copy(&cbsize)).into()
            }
        }
        unsafe extern "system" fn ParseFromTransitions<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparseinfo: *const SPPARSEINFO, ppnewphrase: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngineSite_Impl::ParseFromTransitions(this, core::mem::transmute_copy(&pparseinfo)) {
                    Ok(ok__) => {
                        ppnewphrase.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Recognition<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presultinfo: *const SPRECORESULTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::Recognition(this, core::mem::transmute_copy(&presultinfo)).into()
            }
        }
        unsafe extern "system" fn AddEvent<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *const super::SPEVENT, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::AddEvent(this, core::mem::transmute_copy(&pevent), core::mem::transmute_copy(&hsapirecocontext)).into()
            }
        }
        unsafe extern "system" fn Synchronize<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullprocessedthrupos: super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::Synchronize(this, core::mem::transmute_copy(&ullprocessedthrupos)).into()
            }
        }
        unsafe extern "system" fn GetWordInfo<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwordentry: *mut SPWORDENTRY, options: SPWORDINFOOPT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::GetWordInfo(this, core::mem::transmute_copy(&pwordentry), core::mem::transmute_copy(&options)).into()
            }
        }
        unsafe extern "system" fn SetWordClientContext<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hword: SPWORDHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::SetWordClientContext(this, core::mem::transmute_copy(&hword), core::mem::transmute_copy(&pvclientcontext)).into()
            }
        }
        unsafe extern "system" fn GetRuleInfo<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pruleentry: *mut SPRULEENTRY, options: SPRULEINFOOPT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::GetRuleInfo(this, core::mem::transmute_copy(&pruleentry), core::mem::transmute_copy(&options)).into()
            }
        }
        unsafe extern "system" fn SetRuleClientContext<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::SetRuleClientContext(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&pvclientcontext)).into()
            }
        }
        unsafe extern "system" fn GetStateInfo<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstate: super::SPSTATEHANDLE, pstateinfo: *mut SPSTATEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::GetStateInfo(this, core::mem::transmute_copy(&hstate), core::mem::transmute_copy(&pstateinfo)).into()
            }
        }
        unsafe extern "system" fn GetResource<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pszresourcename: windows_core::PCWSTR, ppcomemresource: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngineSite_Impl::GetResource(this, core::mem::transmute_copy(&hrule), core::mem::transmute(&pszresourcename)) {
                    Ok(ok__) => {
                        ppcomemresource.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransitionProperty<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: SPTRANSITIONID, ppcomemproperty: *mut *mut SPTRANSITIONPROPERTY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngineSite_Impl::GetTransitionProperty(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        ppcomemproperty.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAlternate<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, haltrule: SPRULEHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::IsAlternate(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&haltrule)).into()
            }
        }
        unsafe extern "system" fn GetMaxAlternates<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pulnumalts: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngineSite_Impl::GetMaxAlternates(this, core::mem::transmute_copy(&hrule)) {
                    Ok(ok__) => {
                        pulnumalts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContextMaxAlternates<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcontext: SPRECOCONTEXTHANDLE, pulnumalts: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpSREngineSite_Impl::GetContextMaxAlternates(this, core::mem::transmute_copy(&hcontext)) {
                    Ok(ok__) => {
                        pulnumalts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateRecoPos<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullcurrentrecopos: super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite_Impl::UpdateRecoPos(this, core::mem::transmute_copy(&ullcurrentrecopos)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            DataAvailable: DataAvailable::<Identity, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, OFFSET>,
            ParseFromTransitions: ParseFromTransitions::<Identity, OFFSET>,
            Recognition: Recognition::<Identity, OFFSET>,
            AddEvent: AddEvent::<Identity, OFFSET>,
            Synchronize: Synchronize::<Identity, OFFSET>,
            GetWordInfo: GetWordInfo::<Identity, OFFSET>,
            SetWordClientContext: SetWordClientContext::<Identity, OFFSET>,
            GetRuleInfo: GetRuleInfo::<Identity, OFFSET>,
            SetRuleClientContext: SetRuleClientContext::<Identity, OFFSET>,
            GetStateInfo: GetStateInfo::<Identity, OFFSET>,
            GetResource: GetResource::<Identity, OFFSET>,
            GetTransitionProperty: GetTransitionProperty::<Identity, OFFSET>,
            IsAlternate: IsAlternate::<Identity, OFFSET>,
            GetMaxAlternates: GetMaxAlternates::<Identity, OFFSET>,
            GetContextMaxAlternates: GetContextMaxAlternates::<Identity, OFFSET>,
            UpdateRecoPos: UpdateRecoPos::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngineSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISpSREngineSite {}
windows_core::imp::define_interface!(ISpSREngineSite2, ISpSREngineSite2_Vtbl, 0x7bc6e012_684a_493e_bdd4_2bf5fbf48cfe);
impl core::ops::Deref for ISpSREngineSite2 {
    type Target = ISpSREngineSite;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSREngineSite2, windows_core::IUnknown, ISpSREngineSite);
impl ISpSREngineSite2 {
    #[cfg(all(feature = "minwindef", feature = "sapi", feature = "shared"))]
    pub unsafe fn AddEventEx(&self, pevent: *const super::SPEVENTEX, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddEventEx)(windows_core::Interface::as_raw(self), pevent, hsapirecocontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UpdateRecoPosEx(&self, ullcurrentrecopos: super::ULONGLONG, ullcurrentrecotime: super::ULONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateRecoPosEx)(windows_core::Interface::as_raw(self), ullcurrentrecopos, ullcurrentrecotime) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn GetRuleTransition(&self, ulgrammarid: super::ULONG, ruleindex: super::ULONG, ptrans: *mut SPTRANSITIONENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRuleTransition)(windows_core::Interface::as_raw(self), ulgrammarid, ruleindex, ptrans as _) }
    }
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub unsafe fn RecognitionEx(&self, presultinfo: *const SPRECORESULTINFOEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecognitionEx)(windows_core::Interface::as_raw(self), presultinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpSREngineSite2_Vtbl {
    pub base__: ISpSREngineSite_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "sapi", feature = "shared"))]
    pub AddEventEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SPEVENTEX, SPRECOCONTEXTHANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "sapi", feature = "shared")))]
    AddEventEx: usize,
    #[cfg(feature = "shared")]
    pub UpdateRecoPosEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG, super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UpdateRecoPosEx: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub GetRuleTransition: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::ULONG, *mut SPTRANSITIONENTRY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    GetRuleTransition: usize,
    #[cfg(all(feature = "sapi", feature = "shared"))]
    pub RecognitionEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPRECORESULTINFOEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared")))]
    RecognitionEx: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISpSREngineSite2_Impl: ISpSREngineSite_Impl {
    fn AddEventEx(&self, pevent: *const super::SPEVENTEX, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::Result<()>;
    fn UpdateRecoPosEx(&self, ullcurrentrecopos: super::ULONGLONG, ullcurrentrecotime: super::ULONGLONG) -> windows_core::Result<()>;
    fn GetRuleTransition(&self, ulgrammarid: super::ULONG, ruleindex: super::ULONG, ptrans: *mut SPTRANSITIONENTRY) -> windows_core::Result<()>;
    fn RecognitionEx(&self, presultinfo: *const SPRECORESULTINFOEX) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISpSREngineSite2_Vtbl {
    pub const fn new<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddEventEx<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *const super::SPEVENTEX, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite2_Impl::AddEventEx(this, core::mem::transmute_copy(&pevent), core::mem::transmute_copy(&hsapirecocontext)).into()
            }
        }
        unsafe extern "system" fn UpdateRecoPosEx<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullcurrentrecopos: super::ULONGLONG, ullcurrentrecotime: super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite2_Impl::UpdateRecoPosEx(this, core::mem::transmute_copy(&ullcurrentrecopos), core::mem::transmute_copy(&ullcurrentrecotime)).into()
            }
        }
        unsafe extern "system" fn GetRuleTransition<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulgrammarid: super::ULONG, ruleindex: super::ULONG, ptrans: *mut SPTRANSITIONENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite2_Impl::GetRuleTransition(this, core::mem::transmute_copy(&ulgrammarid), core::mem::transmute_copy(&ruleindex), core::mem::transmute_copy(&ptrans)).into()
            }
        }
        unsafe extern "system" fn RecognitionEx<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presultinfo: *const SPRECORESULTINFOEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpSREngineSite2_Impl::RecognitionEx(this, core::mem::transmute_copy(&presultinfo)).into()
            }
        }
        Self {
            base__: ISpSREngineSite_Vtbl::new::<Identity, OFFSET>(),
            AddEventEx: AddEventEx::<Identity, OFFSET>,
            UpdateRecoPosEx: UpdateRecoPosEx::<Identity, OFFSET>,
            GetRuleTransition: GetRuleTransition::<Identity, OFFSET>,
            RecognitionEx: RecognitionEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngineSite2 as windows_core::Interface>::IID || iid == &<ISpSREngineSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "sapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISpSREngineSite2 {}
windows_core::imp::define_interface!(ISpTTSEngine, ISpTTSEngine_Vtbl, 0xa74d7c8e_4cc5_4f2f_a6eb_804dee18500e);
windows_core::imp::interface_hierarchy!(ISpTTSEngine, windows_core::IUnknown);
impl ISpTTSEngine {
    #[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared"))]
    pub unsafe fn Speak<P4>(&self, dwspeakflags: super::DWORD, rguidformatid: *const windows_core::GUID, pwaveformatex: *const super::WAVEFORMATEX, ptextfraglist: *const SPVTEXTFRAG, poutputsite: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<ISpTTSEngineSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).Speak)(windows_core::Interface::as_raw(self), dwspeakflags, rguidformatid, pwaveformatex, ptextfraglist, poutputsite.param().abi()) }
    }
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub unsafe fn GetOutputFormat(&self, ptargetfmtid: *const windows_core::GUID, ptargetwaveformatex: *const super::WAVEFORMATEX, poutputformatid: *mut windows_core::GUID, ppcomemoutputwaveformatex: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputFormat)(windows_core::Interface::as_raw(self), ptargetfmtid, ptargetwaveformatex, poutputformatid as _, ppcomemoutputwaveformatex as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpTTSEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared"))]
    pub Speak: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *const super::WAVEFORMATEX, *const SPVTEXTFRAG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "sapi", feature = "shared")))]
    Speak: usize,
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub GetOutputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::WAVEFORMATEX, *mut windows_core::GUID, *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared")))]
    GetOutputFormat: usize,
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared"))]
pub trait ISpTTSEngine_Impl: windows_core::IUnknownImpl {
    fn Speak(&self, dwspeakflags: super::DWORD, rguidformatid: *const windows_core::GUID, pwaveformatex: *const super::WAVEFORMATEX, ptextfraglist: *const SPVTEXTFRAG, poutputsite: windows_core::Ref<ISpTTSEngineSite>) -> windows_core::Result<()>;
    fn GetOutputFormat(&self, ptargetfmtid: *const windows_core::GUID, ptargetwaveformatex: *const super::WAVEFORMATEX, poutputformatid: *mut windows_core::GUID, ppcomemoutputwaveformatex: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared"))]
impl ISpTTSEngine_Vtbl {
    pub const fn new<Identity: ISpTTSEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Speak<Identity: ISpTTSEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspeakflags: super::DWORD, rguidformatid: *const windows_core::GUID, pwaveformatex: *const super::WAVEFORMATEX, ptextfraglist: *const SPVTEXTFRAG, poutputsite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTTSEngine_Impl::Speak(this, core::mem::transmute_copy(&dwspeakflags), core::mem::transmute_copy(&rguidformatid), core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&ptextfraglist), core::mem::transmute_copy(&poutputsite)).into()
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ISpTTSEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetfmtid: *const windows_core::GUID, ptargetwaveformatex: *const super::WAVEFORMATEX, poutputformatid: *mut windows_core::GUID, ppcomemoutputwaveformatex: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTTSEngine_Impl::GetOutputFormat(this, core::mem::transmute_copy(&ptargetfmtid), core::mem::transmute_copy(&ptargetwaveformatex), core::mem::transmute_copy(&poutputformatid), core::mem::transmute_copy(&ppcomemoutputwaveformatex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Speak: Speak::<Identity, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTTSEngine as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mmeapi", feature = "sapi", feature = "shared"))]
impl windows_core::RuntimeName for ISpTTSEngine {}
#[cfg(feature = "sapi")]
windows_core::imp::define_interface!(ISpTTSEngineSite, ISpTTSEngineSite_Vtbl, 0x9880499b_cce9_11d2_b503_00c04f797396);
#[cfg(feature = "sapi")]
impl core::ops::Deref for ISpTTSEngineSite {
    type Target = super::ISpEventSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "sapi")]
windows_core::imp::interface_hierarchy!(ISpTTSEngineSite, windows_core::IUnknown, super::ISpEventSink);
#[cfg(feature = "sapi")]
impl ISpTTSEngineSite {
    #[cfg(feature = "shared")]
    pub unsafe fn GetActions(&self) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).GetActions)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Write(&self, pbuff: *const core::ffi::c_void, cb: super::ULONG) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pbuff, cb, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetVolume(&self) -> windows_core::Result<super::USHORT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSkipInfo(&self, petype: *mut SPVSKIPTYPE, plnumitems: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSkipInfo)(windows_core::Interface::as_raw(self), petype as _, plnumitems as _) }
    }
    pub unsafe fn CompleteSkip(&self, ulnumskipped: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompleteSkip)(windows_core::Interface::as_raw(self), ulnumskipped) }
    }
}
#[cfg(feature = "sapi")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpTTSEngineSite_Vtbl {
    pub base__: super::ISpEventSink_Vtbl,
    #[cfg(feature = "shared")]
    pub GetActions: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    GetActions: usize,
    #[cfg(feature = "shared")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Write: usize,
    pub GetRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::USHORT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVolume: usize,
    pub GetSkipInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPVSKIPTYPE, *mut i32) -> windows_core::HRESULT,
    pub CompleteSkip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "sapi", feature = "shared"))]
pub trait ISpTTSEngineSite_Impl: super::ISpEventSink_Impl {
    fn GetActions(&self) -> super::DWORD;
    fn Write(&self, pbuff: *const core::ffi::c_void, cb: super::ULONG) -> windows_core::Result<super::ULONG>;
    fn GetRate(&self) -> windows_core::Result<i32>;
    fn GetVolume(&self) -> windows_core::Result<super::USHORT>;
    fn GetSkipInfo(&self, petype: *mut SPVSKIPTYPE, plnumitems: *mut i32) -> windows_core::Result<()>;
    fn CompleteSkip(&self, ulnumskipped: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "sapi", feature = "shared"))]
impl ISpTTSEngineSite_Vtbl {
    pub const fn new<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActions<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTTSEngineSite_Impl::GetActions(this)
            }
        }
        unsafe extern "system" fn Write<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuff: *const core::ffi::c_void, cb: super::ULONG, pcbwritten: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpTTSEngineSite_Impl::Write(this, core::mem::transmute_copy(&pbuff), core::mem::transmute_copy(&cb)) {
                    Ok(ok__) => {
                        pcbwritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRate<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prateadjust: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpTTSEngineSite_Impl::GetRate(this) {
                    Ok(ok__) => {
                        prateadjust.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVolume<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusvolume: *mut super::USHORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpTTSEngineSite_Impl::GetVolume(this) {
                    Ok(ok__) => {
                        pusvolume.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSkipInfo<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, petype: *mut SPVSKIPTYPE, plnumitems: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTTSEngineSite_Impl::GetSkipInfo(this, core::mem::transmute_copy(&petype), core::mem::transmute_copy(&plnumitems)).into()
            }
        }
        unsafe extern "system" fn CompleteSkip<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulnumskipped: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTTSEngineSite_Impl::CompleteSkip(this, core::mem::transmute_copy(&ulnumskipped)).into()
            }
        }
        Self {
            base__: super::ISpEventSink_Vtbl::new::<Identity, OFFSET>(),
            GetActions: GetActions::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            GetRate: GetRate::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            GetSkipInfo: GetSkipInfo::<Identity, OFFSET>,
            CompleteSkip: CompleteSkip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTTSEngineSite as windows_core::Interface>::IID || iid == &<super::ISpEventSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "sapi", feature = "shared"))]
impl windows_core::RuntimeName for ISpTTSEngineSite {}
windows_core::imp::define_interface!(ISpTask, ISpTask_Vtbl);
impl ISpTask {
    pub unsafe fn Execute(&self, pvtaskdata: *mut core::ffi::c_void, pfcontinueprocessing: *const windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), pvtaskdata as _, pfcontinueprocessing) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpTask_Vtbl {
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ISpTask_Impl {
    fn Execute(&self, pvtaskdata: *mut core::ffi::c_void, pfcontinueprocessing: *const windows_core::BOOL) -> windows_core::Result<()>;
}
impl ISpTask_Vtbl {
    pub const fn new<Identity: ISpTask_Impl>() -> Self {
        unsafe extern "system" fn Execute<Identity: ISpTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, pfcontinueprocessing: *const windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ISpTask_Impl::Execute(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&pfcontinueprocessing)).into()
            }
        }
        Self { Execute: Execute::<Identity> }
    }
}
struct ISpTask_ImplVtbl<T: ISpTask_Impl>(core::marker::PhantomData<T>);
impl<T: ISpTask_Impl> ISpTask_ImplVtbl<T> {
    const VTABLE: ISpTask_Vtbl = ISpTask_Vtbl::new::<T>();
}
impl ISpTask {
    pub fn new<'a, T: ISpTask_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ISpTask_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ISpTaskManager, ISpTaskManager_Vtbl, 0x2baeef81_2ca3_4331_98f3_26ec5abefb03);
windows_core::imp::interface_hierarchy!(ISpTaskManager, windows_core::IUnknown);
impl ISpTaskManager {
    #[cfg(feature = "shared")]
    pub unsafe fn SetThreadPoolInfo(&self, ppoolinfo: *const SPTMTHREADINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetThreadPoolInfo)(windows_core::Interface::as_raw(self), ppoolinfo) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetThreadPoolInfo(&self) -> windows_core::Result<SPTMTHREADINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThreadPoolInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn QueueTask<P0>(&self, ptask: P0, pvtaskdata: *const core::ffi::c_void, hcompevent: super::HANDLE, pdwgroupid: *mut super::DWORD, ptaskid: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpTask>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueueTask)(windows_core::Interface::as_raw(self), ptask.param().abi(), pvtaskdata, hcompevent, pdwgroupid as _, ptaskid as _) }
    }
    #[cfg(all(feature = "sapi", feature = "winnt"))]
    pub unsafe fn CreateReoccurringTask<P0>(&self, ptask: P0, pvtaskdata: *const core::ffi::c_void, hcompevent: super::HANDLE) -> windows_core::Result<super::ISpNotifySink>
    where
        P0: windows_core::Param<ISpTask>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateReoccurringTask)(windows_core::Interface::as_raw(self), ptask.param().abi(), pvtaskdata, hcompevent, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "sapi")]
    pub unsafe fn CreateThreadControl<P0>(&self, ptask: P0, pvtaskdata: *const core::ffi::c_void, npriority: i32) -> windows_core::Result<ISpThreadControl>
    where
        P0: windows_core::Param<ISpThreadTask>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateThreadControl)(windows_core::Interface::as_raw(self), ptask.param().abi(), pvtaskdata, npriority, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn TerminateTask(&self, dwtaskid: super::DWORD, ulwaitperiod: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TerminateTask)(windows_core::Interface::as_raw(self), dwtaskid, ulwaitperiod) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn TerminateTaskGroup(&self, dwgroupid: super::DWORD, ulwaitperiod: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TerminateTaskGroup)(windows_core::Interface::as_raw(self), dwgroupid, ulwaitperiod) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpTaskManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetThreadPoolInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPTMTHREADINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetThreadPoolInfo: usize,
    #[cfg(feature = "shared")]
    pub GetThreadPoolInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPTMTHREADINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetThreadPoolInfo: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub QueueTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, super::HANDLE, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    QueueTask: usize,
    #[cfg(all(feature = "sapi", feature = "winnt"))]
    pub CreateReoccurringTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, super::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "winnt")))]
    CreateReoccurringTask: usize,
    #[cfg(feature = "sapi")]
    pub CreateThreadControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "sapi"))]
    CreateThreadControl: usize,
    #[cfg(feature = "shared")]
    pub TerminateTask: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    TerminateTask: usize,
    #[cfg(feature = "shared")]
    pub TerminateTaskGroup: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    TerminateTaskGroup: usize,
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
pub trait ISpTaskManager_Impl: windows_core::IUnknownImpl {
    fn SetThreadPoolInfo(&self, ppoolinfo: *const SPTMTHREADINFO) -> windows_core::Result<()>;
    fn GetThreadPoolInfo(&self) -> windows_core::Result<SPTMTHREADINFO>;
    fn QueueTask(&self, ptask: windows_core::Ref<ISpTask>, pvtaskdata: *const core::ffi::c_void, hcompevent: super::HANDLE, pdwgroupid: *mut super::DWORD, ptaskid: *mut super::DWORD) -> windows_core::Result<()>;
    fn CreateReoccurringTask(&self, ptask: windows_core::Ref<ISpTask>, pvtaskdata: *const core::ffi::c_void, hcompevent: super::HANDLE) -> windows_core::Result<super::ISpNotifySink>;
    fn CreateThreadControl(&self, ptask: windows_core::Ref<ISpThreadTask>, pvtaskdata: *const core::ffi::c_void, npriority: i32) -> windows_core::Result<ISpThreadControl>;
    fn TerminateTask(&self, dwtaskid: super::DWORD, ulwaitperiod: super::ULONG) -> windows_core::Result<()>;
    fn TerminateTaskGroup(&self, dwgroupid: super::DWORD, ulwaitperiod: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
impl ISpTaskManager_Vtbl {
    pub const fn new<Identity: ISpTaskManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetThreadPoolInfo<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoolinfo: *const SPTMTHREADINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTaskManager_Impl::SetThreadPoolInfo(this, core::mem::transmute_copy(&ppoolinfo)).into()
            }
        }
        unsafe extern "system" fn GetThreadPoolInfo<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoolinfo: *mut SPTMTHREADINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpTaskManager_Impl::GetThreadPoolInfo(this) {
                    Ok(ok__) => {
                        ppoolinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueTask<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut core::ffi::c_void, pvtaskdata: *const core::ffi::c_void, hcompevent: super::HANDLE, pdwgroupid: *mut super::DWORD, ptaskid: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTaskManager_Impl::QueueTask(this, core::mem::transmute_copy(&ptask), core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hcompevent), core::mem::transmute_copy(&pdwgroupid), core::mem::transmute_copy(&ptaskid)).into()
            }
        }
        unsafe extern "system" fn CreateReoccurringTask<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut core::ffi::c_void, pvtaskdata: *const core::ffi::c_void, hcompevent: super::HANDLE, pptaskctrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpTaskManager_Impl::CreateReoccurringTask(this, core::mem::transmute_copy(&ptask), core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hcompevent)) {
                    Ok(ok__) => {
                        pptaskctrl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateThreadControl<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut core::ffi::c_void, pvtaskdata: *const core::ffi::c_void, npriority: i32, pptaskctrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpTaskManager_Impl::CreateThreadControl(this, core::mem::transmute_copy(&ptask), core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&npriority)) {
                    Ok(ok__) => {
                        pptaskctrl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TerminateTask<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtaskid: super::DWORD, ulwaitperiod: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTaskManager_Impl::TerminateTask(this, core::mem::transmute_copy(&dwtaskid), core::mem::transmute_copy(&ulwaitperiod)).into()
            }
        }
        unsafe extern "system" fn TerminateTaskGroup<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwgroupid: super::DWORD, ulwaitperiod: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTaskManager_Impl::TerminateTaskGroup(this, core::mem::transmute_copy(&dwgroupid), core::mem::transmute_copy(&ulwaitperiod)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetThreadPoolInfo: SetThreadPoolInfo::<Identity, OFFSET>,
            GetThreadPoolInfo: GetThreadPoolInfo::<Identity, OFFSET>,
            QueueTask: QueueTask::<Identity, OFFSET>,
            CreateReoccurringTask: CreateReoccurringTask::<Identity, OFFSET>,
            CreateThreadControl: CreateThreadControl::<Identity, OFFSET>,
            TerminateTask: TerminateTask::<Identity, OFFSET>,
            TerminateTaskGroup: TerminateTaskGroup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTaskManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ISpTaskManager {}
#[cfg(feature = "sapi")]
windows_core::imp::define_interface!(ISpThreadControl, ISpThreadControl_Vtbl, 0xa6be4d73_4403_4358_b22d_0346e23b1764);
#[cfg(feature = "sapi")]
impl core::ops::Deref for ISpThreadControl {
    type Target = super::ISpNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "sapi")]
windows_core::imp::interface_hierarchy!(ISpThreadControl, windows_core::IUnknown, super::ISpNotifySink);
#[cfg(feature = "sapi")]
impl ISpThreadControl {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn StartThread(&self, dwflags: super::DWORD) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartThread)(windows_core::Interface::as_raw(self), dwflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn WaitForThreadDone(&self, fforcestop: bool, phrthreadresult: *mut windows_core::HRESULT, mstimeout: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForThreadDone)(windows_core::Interface::as_raw(self), fforcestop.into(), phrthreadresult as _, mstimeout) }
    }
    pub unsafe fn TerminateThread(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TerminateThread)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn ThreadHandle(&self) -> super::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).ThreadHandle)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ThreadId(&self) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).ThreadId)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn NotifyEvent(&self) -> super::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).NotifyEvent)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn WindowHandle(&self) -> super::HWND {
        unsafe { (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn ThreadCompleteEvent(&self) -> super::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).ThreadCompleteEvent)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn ExitThreadEvent(&self) -> super::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).ExitThreadEvent)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "sapi")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpThreadControl_Vtbl {
    pub base__: super::ISpNotifySink_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub StartThread: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    StartThread: usize,
    #[cfg(feature = "shared")]
    pub WaitForThreadDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::HRESULT, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WaitForThreadDone: usize,
    pub TerminateThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub ThreadHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HANDLE,
    #[cfg(not(feature = "winnt"))]
    ThreadHandle: usize,
    #[cfg(feature = "shared")]
    pub ThreadId: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    ThreadId: usize,
    #[cfg(feature = "winnt")]
    pub NotifyEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HANDLE,
    #[cfg(not(feature = "winnt"))]
    NotifyEvent: usize,
    #[cfg(feature = "windef")]
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HWND,
    #[cfg(not(feature = "windef"))]
    WindowHandle: usize,
    #[cfg(feature = "winnt")]
    pub ThreadCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HANDLE,
    #[cfg(not(feature = "winnt"))]
    ThreadCompleteEvent: usize,
    #[cfg(feature = "winnt")]
    pub ExitThreadEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HANDLE,
    #[cfg(not(feature = "winnt"))]
    ExitThreadEvent: usize,
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ISpThreadControl_Impl: super::ISpNotifySink_Impl {
    fn StartThread(&self, dwflags: super::DWORD) -> windows_core::Result<super::HWND>;
    fn WaitForThreadDone(&self, fforcestop: windows_core::BOOL, phrthreadresult: *mut windows_core::HRESULT, mstimeout: super::ULONG) -> windows_core::Result<()>;
    fn TerminateThread(&self) -> windows_core::Result<()>;
    fn ThreadHandle(&self) -> super::HANDLE;
    fn ThreadId(&self) -> super::DWORD;
    fn NotifyEvent(&self) -> super::HANDLE;
    fn WindowHandle(&self) -> super::HWND;
    fn ThreadCompleteEvent(&self) -> super::HANDLE;
    fn ExitThreadEvent(&self) -> super::HANDLE;
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "windef", feature = "winnt"))]
impl ISpThreadControl_Vtbl {
    pub const fn new<Identity: ISpThreadControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartThread<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, phwnd: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpThreadControl_Impl::StartThread(this, core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WaitForThreadDone<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforcestop: windows_core::BOOL, phrthreadresult: *mut windows_core::HRESULT, mstimeout: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::WaitForThreadDone(this, core::mem::transmute_copy(&fforcestop), core::mem::transmute_copy(&phrthreadresult), core::mem::transmute_copy(&mstimeout)).into()
            }
        }
        unsafe extern "system" fn TerminateThread<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::TerminateThread(this).into()
            }
        }
        unsafe extern "system" fn ThreadHandle<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::ThreadHandle(this)
            }
        }
        unsafe extern "system" fn ThreadId<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::ThreadId(this)
            }
        }
        unsafe extern "system" fn NotifyEvent<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::NotifyEvent(this)
            }
        }
        unsafe extern "system" fn WindowHandle<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HWND {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::WindowHandle(this)
            }
        }
        unsafe extern "system" fn ThreadCompleteEvent<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::ThreadCompleteEvent(this)
            }
        }
        unsafe extern "system" fn ExitThreadEvent<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpThreadControl_Impl::ExitThreadEvent(this)
            }
        }
        Self {
            base__: super::ISpNotifySink_Vtbl::new::<Identity, OFFSET>(),
            StartThread: StartThread::<Identity, OFFSET>,
            WaitForThreadDone: WaitForThreadDone::<Identity, OFFSET>,
            TerminateThread: TerminateThread::<Identity, OFFSET>,
            ThreadHandle: ThreadHandle::<Identity, OFFSET>,
            ThreadId: ThreadId::<Identity, OFFSET>,
            NotifyEvent: NotifyEvent::<Identity, OFFSET>,
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            ThreadCompleteEvent: ThreadCompleteEvent::<Identity, OFFSET>,
            ExitThreadEvent: ExitThreadEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpThreadControl as windows_core::Interface>::IID || iid == &<super::ISpNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ISpThreadControl {}
windows_core::imp::define_interface!(ISpThreadTask, ISpThreadTask_Vtbl);
impl ISpThreadTask {
    #[cfg(feature = "windef")]
    pub unsafe fn InitThread(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitThread)(windows_core::Interface::as_raw(self), pvtaskdata as _, hwnd as _) }
    }
    #[cfg(all(feature = "windef", feature = "winnt"))]
    pub unsafe fn ThreadProc(&self, pvtaskdata: *mut core::ffi::c_void, hexitthreadevent: super::HANDLE, hnotifyevent: super::HANDLE, hwndworker: super::HWND, pfcontinueprocessing: *const windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ThreadProc)(windows_core::Interface::as_raw(self), pvtaskdata as _, hexitthreadevent as _, hnotifyevent as _, hwndworker as _, pfcontinueprocessing) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub unsafe fn WindowMessage(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: super::HWND, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
        unsafe { (windows_core::Interface::vtable(self).WindowMessage)(windows_core::Interface::as_raw(self), pvtaskdata as _, hwnd as _, msg, wparam, lparam) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpThreadTask_Vtbl {
    #[cfg(feature = "windef")]
    pub InitThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    InitThread: usize,
    #[cfg(all(feature = "windef", feature = "winnt"))]
    pub ThreadProc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::HANDLE, super::HANDLE, super::HWND, *const windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "windef", feature = "winnt")))]
    ThreadProc: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub WindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::HWND, super::UINT, super::WPARAM, super::LPARAM) -> super::LRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef")))]
    WindowMessage: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ISpThreadTask_Impl {
    fn InitThread(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: super::HWND) -> windows_core::Result<()>;
    fn ThreadProc(&self, pvtaskdata: *mut core::ffi::c_void, hexitthreadevent: super::HANDLE, hnotifyevent: super::HANDLE, hwndworker: super::HWND, pfcontinueprocessing: *const windows_core::BOOL) -> windows_core::Result<()>;
    fn WindowMessage(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: super::HWND, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl ISpThreadTask_Vtbl {
    pub const fn new<Identity: ISpThreadTask_Impl>() -> Self {
        unsafe extern "system" fn InitThread<Identity: ISpThreadTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, hwnd: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ISpThreadTask_Impl::InitThread(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn ThreadProc<Identity: ISpThreadTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, hexitthreadevent: super::HANDLE, hnotifyevent: super::HANDLE, hwndworker: super::HWND, pfcontinueprocessing: *const windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ISpThreadTask_Impl::ThreadProc(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hexitthreadevent), core::mem::transmute_copy(&hnotifyevent), core::mem::transmute_copy(&hwndworker), core::mem::transmute_copy(&pfcontinueprocessing)).into()
            }
        }
        unsafe extern "system" fn WindowMessage<Identity: ISpThreadTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, hwnd: super::HWND, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ISpThreadTask_Impl::WindowMessage(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam))
            }
        }
        Self { InitThread: InitThread::<Identity>, ThreadProc: ThreadProc::<Identity>, WindowMessage: WindowMessage::<Identity> }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
struct ISpThreadTask_ImplVtbl<T: ISpThreadTask_Impl>(core::marker::PhantomData<T>);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl<T: ISpThreadTask_Impl> ISpThreadTask_ImplVtbl<T> {
    const VTABLE: ISpThreadTask_Vtbl = ISpThreadTask_Vtbl::new::<T>();
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl ISpThreadTask {
    pub fn new<'a, T: ISpThreadTask_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ISpThreadTask_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ISpTokenUI, ISpTokenUI_Vtbl, 0xf8e690f0_39cb_4843_b8d7_c84696e1119d);
windows_core::imp::interface_hierarchy!(ISpTokenUI, windows_core::IUnknown);
impl ISpTokenUI {
    #[cfg(feature = "shared")]
    pub unsafe fn IsUISupported<P0, P3>(&self, psztypeofui: P0, pvextradata: *const core::ffi::c_void, cbextradata: super::ULONG, punkobject: P3) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), psztypeofui.param().abi(), pvextradata, cbextradata, punkobject.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "sapi", feature = "shared", feature = "windef"))]
    pub unsafe fn DisplayUI<P1, P2, P5, P6>(&self, hwndparent: super::HWND, psztitle: P1, psztypeofui: P2, pvextradata: *const core::ffi::c_void, cbextradata: super::ULONG, ptoken: P5, punkobject: P6) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::ISpObjectToken>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwndparent, psztitle.param().abi(), psztypeofui.param().abi(), pvextradata, cbextradata, ptoken.param().abi(), punkobject.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpTokenUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, super::ULONG, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    IsUISupported: usize,
    #[cfg(all(feature = "sapi", feature = "shared", feature = "windef"))]
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, windows_core::PCWSTR, windows_core::PCWSTR, *const core::ffi::c_void, super::ULONG, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "sapi", feature = "shared", feature = "windef")))]
    DisplayUI: usize,
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "windef"))]
pub trait ISpTokenUI_Impl: windows_core::IUnknownImpl {
    fn IsUISupported(&self, psztypeofui: &windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: super::ULONG, punkobject: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::BOOL>;
    fn DisplayUI(&self, hwndparent: super::HWND, psztitle: &windows_core::PCWSTR, psztypeofui: &windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: super::ULONG, ptoken: windows_core::Ref<super::ISpObjectToken>, punkobject: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "windef"))]
impl ISpTokenUI_Vtbl {
    pub const fn new<Identity: ISpTokenUI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsUISupported<Identity: ISpTokenUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztypeofui: windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: super::ULONG, punkobject: *mut core::ffi::c_void, pfsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpTokenUI_Impl::IsUISupported(this, core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), core::mem::transmute_copy(&punkobject)) {
                    Ok(ok__) => {
                        pfsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpTokenUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, psztitle: windows_core::PCWSTR, psztypeofui: windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: super::ULONG, ptoken: *mut core::ffi::c_void, punkobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpTokenUI_Impl::DisplayUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&psztitle), core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), core::mem::transmute_copy(&ptoken), core::mem::transmute_copy(&punkobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTokenUI as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sapi", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ISpTokenUI {}
pub const SPALTERNATESCLSID: windows_core::PCWSTR = windows_core::w!("AlternatesCLSID");
pub type SPCFGNOTIFY = i32;
pub const SPCFGN_ACTIVATE: SPCFGNOTIFY = 3;
pub const SPCFGN_ADD: SPCFGNOTIFY = 0;
pub const SPCFGN_DEACTIVATE: SPCFGNOTIFY = 4;
pub const SPCFGN_INVALIDATE: SPCFGNOTIFY = 2;
pub const SPCFGN_REMOVE: SPCFGNOTIFY = 1;
pub type SPGRAMMARHANDLE = *mut SPGRAMMARHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPGRAMMARHANDLE__ {
    pub unused: i32,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPPARSEINFO {
    pub cbSize: super::ULONG,
    pub hRule: SPRULEHANDLE,
    pub ullAudioStreamPosition: super::ULONGLONG,
    pub ulAudioSize: super::ULONG,
    pub cTransitions: super::ULONG,
    pub pPath: *mut SPPATHENTRY,
    pub SREngineID: windows_core::GUID,
    pub ulSREnginePrivateDataSize: super::ULONG,
    pub pSREnginePrivateData: *const super::BYTE,
    pub fHypothesis: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPPATHENTRY {
    pub hTransition: SPTRANSITIONID,
    pub elem: super::SPPHRASEELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SPPHRASEALT {
    pub pPhrase: core::mem::ManuallyDrop<Option<ISpPhraseBuilder>>,
    pub ulStartElementInParent: super::ULONG,
    pub cElementsInParent: super::ULONG,
    pub cElementsInAlternate: super::ULONG,
    pub pvAltExtra: *mut core::ffi::c_void,
    pub cbAltExtra: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SPPHRASEALTREQUEST {
    pub ulStartElement: super::ULONG,
    pub cElements: super::ULONG,
    pub ulRequestAltCount: super::ULONG,
    pub pvResultExtra: *mut core::ffi::c_void,
    pub cbResultExtra: super::ULONG,
    pub pPhrase: core::mem::ManuallyDrop<Option<super::ISpPhrase>>,
    pub pRecoContext: core::mem::ManuallyDrop<Option<super::ISpRecoContext>>,
}
pub type SPPHRASEPROPERTYHANDLE = *mut SPPHRASEPROPERTYHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPPHRASEPROPERTYHANDLE__ {
    pub unused: i32,
}
pub type SPPHRASERULEHANDLE = *mut SPPHRASERULEHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPPHRASERULEHANDLE__ {
    pub unused: i32,
}
pub type SPPROPSRC = i32;
pub const SPPROPSRC_RECO_CTX: SPPROPSRC = 1;
pub const SPPROPSRC_RECO_GRAMMAR: SPPROPSRC = 2;
pub const SPPROPSRC_RECO_INST: SPPROPSRC = 0;
pub type SPRECOCONTEXTHANDLE = *mut SPRECOCONTEXTHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPRECOCONTEXTHANDLE__ {
    pub unused: i32,
}
pub const SPRECOEXTENSION: windows_core::PCWSTR = windows_core::w!("RecoExtension");
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SPRECORESULTINFO {
    pub cbSize: super::ULONG,
    pub eResultType: SPRESULTTYPE,
    pub fHypothesis: windows_core::BOOL,
    pub fProprietaryAutoPause: windows_core::BOOL,
    pub ullStreamPosStart: super::ULONGLONG,
    pub ullStreamPosEnd: super::ULONGLONG,
    pub hGrammar: SPGRAMMARHANDLE,
    pub ulSizeEngineData: super::ULONG,
    pub pvEngineData: *mut core::ffi::c_void,
    pub pPhrase: core::mem::ManuallyDrop<Option<ISpPhraseBuilder>>,
    pub aPhraseAlts: *mut SPPHRASEALT,
    pub ulNumAlts: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SPRECORESULTINFOEX {
    pub base: SPRECORESULTINFO,
    pub ullStreamTimeStart: super::ULONGLONG,
    pub ullStreamTimeEnd: super::ULONGLONG,
}
pub type SPRESULTTYPE = i32;
pub const SPRIO_NONE: SPRULEINFOOPT = 0;
pub const SPRT_CFG: SPRESULTTYPE = 0;
pub const SPRT_EMULATED: SPRESULTTYPE = 8;
pub const SPRT_EXTENDABLE_PARSE: SPRESULTTYPE = 16;
pub const SPRT_FALSE_RECOGNITION: SPRESULTTYPE = 4;
pub const SPRT_PROPRIETARY: SPRESULTTYPE = 2;
pub const SPRT_SLM: SPRESULTTYPE = 1;
pub const SPRT_TYPE_MASK: SPRESULTTYPE = 3;
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPRULEENTRY {
    pub hRule: SPRULEHANDLE,
    pub hInitialState: super::SPSTATEHANDLE,
    pub Attributes: super::DWORD,
    pub pvClientRuleContext: *mut core::ffi::c_void,
    pub pvClientGrammarContext: *mut core::ffi::c_void,
}
pub type SPRULEHANDLE = *mut SPRULEHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPRULEHANDLE__ {
    pub unused: i32,
}
pub type SPRULEINFOOPT = i32;
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPSTATEINFO {
    pub cAllocatedEntries: super::ULONG,
    pub pTransitions: *mut SPTRANSITIONENTRY,
    pub cEpsilons: super::ULONG,
    pub cRules: super::ULONG,
    pub cWords: super::ULONG,
    pub cSpecialTransitions: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPTMTHREADINFO {
    pub lPoolSize: i32,
    pub lPriority: i32,
    pub ulConcurrencyLimit: super::ULONG,
    pub ulMaxQuickAllocThreads: super::ULONG,
}
pub const SPTRANSDICTATION: SPTRANSITIONTYPE = 5;
pub const SPTRANSEPSILON: SPTRANSITIONTYPE = 0;
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct SPTRANSITIONENTRY {
    pub ID: SPTRANSITIONID,
    pub hNextState: super::SPSTATEHANDLE,
    pub Type: super::BYTE,
    pub RequiredConfidence: i8,
    pub Anonymous: SPTRANSITIONENTRY_0,
    pub Weight: f32,
    pub Anonymous2: SPTRANSITIONENTRY_1,
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl Default for SPTRANSITIONENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_0 {
    pub fHasProperty: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy)]
pub union SPTRANSITIONENTRY_1 {
    pub Anonymous: SPTRANSITIONENTRY_1_0,
    pub Anonymous2: SPTRANSITIONENTRY_1_1,
    pub Anonymous3: SPTRANSITIONENTRY_1_2,
}
#[cfg(all(feature = "sapi", feature = "shared"))]
impl Default for SPTRANSITIONENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_1_0 {
    pub hRuleInitialState: super::SPSTATEHANDLE,
    pub hRule: SPRULEHANDLE,
    pub pvClientRuleContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_1_1 {
    pub hWord: SPWORDHANDLE,
    pub pvClientWordContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_1_2 {
    pub pvGrammarCookie: *mut core::ffi::c_void,
}
pub type SPTRANSITIONID = *mut SPTRANSITIONID__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPTRANSITIONID__ {
    pub unused: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct SPTRANSITIONPROPERTY {
    pub pszName: windows_core::PCWSTR,
    pub ulId: super::ULONG,
    pub pszValue: windows_core::PCWSTR,
    pub vValue: super::VARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for SPTRANSITIONPROPERTY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SPTRANSITIONPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPTRANSITIONTYPE = i32;
pub const SPTRANSRULE: SPTRANSITIONTYPE = 2;
pub const SPTRANSTEXTBUF: SPTRANSITIONTYPE = 3;
pub const SPTRANSWILDCARD: SPTRANSITIONTYPE = 4;
pub const SPTRANSWORD: SPTRANSITIONTYPE = 1;
pub type SPVESACTIONS = i32;
pub const SPVES_ABORT: SPVESACTIONS = 1;
pub const SPVES_CONTINUE: SPVESACTIONS = 0;
pub const SPVES_RATE: SPVESACTIONS = 4;
pub const SPVES_SKIP: SPVESACTIONS = 2;
pub const SPVES_VOLUME: SPVESACTIONS = 8;
pub type SPVSKIPTYPE = i32;
pub const SPVST_SENTENCE: SPVSKIPTYPE = 1;
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPVTEXTFRAG {
    pub pNext: *mut Self,
    pub State: super::SPVSTATE,
    pub pTextStart: windows_core::PCWSTR,
    pub ulTextLen: super::ULONG,
    pub ulTextSrcOffset: super::ULONG,
}
pub const SPWIO_NONE: SPWORDINFOOPT = 0;
pub const SPWIO_WANT_TEXT: SPWORDINFOOPT = 1;
#[repr(C)]
#[cfg(all(feature = "sapi", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPWORDENTRY {
    pub hWord: SPWORDHANDLE,
    pub LangID: super::WORD,
    pub pszDisplayText: *mut super::WCHAR,
    pub pszLexicalForm: *mut super::WCHAR,
    pub aPhoneId: *mut super::SPPHONEID,
    pub pvClientContext: *mut core::ffi::c_void,
}
pub type SPWORDHANDLE = *mut SPWORDHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPWORDHANDLE__ {
    pub unused: i32,
}
pub type SPWORDINFOOPT = i32;
pub const SR_LOCALIZED_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const SpDataKey: windows_core::GUID = windows_core::GUID::from_u128(0xd9f6ee60_58c9_458b_88e1_2f908fd7f87c);
pub const SpGramCompBackend: windows_core::GUID = windows_core::GUID::from_u128(0xda93e903_c843_11d2_a084_00c04f8ef9b5);
pub const SpGrammarCompiler: windows_core::GUID = windows_core::GUID::from_u128(0xb1e29d59_a675_11d2_8302_00c04f8ee6c0);
pub const SpITNProcessor: windows_core::GUID = windows_core::GUID::from_u128(0x12d73610_a1c9_11d3_bc90_00c04f72df9f);
pub const SpObjectTokenEnum: windows_core::GUID = windows_core::GUID::from_u128(0x3918d75f_0acb_41f2_b733_92aa15bcecf6);
pub const SpPhraseBuilder: windows_core::GUID = windows_core::GUID::from_u128(0x777b6bbd_2ff2_11d3_88fe_00c04f8ef9b5);
pub const SpW3CGrammarCompiler: windows_core::GUID = windows_core::GUID::from_u128(0xd2c13906_51ef_454e_bc67_a52475ff074c);
windows_core::imp::define_interface!(_ISpPrivateEngineCall, _ISpPrivateEngineCall_Vtbl, 0x8e7c791e_4467_11d3_9723_00c04f72db08);
windows_core::imp::interface_hierarchy!(_ISpPrivateEngineCall, windows_core::IUnknown);
impl _ISpPrivateEngineCall {
    #[cfg(feature = "shared")]
    pub unsafe fn CallEngine(&self, pcallframe: *mut core::ffi::c_void, ulcallframesize: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CallEngine)(windows_core::Interface::as_raw(self), pcallframe as _, ulcallframesize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CallEngineEx(&self, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CallEngineEx)(windows_core::Interface::as_raw(self), pinframe, ulinframesize, ppcomemoutframe as _, puloutframesize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ISpPrivateEngineCall_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub CallEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CallEngine: usize,
    #[cfg(feature = "shared")]
    pub CallEngineEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CallEngineEx: usize,
}
#[cfg(feature = "shared")]
pub trait _ISpPrivateEngineCall_Impl: windows_core::IUnknownImpl {
    fn CallEngine(&self, pcallframe: *mut core::ffi::c_void, ulcallframesize: super::ULONG) -> windows_core::Result<()>;
    fn CallEngineEx(&self, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl _ISpPrivateEngineCall_Vtbl {
    pub const fn new<Identity: _ISpPrivateEngineCall_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallEngine<Identity: _ISpPrivateEngineCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallframe: *mut core::ffi::c_void, ulcallframesize: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISpPrivateEngineCall_Impl::CallEngine(this, core::mem::transmute_copy(&pcallframe), core::mem::transmute_copy(&ulcallframesize)).into()
            }
        }
        unsafe extern "system" fn CallEngineEx<Identity: _ISpPrivateEngineCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinframe: *const core::ffi::c_void, ulinframesize: super::ULONG, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISpPrivateEngineCall_Impl::CallEngineEx(this, core::mem::transmute_copy(&pinframe), core::mem::transmute_copy(&ulinframesize), core::mem::transmute_copy(&ppcomemoutframe), core::mem::transmute_copy(&puloutframesize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CallEngine: CallEngine::<Identity, OFFSET>,
            CallEngineEx: CallEngineEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ISpPrivateEngineCall as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for _ISpPrivateEngineCall {}
