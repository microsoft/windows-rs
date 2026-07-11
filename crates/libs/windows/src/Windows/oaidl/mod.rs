#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
#[cfg(feature = "wtypes")]
impl Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: core::mem::ManuallyDrop<Option<ITypeComp>>,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Clone for BINDPTR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for BINDPTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CALLCONV = i32;
pub const CC_CDECL: CALLCONV = 1;
pub const CC_FASTCALL: CALLCONV = 0;
pub const CC_FPFASTCALL: CALLCONV = 5;
pub const CC_MACPASCAL: CALLCONV = 3;
pub const CC_MAX: CALLCONV = 9;
pub const CC_MPWCDECL: CALLCONV = 7;
pub const CC_MPWPASCAL: CALLCONV = 8;
pub const CC_MSCPASCAL: CALLCONV = 2;
pub const CC_PASCAL: CALLCONV = 2;
pub const CC_STDCALL: CALLCONV = 4;
pub const CC_SYSCALL: CALLCONV = 6;
pub type CHANGEKIND = i32;
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = 0;
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = 6;
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = 1;
pub const CHANGEKIND_GENERAL: CHANGEKIND = 4;
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = 5;
pub const CHANGEKIND_MAX: CHANGEKIND = 7;
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = 3;
pub const CHANGEKIND_SETNAMES: CHANGEKIND = 2;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pStorage: *mut core::ffi::c_void,
    pub flags: u32,
}
impl Default for CLEANLOCALSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wtypes")]
pub type CURRENCY = super::wtypes::CY;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: LPCUSTDATAITEM,
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub struct CUSTDATAITEM {
    pub guid: windows_core::GUID,
    pub varValue: VARIANTARG,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DESCKIND = i32;
pub const DESCKIND_FUNCDESC: DESCKIND = 1;
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = 4;
pub const DESCKIND_MAX: DESCKIND = 5;
pub const DESCKIND_NONE: DESCKIND = 0;
pub const DESCKIND_TYPECOMP: DESCKIND = 3;
pub const DESCKIND_VARDESC: DESCKIND = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DISPID(pub i32);
pub const DISPID_COLLECT: i32 = -8;
pub const DISPID_CONSTRUCTOR: i32 = -6;
pub const DISPID_DESTRUCTOR: i32 = -7;
pub const DISPID_EVALUATE: i32 = -5;
pub const DISPID_NEWENUM: i32 = -4;
pub const DISPID_PROPERTYPUT: i32 = -3;
pub const DISPID_UNKNOWN: i32 = -1;
pub const DISPID_VALUE: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANTARG,
    pub rgdispidNamedArgs: *mut DISPID,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for ELEMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Debug, PartialEq)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrDescription: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrHelpFile: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dwHelpContext: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub pfnDeferredFillIn: *mut u8,
    pub scode: super::wtypesbase::SCODE,
}
#[cfg(feature = "wtypesbase")]
impl Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FADF_AUTO: u32 = 1;
pub const FADF_BSTR: u32 = 256;
pub const FADF_DISPATCH: u32 = 1024;
pub const FADF_EMBEDDED: u32 = 4;
pub const FADF_FIXEDSIZE: u32 = 16;
pub const FADF_HAVEIID: u32 = 64;
pub const FADF_HAVEVARTYPE: u32 = 128;
pub const FADF_RECORD: u32 = 32;
pub const FADF_RESERVED: u32 = 61448;
pub const FADF_STATIC: u32 = 2;
pub const FADF_UNKNOWN: u32 = 512;
pub const FADF_VARIANT: u32 = 2048;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FUNCDESC {
    pub memid: MEMBERID,
    pub lprgscode: *mut super::wtypesbase::SCODE,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for FUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FUNCFLAGS = i32;
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = 4;
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = 32;
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = 256;
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = 16;
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = 64;
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = 4096;
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = 1024;
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = 2048;
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = 8;
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = 1;
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = 2;
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = 512;
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = 128;
pub type FUNCKIND = i32;
pub const FUNC_DISPATCH: FUNCKIND = 4;
pub const FUNC_NONVIRTUAL: FUNCKIND = 2;
pub const FUNC_PUREVIRTUAL: FUNCKIND = 1;
pub const FUNC_STATIC: FUNCKIND = 3;
pub const FUNC_VIRTUAL: FUNCKIND = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HREFTYPE(pub u32);
windows_core::imp::define_interface!(ICreateErrorInfo, ICreateErrorInfo_Vtbl, 0x22f03340_547d_101b_8e65_08002b2bd119);
windows_core::imp::interface_hierarchy!(ICreateErrorInfo, windows_core::IUnknown);
impl ICreateErrorInfo {
    pub unsafe fn SetGUID(&self, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGUID)(windows_core::Interface::as_raw(self), rguid) }
    }
    pub unsafe fn SetSource<P0>(&self, szsource: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), szsource.param().abi()) }
    }
    pub unsafe fn SetDescription<P0>(&self, szdescription: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), szdescription.param().abi()) }
    }
    pub unsafe fn SetHelpFile<P0>(&self, szhelpfile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHelpFile)(windows_core::Interface::as_raw(self), szhelpfile.param().abi()) }
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ICreateErrorInfo_Impl: windows_core::IUnknownImpl {
    fn SetGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetSource(&self, szsource: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetDescription(&self, szdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpFile(&self, szhelpfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()>;
}
impl ICreateErrorInfo_Vtbl {
    pub const fn new<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGUID<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetGUID(this, core::mem::transmute_copy(&rguid)).into()
            }
        }
        unsafe extern "system" fn SetSource<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsource: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetSource(this, core::mem::transmute(&szsource)).into()
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetDescription(this, core::mem::transmute(&szdescription)).into()
            }
        }
        unsafe extern "system" fn SetHelpFile<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhelpfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetHelpFile(this, core::mem::transmute(&szhelpfile)).into()
            }
        }
        unsafe extern "system" fn SetHelpContext<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGUID: SetGUID::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            SetHelpFile: SetHelpFile::<Identity, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICreateErrorInfo {}
windows_core::imp::define_interface!(ICreateTypeInfo, ICreateTypeInfo_Vtbl, 0x00020405_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICreateTypeInfo, windows_core::IUnknown);
impl ICreateTypeInfo {
    pub unsafe fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid) }
    }
    pub unsafe fn SetTypeFlags(&self, utypeflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypeFlags)(windows_core::Interface::as_raw(self), utypeflags) }
    }
    pub unsafe fn SetDocString<P0>(&self, pstrdoc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocString)(windows_core::Interface::as_raw(self), pstrdoc.param().abi()) }
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext) }
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), wmajorvernum, wminorvernum) }
    }
    pub unsafe fn AddRefTypeInfo<P0>(&self, ptinfo: P0, phreftype: *const HREFTYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITypeInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRefTypeInfo)(windows_core::Interface::as_raw(self), ptinfo.param().abi(), phreftype) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddFuncDesc(&self, index: u32, pfuncdesc: *const FUNCDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddFuncDesc)(windows_core::Interface::as_raw(self), index, pfuncdesc) }
    }
    pub unsafe fn AddImplType(&self, index: u32, hreftype: HREFTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddImplType)(windows_core::Interface::as_raw(self), index, hreftype) }
    }
    pub unsafe fn SetImplTypeFlags(&self, index: u32, impltypeflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetImplTypeFlags)(windows_core::Interface::as_raw(self), index, impltypeflags) }
    }
    pub unsafe fn SetAlignment(&self, cbalignment: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAlignment)(windows_core::Interface::as_raw(self), cbalignment) }
    }
    pub unsafe fn SetSchema<P0>(&self, pstrschema: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSchema)(windows_core::Interface::as_raw(self), pstrschema.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddVarDesc(&self, index: u32, pvardesc: *const VARDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddVarDesc)(windows_core::Interface::as_raw(self), index, pvardesc) }
    }
    pub unsafe fn SetFuncAndParamNames(&self, index: u32, rgsznames: *const windows_core::PCWSTR, cnames: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncAndParamNames)(windows_core::Interface::as_raw(self), index, rgsznames, cnames) }
    }
    pub unsafe fn SetVarName<P1>(&self, index: u32, szname: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVarName)(windows_core::Interface::as_raw(self), index, szname.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetTypeDescAlias(&self, ptdescalias: *const TYPEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypeDescAlias)(windows_core::Interface::as_raw(self), ptdescalias) }
    }
    pub unsafe fn DefineFuncAsDllEntry<P1, P2>(&self, index: u32, szdllname: P1, szprocname: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DefineFuncAsDllEntry)(windows_core::Interface::as_raw(self), index, szdllname.param().abi(), szprocname.param().abi()) }
    }
    pub unsafe fn SetFuncDocString<P1>(&self, index: u32, szdocstring: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFuncDocString)(windows_core::Interface::as_raw(self), index, szdocstring.param().abi()) }
    }
    pub unsafe fn SetVarDocString<P1>(&self, index: u32, szdocstring: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVarDocString)(windows_core::Interface::as_raw(self), index, szdocstring.param().abi()) }
    }
    pub unsafe fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncHelpContext)(windows_core::Interface::as_raw(self), index, dwhelpcontext) }
    }
    pub unsafe fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarHelpContext)(windows_core::Interface::as_raw(self), index, dwhelpcontext) }
    }
    pub unsafe fn SetMops(&self, index: u32, bstrmops: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMops)(windows_core::Interface::as_raw(self), index, core::mem::transmute_copy(bstrmops)) }
    }
    pub unsafe fn SetTypeIdldesc(&self, pidldesc: *const IDLDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypeIdldesc)(windows_core::Interface::as_raw(self), pidldesc) }
    }
    pub unsafe fn LayOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LayOut)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub AddRefTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const HREFTYPE) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const FUNCDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddFuncDesc: usize,
    pub AddImplType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, HREFTYPE) -> windows_core::HRESULT,
    pub SetImplTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SetSchema: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const VARDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddVarDesc: usize,
    pub SetFuncAndParamNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetVarName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetTypeDescAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const TYPEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetTypeDescAlias: usize,
    pub DefineFuncAsDllEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFuncDocString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetVarDocString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFuncHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetVarHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetMops: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTypeIdldesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const IDLDESC) -> windows_core::HRESULT,
    pub LayOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub trait ICreateTypeInfo_Impl: windows_core::IUnknownImpl {
    fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetTypeFlags(&self, utypeflags: u32) -> windows_core::Result<()>;
    fn SetDocString(&self, pstrdoc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::Result<()>;
    fn AddRefTypeInfo(&self, ptinfo: windows_core::Ref<ITypeInfo>, phreftype: *const HREFTYPE) -> windows_core::Result<()>;
    fn AddFuncDesc(&self, index: u32, pfuncdesc: *const FUNCDESC) -> windows_core::Result<()>;
    fn AddImplType(&self, index: u32, hreftype: HREFTYPE) -> windows_core::Result<()>;
    fn SetImplTypeFlags(&self, index: u32, impltypeflags: i32) -> windows_core::Result<()>;
    fn SetAlignment(&self, cbalignment: u16) -> windows_core::Result<()>;
    fn SetSchema(&self, pstrschema: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddVarDesc(&self, index: u32, pvardesc: *const VARDESC) -> windows_core::Result<()>;
    fn SetFuncAndParamNames(&self, index: u32, rgsznames: *const windows_core::PCWSTR, cnames: u32) -> windows_core::Result<()>;
    fn SetVarName(&self, index: u32, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetTypeDescAlias(&self, ptdescalias: *const TYPEDESC) -> windows_core::Result<()>;
    fn DefineFuncAsDllEntry(&self, index: u32, szdllname: &windows_core::PCWSTR, szprocname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFuncDocString(&self, index: u32, szdocstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetVarDocString(&self, index: u32, szdocstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetMops(&self, index: u32, bstrmops: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTypeIdldesc(&self, pidldesc: *const IDLDESC) -> windows_core::Result<()>;
    fn LayOut(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl ICreateTypeInfo_Vtbl {
    pub const fn new<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGuid<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetGuid(this, core::mem::transmute_copy(&guid)).into()
            }
        }
        unsafe extern "system" fn SetTypeFlags<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, utypeflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetTypeFlags(this, core::mem::transmute_copy(&utypeflags)).into()
            }
        }
        unsafe extern "system" fn SetDocString<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdoc: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetDocString(this, core::mem::transmute(&pstrdoc)).into()
            }
        }
        unsafe extern "system" fn SetHelpContext<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetVersion(this, core::mem::transmute_copy(&wmajorvernum), core::mem::transmute_copy(&wminorvernum)).into()
            }
        }
        unsafe extern "system" fn AddRefTypeInfo<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptinfo: *mut core::ffi::c_void, phreftype: *const HREFTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::AddRefTypeInfo(this, core::mem::transmute_copy(&ptinfo), core::mem::transmute_copy(&phreftype)).into()
            }
        }
        unsafe extern "system" fn AddFuncDesc<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pfuncdesc: *const FUNCDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::AddFuncDesc(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pfuncdesc)).into()
            }
        }
        unsafe extern "system" fn AddImplType<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, hreftype: HREFTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::AddImplType(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&hreftype)).into()
            }
        }
        unsafe extern "system" fn SetImplTypeFlags<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, impltypeflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetImplTypeFlags(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&impltypeflags)).into()
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbalignment: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetAlignment(this, core::mem::transmute_copy(&cbalignment)).into()
            }
        }
        unsafe extern "system" fn SetSchema<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrschema: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetSchema(this, core::mem::transmute(&pstrschema)).into()
            }
        }
        unsafe extern "system" fn AddVarDesc<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pvardesc: *const VARDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::AddVarDesc(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pvardesc)).into()
            }
        }
        unsafe extern "system" fn SetFuncAndParamNames<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, rgsznames: *const windows_core::PCWSTR, cnames: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetFuncAndParamNames(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames)).into()
            }
        }
        unsafe extern "system" fn SetVarName<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetVarName(this, core::mem::transmute_copy(&index), core::mem::transmute(&szname)).into()
            }
        }
        unsafe extern "system" fn SetTypeDescAlias<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptdescalias: *const TYPEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetTypeDescAlias(this, core::mem::transmute_copy(&ptdescalias)).into()
            }
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szdllname: windows_core::PCWSTR, szprocname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::DefineFuncAsDllEntry(this, core::mem::transmute_copy(&index), core::mem::transmute(&szdllname), core::mem::transmute(&szprocname)).into()
            }
        }
        unsafe extern "system" fn SetFuncDocString<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szdocstring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetFuncDocString(this, core::mem::transmute_copy(&index), core::mem::transmute(&szdocstring)).into()
            }
        }
        unsafe extern "system" fn SetVarDocString<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szdocstring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetVarDocString(this, core::mem::transmute_copy(&index), core::mem::transmute(&szdocstring)).into()
            }
        }
        unsafe extern "system" fn SetFuncHelpContext<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetFuncHelpContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetVarHelpContext<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetVarHelpContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetMops<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, bstrmops: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetMops(this, core::mem::transmute_copy(&index), core::mem::transmute(&bstrmops)).into()
            }
        }
        unsafe extern "system" fn SetTypeIdldesc<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidldesc: *const IDLDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetTypeIdldesc(this, core::mem::transmute_copy(&pidldesc)).into()
            }
        }
        unsafe extern "system" fn LayOut<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::LayOut(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGuid: SetGuid::<Identity, OFFSET>,
            SetTypeFlags: SetTypeFlags::<Identity, OFFSET>,
            SetDocString: SetDocString::<Identity, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            AddRefTypeInfo: AddRefTypeInfo::<Identity, OFFSET>,
            AddFuncDesc: AddFuncDesc::<Identity, OFFSET>,
            AddImplType: AddImplType::<Identity, OFFSET>,
            SetImplTypeFlags: SetImplTypeFlags::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            SetSchema: SetSchema::<Identity, OFFSET>,
            AddVarDesc: AddVarDesc::<Identity, OFFSET>,
            SetFuncAndParamNames: SetFuncAndParamNames::<Identity, OFFSET>,
            SetVarName: SetVarName::<Identity, OFFSET>,
            SetTypeDescAlias: SetTypeDescAlias::<Identity, OFFSET>,
            DefineFuncAsDllEntry: DefineFuncAsDllEntry::<Identity, OFFSET>,
            SetFuncDocString: SetFuncDocString::<Identity, OFFSET>,
            SetVarDocString: SetVarDocString::<Identity, OFFSET>,
            SetFuncHelpContext: SetFuncHelpContext::<Identity, OFFSET>,
            SetVarHelpContext: SetVarHelpContext::<Identity, OFFSET>,
            SetMops: SetMops::<Identity, OFFSET>,
            SetTypeIdldesc: SetTypeIdldesc::<Identity, OFFSET>,
            LayOut: LayOut::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICreateTypeInfo {}
windows_core::imp::define_interface!(ICreateTypeInfo2, ICreateTypeInfo2_Vtbl, 0x0002040e_0000_0000_c000_000000000046);
impl core::ops::Deref for ICreateTypeInfo2 {
    type Target = ICreateTypeInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateTypeInfo2, windows_core::IUnknown, ICreateTypeInfo);
impl ICreateTypeInfo2 {
    pub unsafe fn DeleteFuncDesc(&self, index: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteFuncDesc)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn DeleteFuncDescByMemId(&self, memid: MEMBERID, invkind: INVOKEKIND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteFuncDescByMemId)(windows_core::Interface::as_raw(self), memid, invkind) }
    }
    pub unsafe fn DeleteVarDesc(&self, index: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteVarDesc)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn DeleteVarDescByMemId(&self, memid: MEMBERID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteVarDescByMemId)(windows_core::Interface::as_raw(self), memid) }
    }
    pub unsafe fn DeleteImplType(&self, index: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteImplType)(windows_core::Interface::as_raw(self), index) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCustData)(windows_core::Interface::as_raw(self), guid, core::mem::transmute(pvarval)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetFuncCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncCustData)(windows_core::Interface::as_raw(self), index, guid, core::mem::transmute(pvarval)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, guid, core::mem::transmute(pvarval)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetVarCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarCustData)(windows_core::Interface::as_raw(self), index, guid, core::mem::transmute(pvarval)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetImplTypeCustData)(windows_core::Interface::as_raw(self), index, guid, core::mem::transmute(pvarval)) }
    }
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpStringContext)(windows_core::Interface::as_raw(self), dwhelpstringcontext) }
    }
    pub unsafe fn SetFuncHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncHelpStringContext)(windows_core::Interface::as_raw(self), index, dwhelpstringcontext) }
    }
    pub unsafe fn SetVarHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarHelpStringContext)(windows_core::Interface::as_raw(self), index, dwhelpstringcontext) }
    }
    pub unsafe fn Invalidate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), szname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeInfo2_Vtbl {
    pub base__: ICreateTypeInfo_Vtbl,
    pub DeleteFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteFuncDescByMemId: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, INVOKEKIND) -> windows_core::HRESULT,
    pub DeleteVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteVarDescByMemId: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID) -> windows_core::HRESULT,
    pub DeleteImplType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetFuncCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetParamCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetVarCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetImplTypeCustData: usize,
    pub SetHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetFuncHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetVarHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub trait ICreateTypeInfo2_Impl: ICreateTypeInfo_Impl {
    fn DeleteFuncDesc(&self, index: u32) -> windows_core::Result<()>;
    fn DeleteFuncDescByMemId(&self, memid: MEMBERID, invkind: INVOKEKIND) -> windows_core::Result<()>;
    fn DeleteVarDesc(&self, index: u32) -> windows_core::Result<()>;
    fn DeleteVarDescByMemId(&self, memid: MEMBERID) -> windows_core::Result<()>;
    fn DeleteImplType(&self, index: u32) -> windows_core::Result<()>;
    fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetFuncCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetVarCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn SetFuncHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn SetVarHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn Invalidate(&self) -> windows_core::Result<()>;
    fn SetName(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl ICreateTypeInfo2_Vtbl {
    pub const fn new<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteFuncDesc<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteFuncDesc(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, invkind: INVOKEKIND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteFuncDescByMemId(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind)).into()
            }
        }
        unsafe extern "system" fn DeleteVarDesc<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteVarDesc(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteVarDescByMemId(this, core::mem::transmute_copy(&memid)).into()
            }
        }
        unsafe extern "system" fn DeleteImplType<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteImplType(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetCustData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetFuncCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetFuncCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetParamCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetVarCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetVarCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetImplTypeCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetImplTypeCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpstringcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetHelpStringContext(this, core::mem::transmute_copy(&dwhelpstringcontext)).into()
            }
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetFuncHelpStringContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpstringcontext)).into()
            }
        }
        unsafe extern "system" fn SetVarHelpStringContext<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetVarHelpStringContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpstringcontext)).into()
            }
        }
        unsafe extern "system" fn Invalidate<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::Invalidate(this).into()
            }
        }
        unsafe extern "system" fn SetName<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetName(this, core::mem::transmute(&szname)).into()
            }
        }
        Self {
            base__: ICreateTypeInfo_Vtbl::new::<Identity, OFFSET>(),
            DeleteFuncDesc: DeleteFuncDesc::<Identity, OFFSET>,
            DeleteFuncDescByMemId: DeleteFuncDescByMemId::<Identity, OFFSET>,
            DeleteVarDesc: DeleteVarDesc::<Identity, OFFSET>,
            DeleteVarDescByMemId: DeleteVarDescByMemId::<Identity, OFFSET>,
            DeleteImplType: DeleteImplType::<Identity, OFFSET>,
            SetCustData: SetCustData::<Identity, OFFSET>,
            SetFuncCustData: SetFuncCustData::<Identity, OFFSET>,
            SetParamCustData: SetParamCustData::<Identity, OFFSET>,
            SetVarCustData: SetVarCustData::<Identity, OFFSET>,
            SetImplTypeCustData: SetImplTypeCustData::<Identity, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, OFFSET>,
            SetFuncHelpStringContext: SetFuncHelpStringContext::<Identity, OFFSET>,
            SetVarHelpStringContext: SetVarHelpStringContext::<Identity, OFFSET>,
            Invalidate: Invalidate::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeInfo2 as windows_core::Interface>::IID || iid == &<ICreateTypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICreateTypeInfo2 {}
windows_core::imp::define_interface!(ICreateTypeLib, ICreateTypeLib_Vtbl, 0x00020406_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICreateTypeLib, windows_core::IUnknown);
impl ICreateTypeLib {
    pub unsafe fn CreateTypeInfo<P0>(&self, szname: P0, tkind: TYPEKIND) -> windows_core::Result<ICreateTypeInfo>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTypeInfo)(windows_core::Interface::as_raw(self), szname.param().abi(), tkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), szname.param().abi()) }
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), wmajorvernum, wminorvernum) }
    }
    pub unsafe fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid) }
    }
    pub unsafe fn SetDocString<P0>(&self, szdoc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocString)(windows_core::Interface::as_raw(self), szdoc.param().abi()) }
    }
    pub unsafe fn SetHelpFileName<P0>(&self, szhelpfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHelpFileName)(windows_core::Interface::as_raw(self), szhelpfilename.param().abi()) }
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetLcid(&self, lcid: super::winnt::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLcid)(windows_core::Interface::as_raw(self), lcid) }
    }
    pub unsafe fn SetLibFlags(&self, ulibflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLibFlags)(windows_core::Interface::as_raw(self), ulibflags) }
    }
    pub unsafe fn SaveAllChanges(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveAllChanges)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeLib_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, TYPEKIND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub SetLcid: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetLcid: usize,
    pub SetLibFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveAllChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait ICreateTypeLib_Impl: windows_core::IUnknownImpl {
    fn CreateTypeInfo(&self, szname: &windows_core::PCWSTR, tkind: TYPEKIND) -> windows_core::Result<ICreateTypeInfo>;
    fn SetName(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::Result<()>;
    fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetDocString(&self, szdoc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpFileName(&self, szhelpfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetLcid(&self, lcid: super::winnt::LCID) -> windows_core::Result<()>;
    fn SetLibFlags(&self, ulibflags: u32) -> windows_core::Result<()>;
    fn SaveAllChanges(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl ICreateTypeLib_Vtbl {
    pub const fn new<Identity: ICreateTypeLib_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTypeInfo<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, tkind: TYPEKIND, ppctinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICreateTypeLib_Impl::CreateTypeInfo(this, core::mem::transmute(&szname), core::mem::transmute_copy(&tkind)) {
                    Ok(ok__) => {
                        ppctinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetName(this, core::mem::transmute(&szname)).into()
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetVersion(this, core::mem::transmute_copy(&wmajorvernum), core::mem::transmute_copy(&wminorvernum)).into()
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetGuid(this, core::mem::transmute_copy(&guid)).into()
            }
        }
        unsafe extern "system" fn SetDocString<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdoc: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetDocString(this, core::mem::transmute(&szdoc)).into()
            }
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhelpfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetHelpFileName(this, core::mem::transmute(&szhelpfilename)).into()
            }
        }
        unsafe extern "system" fn SetHelpContext<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetLcid<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetLcid(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn SetLibFlags<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulibflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetLibFlags(this, core::mem::transmute_copy(&ulibflags)).into()
            }
        }
        unsafe extern "system" fn SaveAllChanges<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SaveAllChanges(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTypeInfo: CreateTypeInfo::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            SetDocString: SetDocString::<Identity, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, OFFSET>,
            SetLcid: SetLcid::<Identity, OFFSET>,
            SetLibFlags: SetLibFlags::<Identity, OFFSET>,
            SaveAllChanges: SaveAllChanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeLib as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ICreateTypeLib {}
windows_core::imp::define_interface!(ICreateTypeLib2, ICreateTypeLib2_Vtbl, 0x0002040f_0000_0000_c000_000000000046);
impl core::ops::Deref for ICreateTypeLib2 {
    type Target = ICreateTypeLib;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateTypeLib2, windows_core::IUnknown, ICreateTypeLib);
impl ICreateTypeLib2 {
    pub unsafe fn DeleteTypeInfo<P0>(&self, szname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteTypeInfo)(windows_core::Interface::as_raw(self), szname.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCustData)(windows_core::Interface::as_raw(self), guid, core::mem::transmute(pvarval)) }
    }
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpStringContext)(windows_core::Interface::as_raw(self), dwhelpstringcontext) }
    }
    pub unsafe fn SetHelpStringDll<P0>(&self, szfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHelpStringDll)(windows_core::Interface::as_raw(self), szfilename.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeLib2_Vtbl {
    pub base__: ICreateTypeLib_Vtbl,
    pub DeleteTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetCustData: usize,
    pub SetHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetHelpStringDll: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICreateTypeLib2_Impl: ICreateTypeLib_Impl {
    fn DeleteTypeInfo(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn SetHelpStringDll(&self, szfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICreateTypeLib2_Vtbl {
    pub const fn new<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteTypeInfo<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::DeleteTypeInfo(this, core::mem::transmute(&szname)).into()
            }
        }
        unsafe extern "system" fn SetCustData<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::SetCustData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpstringcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::SetHelpStringContext(this, core::mem::transmute_copy(&dwhelpstringcontext)).into()
            }
        }
        unsafe extern "system" fn SetHelpStringDll<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::SetHelpStringDll(this, core::mem::transmute(&szfilename)).into()
            }
        }
        Self {
            base__: ICreateTypeLib_Vtbl::new::<Identity, OFFSET>(),
            DeleteTypeInfo: DeleteTypeInfo::<Identity, OFFSET>,
            SetCustData: SetCustData::<Identity, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, OFFSET>,
            SetHelpStringDll: SetHelpStringDll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeLib2 as windows_core::Interface>::IID || iid == &<ICreateTypeLib as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICreateTypeLib2 {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
pub const IDLFLAG_FIN: u32 = 1;
pub const IDLFLAG_FLCID: u32 = 4;
pub const IDLFLAG_FOUT: u32 = 2;
pub const IDLFLAG_FRETVAL: u32 = 8;
pub const IDLFLAG_NONE: u32 = 0;
windows_core::imp::define_interface!(IDispatch, IDispatch_Vtbl, 0x00020400_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: super::winnt::LCID) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: super::winnt::LCID) -> windows_core::Result<DISPID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Invoke(&self, dispidmember: DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: Option<*mut VARIANT>, pexcepinfo: Option<*mut EXCEPINFO>, puargerr: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult.unwrap_or(core::mem::zeroed()) as _, pexcepinfo.unwrap_or(core::mem::zeroed()) as _, puargerr.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetTypeInfo: usize,
    #[cfg(feature = "winnt")]
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::PCWSTR, u32, super::winnt::LCID, *mut DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetIDsOfNames: usize,
    #[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, DISPID, *const windows_core::GUID, super::winnt::LCID, u16, *const DISPPARAMS, *mut VARIANT, *mut EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Invoke: usize,
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispatch_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn GetTypeInfo(&self, itinfo: u32, lcid: super::winnt::LCID) -> windows_core::Result<ITypeInfo>;
    fn GetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: super::winnt::LCID) -> windows_core::Result<DISPID>;
    fn Invoke(&self, dispidmember: DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispatch_Vtbl {
    pub const fn new<Identity: IDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctinfo: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatch_Impl::GetTypeInfoCount(this) {
                    Ok(ok__) => {
                        pctinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itinfo: u32, lcid: super::winnt::LCID, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatch_Impl::GetTypeInfo(this, core::mem::transmute_copy(&itinfo), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: super::winnt::LCID, rgdispid: *mut DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatch_Impl::GetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        rgdispid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Invoke<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatch_Impl::Invoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(IEnumVARIANT, IEnumVARIANT_Vtbl, 0x00020404_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumVARIANT, windows_core::IUnknown);
impl IEnumVARIANT {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut VARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgvar), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumVARIANT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut VARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub trait IEnumVARIANT_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgvar: *mut VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumVARIANT>;
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl IEnumVARIANT_Vtbl {
    pub const fn new<Identity: IEnumVARIANT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut VARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumVARIANT_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumVARIANT_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumVARIANT_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumVARIANT_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumVARIANT as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEnumVARIANT {}
windows_core::imp::define_interface!(IErrorInfo, IErrorInfo_Vtbl, 0x1cf2b120_547d_101b_8e65_08002b2bd119);
windows_core::imp::interface_hierarchy!(IErrorInfo, windows_core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHelpFile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHelpContext(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpContext(&self) -> windows_core::Result<u32>;
}
impl IErrorInfo_Vtbl {
    pub const fn new<Identity: IErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGUID<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetGUID(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetSource(this) {
                    Ok(ok__) => {
                        pbstrsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpFile<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetHelpFile(this) {
                    Ok(ok__) => {
                        pbstrhelpfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpContext<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhelpcontext: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetHelpContext(this) {
                    Ok(ok__) => {
                        pdwhelpcontext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetHelpFile: GetHelpFile::<Identity, OFFSET>,
            GetHelpContext: GetHelpContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IErrorInfo {}
windows_core::imp::define_interface!(IErrorLog, IErrorLog_Vtbl, 0x3127ca40_446e_11ce_8135_00aa004bb851);
windows_core::imp::interface_hierarchy!(IErrorLog, windows_core::IUnknown);
impl IErrorLog {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn AddError<P0>(&self, pszpropname: P0, pexcepinfo: *const EXCEPINFO) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddError)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), core::mem::transmute(pexcepinfo)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub AddError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    AddError: usize,
}
#[cfg(feature = "wtypesbase")]
pub trait IErrorLog_Impl: windows_core::IUnknownImpl {
    fn AddError(&self, pszpropname: &windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypesbase")]
impl IErrorLog_Vtbl {
    pub const fn new<Identity: IErrorLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddError<Identity: IErrorLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorLog_Impl::AddError(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddError: AddError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorLog as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypesbase")]
impl windows_core::RuntimeName for IErrorLog {}
pub const IMPLTYPEFLAG_FDEFAULT: u32 = 1;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: u32 = 8;
pub const IMPLTYPEFLAG_FRESTRICTED: u32 = 4;
pub const IMPLTYPEFLAG_FSOURCE: u32 = 2;
pub type INVOKEKIND = i32;
pub const INVOKE_FUNC: INVOKEKIND = 1;
pub const INVOKE_PROPERTYGET: INVOKEKIND = 2;
pub const INVOKE_PROPERTYPUT: INVOKEKIND = 4;
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = 8;
windows_core::imp::define_interface!(IPropertyBag, IPropertyBag_Vtbl, 0x55272a00_42cb_11ce_8135_00aa004bb851);
windows_core::imp::interface_hierarchy!(IPropertyBag, windows_core::IUnknown);
impl IPropertyBag {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Read<P0, P2>(&self, pszpropname: P0, pvar: *mut VARIANT, perrorlog: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), core::mem::transmute(pvar), perrorlog.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const VARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), core::mem::transmute(pvar)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Read: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Write: usize,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyBag_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pszpropname: &windows_core::PCWSTR, pvar: *mut VARIANT, perrorlog: windows_core::Ref<IErrorLog>) -> windows_core::Result<()>;
    fn Write(&self, pszpropname: &windows_core::PCWSTR, pvar: *const VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyBag_Vtbl {
    pub const fn new<Identity: IPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pvar: *mut VARIANT, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag_Impl::Read(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&perrorlog)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pvar: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag_Impl::Write(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyBag {}
windows_core::imp::define_interface!(IRecordInfo, IRecordInfo_Vtbl, 0x0000002f_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRecordInfo, windows_core::IUnknown);
impl IRecordInfo {
    pub unsafe fn RecordInit(&self, pvnew: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordInit)(windows_core::Interface::as_raw(self), pvnew as _) }
    }
    pub unsafe fn RecordClear(&self, pvexisting: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordClear)(windows_core::Interface::as_raw(self), pvexisting) }
    }
    pub unsafe fn RecordCopy(&self, pvexisting: *const core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordCopy)(windows_core::Interface::as_raw(self), pvexisting, pvnew as _) }
    }
    pub unsafe fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeInfo(&self) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetField<P1>(&self, pvdata: *const core::ffi::c_void, szfieldname: P1) -> windows_core::Result<VARIANT>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetField)(windows_core::Interface::as_raw(self), pvdata, szfieldname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFieldNoCopy<P1>(&self, pvdata: *const core::ffi::c_void, szfieldname: P1, pvarfield: *mut VARIANT, ppvdatacarray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFieldNoCopy)(windows_core::Interface::as_raw(self), pvdata, szfieldname.param().abi(), core::mem::transmute(pvarfield), ppvdatacarray as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutField<P2>(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: P2, pvarfield: *const VARIANT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutField)(windows_core::Interface::as_raw(self), wflags, pvdata as _, szfieldname.param().abi(), core::mem::transmute(pvarfield)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutFieldNoCopy<P2>(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: P2, pvarfield: *const VARIANT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutFieldNoCopy)(windows_core::Interface::as_raw(self), wflags, pvdata as _, szfieldname.param().abi(), core::mem::transmute(pvarfield)) }
    }
    pub unsafe fn GetFieldNames(&self, pcnames: *mut u32, rgbstrnames: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFieldNames)(windows_core::Interface::as_raw(self), pcnames as _, core::mem::transmute(rgbstrnames)) }
    }
    pub unsafe fn IsMatchingType<P0>(&self, precordinfo: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsMatchingType)(windows_core::Interface::as_raw(self), precordinfo.param().abi()) }
    }
    pub unsafe fn RecordCreate(&self) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).RecordCreate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RecordCreateCopy(&self, pvsource: *const core::ffi::c_void, ppvdest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordCreateCopy)(windows_core::Interface::as_raw(self), pvsource, ppvdest as _) }
    }
    pub unsafe fn RecordDestroy(&self, pvrecord: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordDestroy)(windows_core::Interface::as_raw(self), pvrecord) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecordInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RecordInit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordClear: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetField: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetField: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetFieldNoCopy: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub PutField: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    PutField: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub PutFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    PutFieldNoCopy: usize,
    pub GetFieldNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsMatchingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::BOOL,
    pub RecordCreate: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub RecordCreateCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub trait IRecordInfo_Impl: windows_core::IUnknownImpl {
    fn RecordInit(&self, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordClear(&self, pvexisting: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordCopy(&self, pvexisting: *const core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSize(&self) -> windows_core::Result<u32>;
    fn GetTypeInfo(&self) -> windows_core::Result<ITypeInfo>;
    fn GetField(&self, pvdata: *const core::ffi::c_void, szfieldname: &windows_core::PCWSTR) -> windows_core::Result<VARIANT>;
    fn GetFieldNoCopy(&self, pvdata: *const core::ffi::c_void, szfieldname: &windows_core::PCWSTR, pvarfield: *mut VARIANT, ppvdatacarray: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PutField(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: &windows_core::PCWSTR, pvarfield: *const VARIANT) -> windows_core::Result<()>;
    fn PutFieldNoCopy(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: &windows_core::PCWSTR, pvarfield: *const VARIANT) -> windows_core::Result<()>;
    fn GetFieldNames(&self, pcnames: *mut u32, rgbstrnames: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsMatchingType(&self, precordinfo: windows_core::Ref<IRecordInfo>) -> windows_core::BOOL;
    fn RecordCreate(&self) -> *mut core::ffi::c_void;
    fn RecordCreateCopy(&self, pvsource: *const core::ffi::c_void, ppvdest: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordDestroy(&self, pvrecord: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl IRecordInfo_Vtbl {
    pub const fn new<Identity: IRecordInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RecordInit<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordInit(this, core::mem::transmute_copy(&pvnew)).into()
            }
        }
        unsafe extern "system" fn RecordClear<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvexisting: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordClear(this, core::mem::transmute_copy(&pvexisting)).into()
            }
        }
        unsafe extern "system" fn RecordCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvexisting: *const core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCopy(this, core::mem::transmute_copy(&pvexisting), core::mem::transmute_copy(&pvnew)).into()
            }
        }
        unsafe extern "system" fn GetGuid<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetGuid(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetSize(this) {
                    Ok(ok__) => {
                        pcbsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetTypeInfo(this) {
                    Ok(ok__) => {
                        pptypeinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetField<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *const core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetField(this, core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname)) {
                    Ok(ok__) => {
                        pvarfield.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *const core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *mut VARIANT, ppvdatacarray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::GetFieldNoCopy(this, core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname), core::mem::transmute_copy(&pvarfield), core::mem::transmute_copy(&ppvdatacarray)).into()
            }
        }
        unsafe extern "system" fn PutField<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::PutField(this, core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname), core::mem::transmute_copy(&pvarfield)).into()
            }
        }
        unsafe extern "system" fn PutFieldNoCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::PutFieldNoCopy(this, core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname), core::mem::transmute_copy(&pvarfield)).into()
            }
        }
        unsafe extern "system" fn GetFieldNames<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::GetFieldNames(this, core::mem::transmute_copy(&pcnames), core::mem::transmute_copy(&rgbstrnames)).into()
            }
        }
        unsafe extern "system" fn IsMatchingType<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precordinfo: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::IsMatchingType(this, core::mem::transmute_copy(&precordinfo))
            }
        }
        unsafe extern "system" fn RecordCreate<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCreate(this)
            }
        }
        unsafe extern "system" fn RecordCreateCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvsource: *const core::ffi::c_void, ppvdest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCreateCopy(this, core::mem::transmute_copy(&pvsource), core::mem::transmute_copy(&ppvdest)).into()
            }
        }
        unsafe extern "system" fn RecordDestroy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvrecord: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordDestroy(this, core::mem::transmute_copy(&pvrecord)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RecordInit: RecordInit::<Identity, OFFSET>,
            RecordClear: RecordClear::<Identity, OFFSET>,
            RecordCopy: RecordCopy::<Identity, OFFSET>,
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetField: GetField::<Identity, OFFSET>,
            GetFieldNoCopy: GetFieldNoCopy::<Identity, OFFSET>,
            PutField: PutField::<Identity, OFFSET>,
            PutFieldNoCopy: PutFieldNoCopy::<Identity, OFFSET>,
            GetFieldNames: GetFieldNames::<Identity, OFFSET>,
            IsMatchingType: IsMatchingType::<Identity, OFFSET>,
            RecordCreate: RecordCreate::<Identity, OFFSET>,
            RecordCreateCopy: RecordCreateCopy::<Identity, OFFSET>,
            RecordDestroy: RecordDestroy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRecordInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRecordInfo {}
windows_core::imp::define_interface!(ISupportErrorInfo, ISupportErrorInfo_Vtbl, 0xdf0b3d60_548f_101b_8e65_08002b2bd119);
windows_core::imp::interface_hierarchy!(ISupportErrorInfo, windows_core::IUnknown);
impl ISupportErrorInfo {
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InterfaceSupportsErrorInfo)(windows_core::Interface::as_raw(self), riid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InterfaceSupportsErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ISupportErrorInfo_Impl: windows_core::IUnknownImpl {
    fn InterfaceSupportsErrorInfo(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ISupportErrorInfo_Vtbl {
    pub const fn new<Identity: ISupportErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Identity: ISupportErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISupportErrorInfo_Impl::InterfaceSupportsErrorInfo(this, core::mem::transmute_copy(&riid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InterfaceSupportsErrorInfo: InterfaceSupportsErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportErrorInfo {}
windows_core::imp::define_interface!(ITypeChangeEvents, ITypeChangeEvents_Vtbl, 0x00020410_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeChangeEvents, windows_core::IUnknown);
impl ITypeChangeEvents {
    pub unsafe fn RequestTypeChange<P1, P2>(&self, changekind: CHANGEKIND, ptinfobefore: P1, pstrname: P2) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<ITypeInfo>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestTypeChange)(windows_core::Interface::as_raw(self), changekind, ptinfobefore.param().abi(), pstrname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AfterTypeChange<P1, P2>(&self, changekind: CHANGEKIND, ptinfoafter: P1, pstrname: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITypeInfo>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AfterTypeChange)(windows_core::Interface::as_raw(self), changekind, ptinfoafter.param().abi(), pstrname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeChangeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestTypeChange: unsafe extern "system" fn(*mut core::ffi::c_void, CHANGEKIND, *mut core::ffi::c_void, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub AfterTypeChange: unsafe extern "system" fn(*mut core::ffi::c_void, CHANGEKIND, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ITypeChangeEvents_Impl: windows_core::IUnknownImpl {
    fn RequestTypeChange(&self, changekind: CHANGEKIND, ptinfobefore: windows_core::Ref<ITypeInfo>, pstrname: &windows_core::PCWSTR) -> windows_core::Result<i32>;
    fn AfterTypeChange(&self, changekind: CHANGEKIND, ptinfoafter: windows_core::Ref<ITypeInfo>, pstrname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ITypeChangeEvents_Vtbl {
    pub const fn new<Identity: ITypeChangeEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestTypeChange<Identity: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR, pfcancel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeChangeEvents_Impl::RequestTypeChange(this, core::mem::transmute_copy(&changekind), core::mem::transmute_copy(&ptinfobefore), core::mem::transmute(&pstrname)) {
                    Ok(ok__) => {
                        pfcancel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AfterTypeChange<Identity: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeChangeEvents_Impl::AfterTypeChange(this, core::mem::transmute_copy(&changekind), core::mem::transmute_copy(&ptinfoafter), core::mem::transmute(&pstrname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestTypeChange: RequestTypeChange::<Identity, OFFSET>,
            AfterTypeChange: AfterTypeChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeChangeEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeChangeEvents {}
windows_core::imp::define_interface!(ITypeComp, ITypeComp_Vtbl, 0x00020403_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeComp, windows_core::IUnknown);
impl ITypeComp {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Bind<P0>(&self, szname: P0, lhashval: u32, wflags: u16, pptinfo: *mut Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), szname.param().abi(), lhashval, wflags, core::mem::transmute(pptinfo), pdesckind as _, core::mem::transmute(pbindptr)) }
    }
    pub unsafe fn BindType<P0>(&self, szname: P0, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, pptcomp: *mut Option<Self>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindType)(windows_core::Interface::as_raw(self), szname.param().abi(), lhashval, core::mem::transmute(pptinfo), core::mem::transmute(pptcomp)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeComp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u16, *mut *mut core::ffi::c_void, *mut DESCKIND, *mut BINDPTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Bind: usize,
    pub BindType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeComp_Impl: windows_core::IUnknownImpl {
    fn Bind(&self, szname: &windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: windows_core::OutRef<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::Result<()>;
    fn BindType(&self, szname: &windows_core::PCWSTR, lhashval: u32, pptinfo: windows_core::OutRef<ITypeInfo>, pptcomp: windows_core::OutRef<ITypeComp>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl ITypeComp_Vtbl {
    pub const fn new<Identity: ITypeComp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Bind<Identity: ITypeComp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut *mut core::ffi::c_void, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeComp_Impl::Bind(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pdesckind), core::mem::transmute_copy(&pbindptr)).into()
            }
        }
        unsafe extern "system" fn BindType<Identity: ITypeComp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, lhashval: u32, pptinfo: *mut *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeComp_Impl::BindType(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pptcomp)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, OFFSET>, BindType: BindType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeComp as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITypeComp {}
windows_core::imp::define_interface!(ITypeFactory, ITypeFactory_Vtbl, 0x0000002e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeFactory, windows_core::IUnknown);
impl ITypeFactory {
    pub unsafe fn CreateFromTypeInfo<P0, T>(&self, ptypeinfo: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ITypeInfo>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateFromTypeInfo)(windows_core::Interface::as_raw(self), ptypeinfo.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateFromTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITypeFactory_Impl: windows_core::IUnknownImpl {
    fn CreateFromTypeInfo(&self, ptypeinfo: windows_core::Ref<ITypeInfo>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ITypeFactory_Vtbl {
    pub const fn new<Identity: ITypeFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromTypeInfo<Identity: ITypeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeinfo: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeFactory_Impl::CreateFromTypeInfo(this, core::mem::transmute_copy(&ptypeinfo), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateFromTypeInfo: CreateFromTypeInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeFactory {}
windows_core::imp::define_interface!(ITypeInfo, ITypeInfo_Vtbl, 0x00020401_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeInfo, windows_core::IUnknown);
impl ITypeInfo {
    #[cfg(all(feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> windows_core::Result<*mut FUNCDESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> windows_core::Result<*mut VARDESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNames(&self, memid: MEMBERID, rgbstrnames: *mut windows_core::BSTR, cmaxnames: u32, pcnames: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), memid, core::mem::transmute(rgbstrnames), cmaxnames, pcnames as _) }
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> windows_core::Result<HREFTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRefTypeOfImplType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImplTypeFlags)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const windows_core::PCWSTR, cnames: u32) -> windows_core::Result<MEMBERID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), rgsznames, cnames, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Invoke(&self, pvinstance: *const core::ffi::c_void, memid: MEMBERID, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pvinstance, memid, wflags, pdispparams as _, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo), puargerr as _) }
    }
    pub unsafe fn GetDocumentation(&self, memid: MEMBERID, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), memid, core::mem::transmute(pbstrname), core::mem::transmute(pbstrdocstring), pdwhelpcontext as _, core::mem::transmute(pbstrhelpfile)) }
    }
    pub unsafe fn GetDllEntry(&self, memid: MEMBERID, invkind: INVOKEKIND, pbstrdllname: *mut windows_core::BSTR, pbstrname: *mut windows_core::BSTR, pwordinal: *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDllEntry)(windows_core::Interface::as_raw(self), memid, invkind, core::mem::transmute(pbstrdllname), core::mem::transmute(pbstrname), pwordinal as _) }
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: HREFTYPE) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRefTypeInfo)(windows_core::Interface::as_raw(self), hreftype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddressOfMember(&self, memid: MEMBERID, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddressOfMember)(windows_core::Interface::as_raw(self), memid, invkind, ppv as _) }
    }
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetMops(&self, memid: MEMBERID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMops)(windows_core::Interface::as_raw(self), memid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut Option<ITypeLib>, pindex: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContainingTypeLib)(windows_core::Interface::as_raw(self), core::mem::transmute(pptlib), pindex as _) }
    }
    #[cfg(all(feature = "winnt", feature = "wtypes"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseTypeAttr)(windows_core::Interface::as_raw(self), ptypeattr);
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseFuncDesc)(windows_core::Interface::as_raw(self), pfuncdesc);
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseVarDesc)(windows_core::Interface::as_raw(self), pvardesc);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "winnt", feature = "wtypes"))]
    pub GetTypeAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TYPEATTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "winnt", feature = "wtypes")))]
    GetTypeAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut FUNCDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetFuncDesc: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut VARDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetVarDesc: usize,
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, *mut *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRefTypeOfImplType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut HREFTYPE) -> windows_core::HRESULT,
    pub GetImplTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32) -> windows_core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *mut MEMBERID) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, MEMBERID, u16, *mut DISPPARAMS, *mut VARIANT, *mut EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Invoke: usize,
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDllEntry: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, INVOKEKIND, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetRefTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, HREFTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddressOfMember: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, INVOKEKIND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMops: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContainingTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "winnt", feature = "wtypes"))]
    pub ReleaseTypeAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TYPEATTR),
    #[cfg(not(all(feature = "winnt", feature = "wtypes")))]
    ReleaseTypeAttr: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ReleaseFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const FUNCDESC),
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ReleaseFuncDesc: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ReleaseVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const VARDESC),
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ReleaseVarDesc: usize,
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeInfo_Impl: windows_core::IUnknownImpl {
    fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetFuncDesc(&self, index: u32) -> windows_core::Result<*mut FUNCDESC>;
    fn GetVarDesc(&self, index: u32) -> windows_core::Result<*mut VARDESC>;
    fn GetNames(&self, memid: MEMBERID, rgbstrnames: *mut windows_core::BSTR, cmaxnames: u32, pcnames: *mut u32) -> windows_core::Result<()>;
    fn GetRefTypeOfImplType(&self, index: u32) -> windows_core::Result<HREFTYPE>;
    fn GetImplTypeFlags(&self, index: u32) -> windows_core::Result<i32>;
    fn GetIDsOfNames(&self, rgsznames: *const windows_core::PCWSTR, cnames: u32) -> windows_core::Result<MEMBERID>;
    fn Invoke(&self, pvinstance: *const core::ffi::c_void, memid: MEMBERID, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
    fn GetDocumentation(&self, memid: MEMBERID, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDllEntry(&self, memid: MEMBERID, invkind: INVOKEKIND, pbstrdllname: *mut windows_core::BSTR, pbstrname: *mut windows_core::BSTR, pwordinal: *mut u16) -> windows_core::Result<()>;
    fn GetRefTypeInfo(&self, hreftype: HREFTYPE) -> windows_core::Result<ITypeInfo>;
    fn AddressOfMember(&self, memid: MEMBERID, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateInstance(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetMops(&self, memid: MEMBERID) -> windows_core::Result<windows_core::BSTR>;
    fn GetContainingTypeLib(&self, pptlib: windows_core::OutRef<ITypeLib>, pindex: *mut u32) -> windows_core::Result<()>;
    fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR);
    fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC);
    fn ReleaseVarDesc(&self, pvardesc: *const VARDESC);
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITypeInfo_Vtbl {
    pub const fn new<Identity: ITypeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeAttr<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetTypeAttr(this) {
                    Ok(ok__) => {
                        pptypeattr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetTypeComp(this) {
                    Ok(ok__) => {
                        pptcomp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFuncDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetFuncDesc(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppfuncdesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVarDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetVarDesc(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppvardesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNames<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, rgbstrnames: *mut *mut core::ffi::c_void, cmaxnames: u32, pcnames: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetNames(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&rgbstrnames), core::mem::transmute_copy(&cmaxnames), core::mem::transmute_copy(&pcnames)).into()
            }
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, preftype: *mut HREFTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetRefTypeOfImplType(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        preftype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImplTypeFlags<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetImplTypeFlags(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pimpltypeflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgsznames: *const windows_core::PCWSTR, cnames: u32, pmemid: *mut MEMBERID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetIDsOfNames(this, core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames)) {
                    Ok(ok__) => {
                        pmemid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Invoke<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvinstance: *const core::ffi::c_void, memid: MEMBERID, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::Invoke(this, core::mem::transmute_copy(&pvinstance), core::mem::transmute_copy(&memid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        unsafe extern "system" fn GetDocumentation<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, pbstrname: *mut *mut core::ffi::c_void, pbstrdocstring: *mut *mut core::ffi::c_void, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetDocumentation(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
            }
        }
        unsafe extern "system" fn GetDllEntry<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, invkind: INVOKEKIND, pbstrdllname: *mut *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void, pwordinal: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetDllEntry(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind), core::mem::transmute_copy(&pbstrdllname), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pwordinal)).into()
            }
        }
        unsafe extern "system" fn GetRefTypeInfo<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreftype: HREFTYPE, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetRefTypeInfo(this, core::mem::transmute_copy(&hreftype)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddressOfMember<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::AddressOfMember(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::CreateInstance(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn GetMops<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, pbstrmops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetMops(this, core::mem::transmute_copy(&memid)) {
                    Ok(ok__) => {
                        pbstrmops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContainingTypeLib<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlib: *mut *mut core::ffi::c_void, pindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetContainingTypeLib(this, core::mem::transmute_copy(&pptlib), core::mem::transmute_copy(&pindex)).into()
            }
        }
        unsafe extern "system" fn ReleaseTypeAttr<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeattr: *const TYPEATTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::ReleaseTypeAttr(this, core::mem::transmute_copy(&ptypeattr));
            }
        }
        unsafe extern "system" fn ReleaseFuncDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncdesc: *const FUNCDESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::ReleaseFuncDesc(this, core::mem::transmute_copy(&pfuncdesc));
            }
        }
        unsafe extern "system" fn ReleaseVarDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardesc: *const VARDESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::ReleaseVarDesc(this, core::mem::transmute_copy(&pvardesc));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeAttr: GetTypeAttr::<Identity, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, OFFSET>,
            GetFuncDesc: GetFuncDesc::<Identity, OFFSET>,
            GetVarDesc: GetVarDesc::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            GetRefTypeOfImplType: GetRefTypeOfImplType::<Identity, OFFSET>,
            GetImplTypeFlags: GetImplTypeFlags::<Identity, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, OFFSET>,
            GetDllEntry: GetDllEntry::<Identity, OFFSET>,
            GetRefTypeInfo: GetRefTypeInfo::<Identity, OFFSET>,
            AddressOfMember: AddressOfMember::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            GetMops: GetMops::<Identity, OFFSET>,
            GetContainingTypeLib: GetContainingTypeLib::<Identity, OFFSET>,
            ReleaseTypeAttr: ReleaseTypeAttr::<Identity, OFFSET>,
            ReleaseFuncDesc: ReleaseFuncDesc::<Identity, OFFSET>,
            ReleaseVarDesc: ReleaseVarDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITypeInfo {}
windows_core::imp::define_interface!(ITypeInfo2, ITypeInfo2_Vtbl, 0x00020412_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeInfo2 {
    type Target = ITypeInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeInfo2, windows_core::IUnknown, ITypeInfo);
impl ITypeInfo2 {
    pub unsafe fn GetTypeKind(&self) -> windows_core::Result<TYPEKIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeKind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: MEMBERID, invkind: INVOKEKIND) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncIndexOfMemId)(windows_core::Interface::as_raw(self), memid, invkind, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVarIndexOfMemId(&self, memid: MEMBERID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarIndexOfMemId)(windows_core::Interface::as_raw(self), memid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImplTypeCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetDocumentation2(&self, memid: MEMBERID, lcid: super::winnt::LCID, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), memid, lcid, core::mem::transmute(pbstrhelpstring), pdwhelpstringcontext as _, core::mem::transmute(pbstrhelpstringdll)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllCustData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllFuncCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllVarCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllImplTypeCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2_Vtbl {
    pub base__: ITypeInfo_Vtbl,
    pub GetTypeKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TYPEKIND) -> windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFuncIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, INVOKEKIND, *mut u32) -> windows_core::HRESULT,
    pub GetVarIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetFuncCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetParamCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetVarCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetImplTypeCustData: usize,
    #[cfg(feature = "winnt")]
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, MEMBERID, super::winnt::LCID, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetDocumentation2: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetAllCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetAllFuncCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetAllParamCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetAllVarCustData: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetAllImplTypeCustData: usize,
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeInfo2_Impl: ITypeInfo_Impl {
    fn GetTypeKind(&self) -> windows_core::Result<TYPEKIND>;
    fn GetTypeFlags(&self) -> windows_core::Result<u32>;
    fn GetFuncIndexOfMemId(&self, memid: MEMBERID, invkind: INVOKEKIND) -> windows_core::Result<u32>;
    fn GetVarIndexOfMemId(&self, memid: MEMBERID) -> windows_core::Result<u32>;
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetFuncCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetVarCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetDocumentation2(&self, memid: MEMBERID, lcid: super::winnt::LCID, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
    fn GetAllFuncCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllVarCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllImplTypeCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITypeInfo2_Vtbl {
    pub const fn new<Identity: ITypeInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeKind<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypekind: *mut TYPEKIND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetTypeKind(this) {
                    Ok(ok__) => {
                        ptypekind.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeFlags<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetTypeFlags(this) {
                    Ok(ok__) => {
                        ptypeflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFuncIndexOfMemId<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, invkind: INVOKEKIND, pfuncindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetFuncIndexOfMemId(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind)) {
                    Ok(ok__) => {
                        pfuncindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVarIndexOfMemId<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, pvarindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetVarIndexOfMemId(this, core::mem::transmute_copy(&memid)) {
                    Ok(ok__) => {
                        pvarindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetCustData(this, core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFuncCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetFuncCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParamCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVarCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetVarCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImplTypeCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetImplTypeCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: MEMBERID, lcid: super::winnt::LCID, pbstrhelpstring: *mut *mut core::ffi::c_void, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo2_Impl::GetDocumentation2(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpstring), core::mem::transmute_copy(&pdwhelpstringcontext), core::mem::transmute_copy(&pbstrhelpstringdll)).into()
            }
        }
        unsafe extern "system" fn GetAllCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllCustData(this) {
                    Ok(ok__) => {
                        pcustdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllFuncCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllFuncCustData(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcustdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllParamCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam)) {
                    Ok(ok__) => {
                        pcustdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllVarCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllVarCustData(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcustdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllImplTypeCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllImplTypeCustData(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcustdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITypeInfo_Vtbl::new::<Identity, OFFSET>(),
            GetTypeKind: GetTypeKind::<Identity, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, OFFSET>,
            GetFuncIndexOfMemId: GetFuncIndexOfMemId::<Identity, OFFSET>,
            GetVarIndexOfMemId: GetVarIndexOfMemId::<Identity, OFFSET>,
            GetCustData: GetCustData::<Identity, OFFSET>,
            GetFuncCustData: GetFuncCustData::<Identity, OFFSET>,
            GetParamCustData: GetParamCustData::<Identity, OFFSET>,
            GetVarCustData: GetVarCustData::<Identity, OFFSET>,
            GetImplTypeCustData: GetImplTypeCustData::<Identity, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, OFFSET>,
            GetAllFuncCustData: GetAllFuncCustData::<Identity, OFFSET>,
            GetAllParamCustData: GetAllParamCustData::<Identity, OFFSET>,
            GetAllVarCustData: GetAllVarCustData::<Identity, OFFSET>,
            GetAllImplTypeCustData: GetAllImplTypeCustData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeInfo2 as windows_core::Interface>::IID || iid == &<ITypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITypeInfo2 {}
windows_core::imp::define_interface!(ITypeLib, ITypeLib_Vtbl, 0x00020402_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeLib, windows_core::IUnknown);
impl ITypeLib {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> windows_core::Result<TYPEKIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoOfGuid)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLibAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), index, core::mem::transmute(pbstrname), core::mem::transmute(pbstrdocstring), pdwhelpcontext as _, core::mem::transmute(pbstrhelpfile)) }
    }
    pub unsafe fn IsName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsName)(windows_core::Interface::as_raw(self), core::mem::transmute(sznamebuf), lhashval, pfname as _) }
    }
    pub unsafe fn FindName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, rgmemid: *mut MEMBERID, pcfound: *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindName)(windows_core::Interface::as_raw(self), core::mem::transmute(sznamebuf), lhashval, core::mem::transmute(pptinfo), rgmemid as _, pcfound as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseTLibAttr)(windows_core::Interface::as_raw(self), ptlibattr);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypeInfoType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TYPEKIND) -> windows_core::HRESULT,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TLIBATTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLibAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub FindName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut *mut core::ffi::c_void, *mut MEMBERID, *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub ReleaseTLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TLIBATTR),
    #[cfg(not(feature = "winnt"))]
    ReleaseTLibAttr: usize,
}
#[cfg(feature = "winnt")]
pub trait ITypeLib_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> u32;
    fn GetTypeInfo(&self, index: u32) -> windows_core::Result<ITypeInfo>;
    fn GetTypeInfoType(&self, index: u32) -> windows_core::Result<TYPEKIND>;
    fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo>;
    fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetDocumentation(&self, index: i32, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn FindName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: windows_core::OutRef<ITypeInfo>, rgmemid: *mut MEMBERID, pcfound: *mut u16) -> windows_core::Result<()>;
    fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR);
}
#[cfg(feature = "winnt")]
impl ITypeLib_Vtbl {
    pub const fn new<Identity: ITypeLib_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::GetTypeInfoCount(this)
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeInfo(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeInfoType<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeInfoType(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ptkind.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeInfoOfGuid<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeInfoOfGuid(this, core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLibAttr<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetLibAttr(this) {
                    Ok(ok__) => {
                        pptlibattr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeComp(this) {
                    Ok(ok__) => {
                        pptcomp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentation<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pbstrname: *mut *mut core::ffi::c_void, pbstrdocstring: *mut *mut core::ffi::c_void, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::GetDocumentation(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
            }
        }
        unsafe extern "system" fn IsName<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::IsName(this, core::mem::transmute_copy(&sznamebuf), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pfname)).into()
            }
        }
        unsafe extern "system" fn FindName<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: *mut *mut core::ffi::c_void, rgmemid: *mut MEMBERID, pcfound: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::FindName(this, core::mem::transmute_copy(&sznamebuf), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&rgmemid), core::mem::transmute_copy(&pcfound)).into()
            }
        }
        unsafe extern "system" fn ReleaseTLibAttr<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptlibattr: *const TLIBATTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::ReleaseTLibAttr(this, core::mem::transmute_copy(&ptlibattr));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetTypeInfoType: GetTypeInfoType::<Identity, OFFSET>,
            GetTypeInfoOfGuid: GetTypeInfoOfGuid::<Identity, OFFSET>,
            GetLibAttr: GetLibAttr::<Identity, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, OFFSET>,
            IsName: IsName::<Identity, OFFSET>,
            FindName: FindName::<Identity, OFFSET>,
            ReleaseTLibAttr: ReleaseTLibAttr::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLib as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ITypeLib {}
windows_core::imp::define_interface!(ITypeLib2, ITypeLib2_Vtbl, 0x00020411_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeLib2 {
    type Target = ITypeLib;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeLib2, windows_core::IUnknown, ITypeLib);
impl ITypeLib2 {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLibStatistics)(windows_core::Interface::as_raw(self), pcuniquenames as _, pcchuniquenames as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: super::winnt::LCID, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), index, lcid, core::mem::transmute(pbstrhelpstring), pdwhelpstringcontext as _, core::mem::transmute(pbstrhelpstringdll)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllCustData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2_Vtbl {
    pub base__: ITypeLib_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetCustData: usize,
    pub GetLibStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::winnt::LCID, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetDocumentation2: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetAllCustData: usize,
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeLib2_Impl: ITypeLib_Impl {
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::Result<()>;
    fn GetDocumentation2(&self, index: i32, lcid: super::winnt::LCID, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITypeLib2_Vtbl {
    pub const fn new<Identity: ITypeLib2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCustData<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib2_Impl::GetCustData(this, core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLibStatistics<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib2_Impl::GetLibStatistics(this, core::mem::transmute_copy(&pcuniquenames), core::mem::transmute_copy(&pcchuniquenames)).into()
            }
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, lcid: super::winnt::LCID, pbstrhelpstring: *mut *mut core::ffi::c_void, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib2_Impl::GetDocumentation2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpstring), core::mem::transmute_copy(&pdwhelpstringcontext), core::mem::transmute_copy(&pbstrhelpstringdll)).into()
            }
        }
        unsafe extern "system" fn GetAllCustData<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib2_Impl::GetAllCustData(this) {
                    Ok(ok__) => {
                        pcustdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITypeLib_Vtbl::new::<Identity, OFFSET>(),
            GetCustData: GetCustData::<Identity, OFFSET>,
            GetLibStatistics: GetLibStatistics::<Identity, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLib2 as windows_core::Interface>::IID || iid == &<ITypeLib as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITypeLib2 {}
windows_core::imp::define_interface!(ITypeLibRegistration, ITypeLibRegistration_Vtbl, 0x76a3e735_02df_4a12_98eb_043ad3600af3);
windows_core::imp::interface_hierarchy!(ITypeLibRegistration, windows_core::IUnknown);
impl ITypeLibRegistration {
    pub unsafe fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLcid(&self) -> windows_core::Result<super::winnt::LCID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLcid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetWin32Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWin32Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetWin64Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWin64Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHelpDir(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpDir)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetLcid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLcid: usize,
    pub GetWin32Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWin64Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetHelpDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait ITypeLibRegistration_Impl: windows_core::IUnknownImpl {
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcid(&self) -> windows_core::Result<super::winnt::LCID>;
    fn GetWin32Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetWin64Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn GetHelpDir(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "winnt")]
impl ITypeLibRegistration_Vtbl {
    pub const fn new<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGuid<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetGuid(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        pversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLcid<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetLcid(this) {
                    Ok(ok__) => {
                        plcid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWin32Path<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwin32path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetWin32Path(this) {
                    Ok(ok__) => {
                        pwin32path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWin64Path<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwin64path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetWin64Path(this) {
                    Ok(ok__) => {
                        pwin64path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlags<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpDir<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phelpdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetHelpDir(this) {
                    Ok(ok__) => {
                        phelpdir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetLcid: GetLcid::<Identity, OFFSET>,
            GetWin32Path: GetWin32Path::<Identity, OFFSET>,
            GetWin64Path: GetWin64Path::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetHelpDir: GetHelpDir::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLibRegistration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ITypeLibRegistration {}
windows_core::imp::define_interface!(ITypeLibRegistrationReader, ITypeLibRegistrationReader_Vtbl, 0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
windows_core::imp::interface_hierarchy!(ITypeLibRegistrationReader, windows_core::IUnknown);
impl ITypeLibRegistrationReader {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn EnumTypeLibRegistrations(&self) -> windows_core::Result<super::objidlbase::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumTypeLibRegistrations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub EnumTypeLibRegistrations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    EnumTypeLibRegistrations: usize,
}
#[cfg(feature = "objidlbase")]
pub trait ITypeLibRegistrationReader_Impl: windows_core::IUnknownImpl {
    fn EnumTypeLibRegistrations(&self) -> windows_core::Result<super::objidlbase::IEnumUnknown>;
}
#[cfg(feature = "objidlbase")]
impl ITypeLibRegistrationReader_Vtbl {
    pub const fn new<Identity: ITypeLibRegistrationReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumTypeLibRegistrations<Identity: ITypeLibRegistrationReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistrationReader_Impl::EnumTypeLibRegistrations(this) {
                    Ok(ok__) => {
                        ppenumunknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumTypeLibRegistrations: EnumTypeLibRegistrations::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLibRegistrationReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for ITypeLibRegistrationReader {}
windows_core::imp::define_interface!(ITypeMarshal, ITypeMarshal_Vtbl, 0x0000002d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeMarshal, windows_core::IUnknown);
impl ITypeMarshal {
    pub unsafe fn Size(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), pvtype, dwdestcontext, pvdestcontext, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Marshal(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, pbuffer: &mut [u8], pcbwritten: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Marshal)(windows_core::Interface::as_raw(self), pvtype, dwdestcontext, pvdestcontext, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), pcbwritten as _) }
    }
    pub unsafe fn Unmarshal(&self, pvtype: *mut core::ffi::c_void, dwflags: u32, pbuffer: &[u8], pcbread: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unmarshal)(windows_core::Interface::as_raw(self), pvtype as _, dwflags, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), pcbread as _) }
    }
    pub unsafe fn Free(&self, pvtype: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pvtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Marshal: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Unmarshal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const u8, *mut u32) -> windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITypeMarshal_Impl: windows_core::IUnknownImpl {
    fn Size(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<u32>;
    fn Marshal(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> windows_core::Result<()>;
    fn Unmarshal(&self, pvtype: *mut core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> windows_core::Result<()>;
    fn Free(&self, pvtype: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl ITypeMarshal_Vtbl {
    pub const fn new<Identity: ITypeMarshal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Size<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, psize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeMarshal_Impl::Size(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext)) {
                    Ok(ok__) => {
                        psize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Marshal<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeMarshal_Impl::Marshal(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&cbbufferlength), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Unmarshal<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *mut core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeMarshal_Impl::Unmarshal(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cbbufferlength), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn Free<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeMarshal_Impl::Free(this, core::mem::transmute_copy(&pvtype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Size: Size::<Identity, OFFSET>,
            Marshal: Marshal::<Identity, OFFSET>,
            Unmarshal: Unmarshal::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeMarshal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeMarshal {}
pub type LIBFLAGS = i32;
pub const LIBFLAG_FCONTROL: LIBFLAGS = 2;
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = 8;
pub const LIBFLAG_FHIDDEN: LIBFLAGS = 4;
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = 1;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPBINDPTR = *mut BINDPTR;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPCUSTDATA = *mut CUSTDATA;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPCUSTDATAITEM = *mut CUSTDATAITEM;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPELEMDESC = *mut ELEMDESC;
#[cfg(feature = "wtypesbase")]
pub type LPEXCEPINFO = *mut EXCEPINFO;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPFUNCDESC = *mut FUNCDESC;
pub type LPIDLDESC = *mut IDLDESC;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDESC = *mut PARAMDESC;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDESCEX = *mut PARAMDESCEX;
pub type LPSAFEARRAY = *mut SAFEARRAY;
pub type LPSAFEARRAYBOUND = *mut SAFEARRAYBOUND;
#[cfg(feature = "winnt")]
pub type LPTLIBATTR = *mut TLIBATTR;
#[cfg(all(feature = "winnt", feature = "wtypes"))]
pub type LPTYPEATTR = *mut TYPEATTR;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARDESC = *mut VARDESC;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARIANT = *mut VARIANT;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARIANTARG = *mut VARIANT;
pub type MEMBERID = DISPID;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PARAMDESC {
    pub pparamdescex: LPPARAMDESCEX,
    pub wParamFlags: u16,
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: VARIANTARG,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PARAMFLAG_FHASCUSTDATA: u32 = 64;
pub const PARAMFLAG_FHASDEFAULT: u32 = 32;
pub const PARAMFLAG_FIN: u32 = 1;
pub const PARAMFLAG_FLCID: u32 = 4;
pub const PARAMFLAG_FOPT: u32 = 16;
pub const PARAMFLAG_FOUT: u32 = 2;
pub const PARAMFLAG_FRETVAL: u32 = 8;
pub const PARAMFLAG_NONE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SAFEARRAYUNION {
    pub sfType: u32,
    pub u: SAFEARRAYUNION_0,
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union SAFEARRAYUNION_0 {
    pub BstrStr: SAFEARR_BSTR,
    pub UnknownStr: SAFEARR_UNKNOWN,
    pub DispatchStr: SAFEARR_DISPATCH,
    pub VariantStr: SAFEARR_VARIANT,
    pub RecordStr: SAFEARR_BRECORD,
    pub HaveIidStr: SAFEARR_HAVEIID,
    pub ByteStr: super::wtypesbase::BYTE_SIZEDARR,
    pub WordStr: super::wtypesbase::WORD_SIZEDARR,
    pub LongStr: super::wtypesbase::DWORD_SIZEDARR,
    pub HyperStr: super::wtypesbase::HYPER_SIZEDARR,
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARRAYUNION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARR_BRECORD {
    pub Size: u32,
    pub aRecord: *mut wireBRECORD,
}
#[cfg(feature = "rpc")]
impl Default for SAFEARR_BRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARR_BSTR {
    pub Size: u32,
    pub aBstr: *mut super::wtypes::wireBSTR,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARR_BSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut Option<IDispatch>,
}
impl Default for SAFEARR_DISPATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut Option<windows_core::IUnknown>,
    pub iid: windows_core::GUID,
}
impl Default for SAFEARR_HAVEIID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut Option<windows_core::IUnknown>,
}
impl Default for SAFEARR_UNKNOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut wireVARIANT,
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARR_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SF_BSTR: SF_TYPE = 8;
pub const SF_DISPATCH: SF_TYPE = 9;
pub const SF_ERROR: SF_TYPE = 10;
pub const SF_HAVEIID: SF_TYPE = 32781;
pub const SF_I1: SF_TYPE = 16;
pub const SF_I2: SF_TYPE = 2;
pub const SF_I4: SF_TYPE = 3;
pub const SF_I8: SF_TYPE = 20;
pub const SF_RECORD: SF_TYPE = 36;
pub type SF_TYPE = i32;
pub const SF_UNKNOWN: SF_TYPE = 13;
pub const SF_VARIANT: SF_TYPE = 12;
pub type SYSKIND = i32;
pub const SYS_MAC: SYSKIND = 2;
pub const SYS_WIN16: SYSKIND = 0;
pub const SYS_WIN32: SYSKIND = 1;
pub const SYS_WIN64: SYSKIND = 3;
pub const TKIND_ALIAS: TYPEKIND = 6;
pub const TKIND_COCLASS: TYPEKIND = 5;
pub const TKIND_DISPATCH: TYPEKIND = 4;
pub const TKIND_ENUM: TYPEKIND = 0;
pub const TKIND_INTERFACE: TYPEKIND = 3;
pub const TKIND_MAX: TYPEKIND = 8;
pub const TKIND_MODULE: TYPEKIND = 2;
pub const TKIND_RECORD: TYPEKIND = 1;
pub const TKIND_UNION: TYPEKIND = 7;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TLIBATTR {
    pub guid: windows_core::GUID,
    pub lcid: super::winnt::LCID,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct TYPEATTR {
    pub guid: windows_core::GUID,
    pub lcid: super::winnt::LCID,
    pub dwReserved: u32,
    pub memidConstructor: MEMBERID,
    pub memidDestructor: MEMBERID,
    pub lpstrSchema: windows_core::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
impl Default for TYPEATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: super::wtypes::VARTYPE,
}
#[cfg(feature = "wtypes")]
impl Default for TYPEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: HREFTYPE,
}
#[cfg(feature = "wtypes")]
impl Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TYPEFLAGS = i32;
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = 1024;
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = 1;
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = 2;
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = 32;
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = 4096;
pub const TYPEFLAG_FDUAL: TYPEFLAGS = 64;
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = 16;
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = 4;
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = 128;
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = 256;
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = 8;
pub const TYPEFLAG_FPROXY: TYPEFLAGS = 16384;
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = 2048;
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = 512;
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = 8192;
pub type TYPEKIND = i32;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct VARDESC {
    pub memid: MEMBERID,
    pub lpstrSchema: windows_core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VARFLAGS = i32;
pub const VARFLAG_FBINDABLE: VARFLAGS = 4;
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = 32;
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = 256;
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = 16;
pub const VARFLAG_FHIDDEN: VARFLAGS = 64;
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = 4096;
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = 1024;
pub const VARFLAG_FREADONLY: VARFLAGS = 1;
pub const VARFLAG_FREPLACEABLE: VARFLAGS = 2048;
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = 8;
pub const VARFLAG_FRESTRICTED: VARFLAGS = 128;
pub const VARFLAG_FSOURCE: VARFLAGS = 2;
pub const VARFLAG_FUIDEFAULT: VARFLAGS = 512;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Clone for VARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub union VARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::wtypes::DECIMAL,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub struct VARIANT_0_0 {
    pub vt: super::wtypes::VARTYPE,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::wtypes::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::wtypes::VARIANT_BOOL,
    pub scode: super::wtypesbase::SCODE,
    pub cyVal: super::wtypes::CY,
    pub date: f64,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::wtypes::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::wtypes::VARIANT_BOOL,
    pub pscode: *mut super::wtypesbase::SCODE,
    pub pcyVal: *mut super::wtypes::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut core::ffi::c_void,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::wtypes::DECIMAL,
    pub pcVal: *mut i8,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Debug, PartialEq)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: core::mem::ManuallyDrop<Option<IRecordInfo>>,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type VARIANTARG = VARIANT;
pub type VARKIND = i32;
pub const VAR_CONST: VARKIND = 2;
pub const VAR_DISPATCH: VARKIND = 3;
pub const VAR_PERINSTANCE: VARKIND = 0;
pub const VAR_STATIC: VARKIND = 1;
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Debug, PartialEq)]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: core::mem::ManuallyDrop<Option<IRecordInfo>>,
    pub pRecord: *mut super::rpc::byte,
}
#[cfg(feature = "rpc")]
impl Default for _wireBRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: SAFEARRAYUNION,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for _wireVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
pub union _wireVARIANT_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::wtypes::VARIANT_BOOL,
    pub scode: super::wtypesbase::SCODE,
    pub cyVal: super::wtypes::CY,
    pub date: f64,
    pub bstrVal: super::wtypes::wireBSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub parray: wirePSAFEARRAY,
    pub brecVal: wireBRECORD,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::wtypes::VARIANT_BOOL,
    pub pscode: *mut super::wtypesbase::SCODE,
    pub pcyVal: *mut super::wtypes::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut super::wtypes::wireBSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut wirePSAFEARRAY,
    pub pvarVal: *mut wireVARIANT,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: super::wtypes::DECIMAL,
    pub pdecVal: *mut super::wtypes::DECIMAL,
    pub pcVal: *mut i8,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for _wireVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "rpc")]
pub type wireBRECORD = *mut _wireBRECORD;
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
pub type wirePSAFEARRAY = *mut wireSAFEARRAY;
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
pub type wireSAFEARRAY = *mut _wireSAFEARRAY;
#[cfg(all(feature = "rpc", feature = "wtypes", feature = "wtypesbase"))]
pub type wireVARIANT = *mut _wireVARIANT;
