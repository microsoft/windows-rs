#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: super::USHORT,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: core::mem::ManuallyDrop<Option<ITypeComp>>,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for BINDPTR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Default)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pStorage: super::PVOID,
    pub flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub type CURRENCY = super::CY;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct CUSTDATA {
    pub cCustData: super::DWORD,
    pub prgCustData: LPCUSTDATAITEM,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct CUSTDATAITEM {
    pub guid: windows_core::GUID,
    pub varValue: VARIANTARG,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
pub const DISPID_COLLECT: i32 = -8;
pub const DISPID_CONSTRUCTOR: i32 = -6;
pub const DISPID_DESTRUCTOR: i32 = -7;
pub const DISPID_EVALUATE: i32 = -5;
pub const DISPID_NEWENUM: i32 = -4;
pub const DISPID_PROPERTYPUT: i32 = -3;
pub const DISPID_UNKNOWN: i32 = -1;
pub const DISPID_VALUE: i32 = 0;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANTARG,
    pub rgdispidNamedArgs: *mut super::DISPID,
    pub cArgs: super::UINT,
    pub cNamedArgs: super::UINT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for ELEMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
#[derive(Default)]
pub struct EXCEPINFO {
    pub wCode: super::WORD,
    pub wReserved: super::WORD,
    pub bstrSource: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrDescription: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrHelpFile: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dwHelpContext: super::DWORD,
    pub pvReserved: super::PVOID,
    pub pfnDeferredFillIn: Option<unsafe extern "system" fn(*mut Self) -> windows_core::HRESULT>,
    pub scode: super::SCODE,
}
pub const FADF_AUTO: i32 = 1;
pub const FADF_BSTR: i32 = 256;
pub const FADF_DISPATCH: i32 = 1024;
pub const FADF_EMBEDDED: i32 = 4;
pub const FADF_FIXEDSIZE: i32 = 16;
pub const FADF_HAVEIID: i32 = 64;
pub const FADF_HAVEVARTYPE: i32 = 128;
pub const FADF_RECORD: i32 = 32;
pub const FADF_RESERVED: i32 = 61448;
pub const FADF_STATIC: i32 = 2;
pub const FADF_UNKNOWN: i32 = 512;
pub const FADF_VARIANT: i32 = 2048;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FUNCDESC {
    pub memid: super::MEMBERID,
    pub lprgscode: *mut super::SCODE,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: super::SHORT,
    pub cParamsOpt: super::SHORT,
    pub oVft: super::SHORT,
    pub cScodes: super::SHORT,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: super::WORD,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
pub type HREFTYPE = u32;
windows_core::imp::define_interface!(ICreateErrorInfo, ICreateErrorInfo_Vtbl, 0x22f03340_547d_101b_8e65_08002b2bd119);
windows_core::imp::interface_hierarchy!(ICreateErrorInfo, windows_core::IUnknown);
impl ICreateErrorInfo {
    pub unsafe fn SetGUID(&self, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGUID)(windows_core::Interface::as_raw(self), rguid) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetSource(&self, szsource: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), szsource) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetDescription(&self, szdescription: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), szdescription) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetHelpFile(&self, szhelpfile: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpFile)(windows_core::Interface::as_raw(self), szhelpfile) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetSource: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetDescription: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetHelpFile: usize,
    #[cfg(feature = "shared")]
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHelpContext: usize,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait ICreateErrorInfo_Impl: windows_core::IUnknownImpl {
    fn SetGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetSource(&self, szsource: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetDescription(&self, szdescription: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetHelpFile(&self, szhelpfile: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl ICreateErrorInfo_Vtbl {
    pub const fn new<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGUID<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetGUID(this, core::mem::transmute_copy(&rguid)).into()
            }
        }
        unsafe extern "system" fn SetSource<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsource: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetSource(this, core::mem::transmute_copy(&szsource)).into()
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdescription: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetDescription(this, core::mem::transmute_copy(&szdescription)).into()
            }
        }
        unsafe extern "system" fn SetHelpFile<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhelpfile: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateErrorInfo_Impl::SetHelpFile(this, core::mem::transmute_copy(&szhelpfile)).into()
            }
        }
        unsafe extern "system" fn SetHelpContext<Identity: ICreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICreateErrorInfo {}
windows_core::imp::define_interface!(ICreateTypeInfo, ICreateTypeInfo_Vtbl, 0x00020405_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICreateTypeInfo, windows_core::IUnknown);
impl ICreateTypeInfo {
    pub unsafe fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetTypeFlags(&self, utypeflags: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypeFlags)(windows_core::Interface::as_raw(self), utypeflags) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetDocString(&self, pstrdoc: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDocString)(windows_core::Interface::as_raw(self), pstrdoc) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVersion(&self, wmajorvernum: super::WORD, wminorvernum: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), wmajorvernum, wminorvernum) }
    }
    pub unsafe fn AddRefTypeInfo<P0>(&self, ptinfo: P0, phreftype: *const HREFTYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITypeInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRefTypeInfo)(windows_core::Interface::as_raw(self), ptinfo.param().abi(), phreftype) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddFuncDesc(&self, index: super::UINT, pfuncdesc: *const FUNCDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddFuncDesc)(windows_core::Interface::as_raw(self), index, pfuncdesc) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddImplType(&self, index: super::UINT, hreftype: HREFTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddImplType)(windows_core::Interface::as_raw(self), index, hreftype) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetImplTypeFlags(&self, index: super::UINT, impltypeflags: super::INT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetImplTypeFlags)(windows_core::Interface::as_raw(self), index, impltypeflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAlignment(&self, cbalignment: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAlignment)(windows_core::Interface::as_raw(self), cbalignment) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetSchema(&self, pstrschema: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSchema)(windows_core::Interface::as_raw(self), pstrschema) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddVarDesc(&self, index: super::UINT, pvardesc: *const VARDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddVarDesc)(windows_core::Interface::as_raw(self), index, pvardesc) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn SetFuncAndParamNames(&self, index: super::UINT, rgsznames: &[super::LPOLESTR]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncAndParamNames)(windows_core::Interface::as_raw(self), index, rgsznames.as_ptr(), rgsznames.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn SetVarName(&self, index: super::UINT, szname: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarName)(windows_core::Interface::as_raw(self), index, szname) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetTypeDescAlias(&self, ptdescalias: *const TYPEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypeDescAlias)(windows_core::Interface::as_raw(self), ptdescalias) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn DefineFuncAsDllEntry(&self, index: super::UINT, szdllname: super::LPOLESTR, szprocname: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DefineFuncAsDllEntry)(windows_core::Interface::as_raw(self), index, szdllname, szprocname) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn SetFuncDocString(&self, index: super::UINT, szdocstring: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncDocString)(windows_core::Interface::as_raw(self), index, szdocstring) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn SetVarDocString(&self, index: super::UINT, szdocstring: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarDocString)(windows_core::Interface::as_raw(self), index, szdocstring) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFuncHelpContext(&self, index: super::UINT, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncHelpContext)(windows_core::Interface::as_raw(self), index, dwhelpcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVarHelpContext(&self, index: super::UINT, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarHelpContext)(windows_core::Interface::as_raw(self), index, dwhelpcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMops(&self, index: super::UINT, bstrmops: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMops)(windows_core::Interface::as_raw(self), index, core::mem::transmute_copy(bstrmops)) }
    }
    #[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub SetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetTypeFlags: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetDocString: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetDocString: usize,
    #[cfg(feature = "shared")]
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHelpContext: usize,
    #[cfg(feature = "shared")]
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVersion: usize,
    pub AddRefTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const HREFTYPE) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub AddFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const FUNCDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    AddFuncDesc: usize,
    #[cfg(feature = "shared")]
    pub AddImplType: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, HREFTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddImplType: usize,
    #[cfg(feature = "shared")]
    pub SetImplTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::INT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetImplTypeFlags: usize,
    #[cfg(feature = "shared")]
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAlignment: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetSchema: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetSchema: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub AddVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const VARDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    AddVarDesc: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub SetFuncAndParamNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const super::LPOLESTR, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    SetFuncAndParamNames: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub SetVarName: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    SetVarName: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetTypeDescAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const TYPEDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetTypeDescAlias: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub DefineFuncAsDllEntry: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LPOLESTR, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    DefineFuncAsDllEntry: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub SetFuncDocString: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    SetFuncDocString: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub SetVarDocString: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    SetVarDocString: usize,
    #[cfg(feature = "shared")]
    pub SetFuncHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFuncHelpContext: usize,
    #[cfg(feature = "shared")]
    pub SetVarHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVarHelpContext: usize,
    #[cfg(feature = "shared")]
    pub SetMops: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMops: usize,
    #[cfg(feature = "shared")]
    pub SetTypeIdldesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const IDLDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetTypeIdldesc: usize,
    pub LayOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICreateTypeInfo_Impl: windows_core::IUnknownImpl {
    fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetTypeFlags(&self, utypeflags: super::UINT) -> windows_core::Result<()>;
    fn SetDocString(&self, pstrdoc: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: super::DWORD) -> windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: super::WORD, wminorvernum: super::WORD) -> windows_core::Result<()>;
    fn AddRefTypeInfo(&self, ptinfo: windows_core::Ref<ITypeInfo>, phreftype: *const HREFTYPE) -> windows_core::Result<()>;
    fn AddFuncDesc(&self, index: super::UINT, pfuncdesc: *const FUNCDESC) -> windows_core::Result<()>;
    fn AddImplType(&self, index: super::UINT, hreftype: HREFTYPE) -> windows_core::Result<()>;
    fn SetImplTypeFlags(&self, index: super::UINT, impltypeflags: super::INT) -> windows_core::Result<()>;
    fn SetAlignment(&self, cbalignment: super::WORD) -> windows_core::Result<()>;
    fn SetSchema(&self, pstrschema: super::LPOLESTR) -> windows_core::Result<()>;
    fn AddVarDesc(&self, index: super::UINT, pvardesc: *const VARDESC) -> windows_core::Result<()>;
    fn SetFuncAndParamNames(&self, index: super::UINT, rgsznames: *const super::LPOLESTR, cnames: super::UINT) -> windows_core::Result<()>;
    fn SetVarName(&self, index: super::UINT, szname: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetTypeDescAlias(&self, ptdescalias: *const TYPEDESC) -> windows_core::Result<()>;
    fn DefineFuncAsDllEntry(&self, index: super::UINT, szdllname: super::LPOLESTR, szprocname: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetFuncDocString(&self, index: super::UINT, szdocstring: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetVarDocString(&self, index: super::UINT, szdocstring: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetFuncHelpContext(&self, index: super::UINT, dwhelpcontext: super::DWORD) -> windows_core::Result<()>;
    fn SetVarHelpContext(&self, index: super::UINT, dwhelpcontext: super::DWORD) -> windows_core::Result<()>;
    fn SetMops(&self, index: super::UINT, bstrmops: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTypeIdldesc(&self, pidldesc: *const IDLDESC) -> windows_core::Result<()>;
    fn LayOut(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICreateTypeInfo_Vtbl {
    pub const fn new<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGuid<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetGuid(this, core::mem::transmute_copy(&guid)).into()
            }
        }
        unsafe extern "system" fn SetTypeFlags<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, utypeflags: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetTypeFlags(this, core::mem::transmute_copy(&utypeflags)).into()
            }
        }
        unsafe extern "system" fn SetDocString<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdoc: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetDocString(this, core::mem::transmute_copy(&pstrdoc)).into()
            }
        }
        unsafe extern "system" fn SetHelpContext<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmajorvernum: super::WORD, wminorvernum: super::WORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddFuncDesc<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pfuncdesc: *const FUNCDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::AddFuncDesc(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pfuncdesc)).into()
            }
        }
        unsafe extern "system" fn AddImplType<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, hreftype: HREFTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::AddImplType(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&hreftype)).into()
            }
        }
        unsafe extern "system" fn SetImplTypeFlags<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, impltypeflags: super::INT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetImplTypeFlags(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&impltypeflags)).into()
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbalignment: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetAlignment(this, core::mem::transmute_copy(&cbalignment)).into()
            }
        }
        unsafe extern "system" fn SetSchema<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrschema: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetSchema(this, core::mem::transmute_copy(&pstrschema)).into()
            }
        }
        unsafe extern "system" fn AddVarDesc<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pvardesc: *const VARDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::AddVarDesc(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pvardesc)).into()
            }
        }
        unsafe extern "system" fn SetFuncAndParamNames<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, rgsznames: *const super::LPOLESTR, cnames: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetFuncAndParamNames(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames)).into()
            }
        }
        unsafe extern "system" fn SetVarName<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, szname: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetVarName(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&szname)).into()
            }
        }
        unsafe extern "system" fn SetTypeDescAlias<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptdescalias: *const TYPEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetTypeDescAlias(this, core::mem::transmute_copy(&ptdescalias)).into()
            }
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, szdllname: super::LPOLESTR, szprocname: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::DefineFuncAsDllEntry(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&szdllname), core::mem::transmute_copy(&szprocname)).into()
            }
        }
        unsafe extern "system" fn SetFuncDocString<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, szdocstring: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetFuncDocString(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&szdocstring)).into()
            }
        }
        unsafe extern "system" fn SetVarDocString<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, szdocstring: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetVarDocString(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&szdocstring)).into()
            }
        }
        unsafe extern "system" fn SetFuncHelpContext<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetFuncHelpContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetVarHelpContext<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo_Impl::SetVarHelpContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetMops<Identity: ICreateTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, bstrmops: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteFuncDesc(&self, index: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteFuncDesc)(windows_core::Interface::as_raw(self), index) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteFuncDescByMemId(&self, memid: super::MEMBERID, invkind: INVOKEKIND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteFuncDescByMemId)(windows_core::Interface::as_raw(self), memid, invkind) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteVarDesc(&self, index: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteVarDesc)(windows_core::Interface::as_raw(self), index) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteVarDescByMemId(&self, memid: super::MEMBERID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteVarDescByMemId)(windows_core::Interface::as_raw(self), memid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteImplType(&self, index: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteImplType)(windows_core::Interface::as_raw(self), index) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCustData)(windows_core::Interface::as_raw(self), guid, pvarval) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetFuncCustData(&self, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncCustData)(windows_core::Interface::as_raw(self), index, guid, pvarval) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetParamCustData(&self, indexfunc: super::UINT, indexparam: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, guid, pvarval) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetVarCustData(&self, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarCustData)(windows_core::Interface::as_raw(self), index, guid, pvarval) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetImplTypeCustData(&self, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetImplTypeCustData)(windows_core::Interface::as_raw(self), index, guid, pvarval) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpStringContext)(windows_core::Interface::as_raw(self), dwhelpstringcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFuncHelpStringContext(&self, index: super::UINT, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFuncHelpStringContext)(windows_core::Interface::as_raw(self), index, dwhelpstringcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVarHelpStringContext(&self, index: super::UINT, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarHelpStringContext)(windows_core::Interface::as_raw(self), index, dwhelpstringcontext) }
    }
    pub unsafe fn Invalidate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetName(&self, szname: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), szname) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeInfo2_Vtbl {
    pub base__: ICreateTypeInfo_Vtbl,
    #[cfg(feature = "shared")]
    pub DeleteFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteFuncDesc: usize,
    #[cfg(feature = "shared")]
    pub DeleteFuncDescByMemId: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, INVOKEKIND) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteFuncDescByMemId: usize,
    #[cfg(feature = "shared")]
    pub DeleteVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteVarDesc: usize,
    #[cfg(feature = "shared")]
    pub DeleteVarDescByMemId: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteVarDescByMemId: usize,
    #[cfg(feature = "shared")]
    pub DeleteImplType: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteImplType: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetFuncCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetParamCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetVarCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetImplTypeCustData: usize,
    #[cfg(feature = "shared")]
    pub SetHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHelpStringContext: usize,
    #[cfg(feature = "shared")]
    pub SetFuncHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFuncHelpStringContext: usize,
    #[cfg(feature = "shared")]
    pub SetVarHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVarHelpStringContext: usize,
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetName: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICreateTypeInfo2_Impl: ICreateTypeInfo_Impl {
    fn DeleteFuncDesc(&self, index: super::UINT) -> windows_core::Result<()>;
    fn DeleteFuncDescByMemId(&self, memid: super::MEMBERID, invkind: INVOKEKIND) -> windows_core::Result<()>;
    fn DeleteVarDesc(&self, index: super::UINT) -> windows_core::Result<()>;
    fn DeleteVarDescByMemId(&self, memid: super::MEMBERID) -> windows_core::Result<()>;
    fn DeleteImplType(&self, index: super::UINT) -> windows_core::Result<()>;
    fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetFuncCustData(&self, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetParamCustData(&self, indexfunc: super::UINT, indexparam: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetVarCustData(&self, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetImplTypeCustData(&self, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: super::ULONG) -> windows_core::Result<()>;
    fn SetFuncHelpStringContext(&self, index: super::UINT, dwhelpstringcontext: super::ULONG) -> windows_core::Result<()>;
    fn SetVarHelpStringContext(&self, index: super::UINT, dwhelpstringcontext: super::ULONG) -> windows_core::Result<()>;
    fn Invalidate(&self) -> windows_core::Result<()>;
    fn SetName(&self, szname: super::LPOLESTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICreateTypeInfo2_Vtbl {
    pub const fn new<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteFuncDesc<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteFuncDesc(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, invkind: INVOKEKIND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteFuncDescByMemId(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind)).into()
            }
        }
        unsafe extern "system" fn DeleteVarDesc<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteVarDesc(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::DeleteVarDescByMemId(this, core::mem::transmute_copy(&memid)).into()
            }
        }
        unsafe extern "system" fn DeleteImplType<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetFuncCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetFuncCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetParamCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: super::UINT, indexparam: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetVarCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetVarCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetImplTypeCustData<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetImplTypeCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetHelpStringContext(this, core::mem::transmute_copy(&dwhelpstringcontext)).into()
            }
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetFuncHelpStringContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpstringcontext)).into()
            }
        }
        unsafe extern "system" fn SetVarHelpStringContext<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetName<Identity: ICreateTypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeInfo2_Impl::SetName(this, core::mem::transmute_copy(&szname)).into()
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICreateTypeInfo2 {}
windows_core::imp::define_interface!(ICreateTypeLib, ICreateTypeLib_Vtbl, 0x00020406_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICreateTypeLib, windows_core::IUnknown);
impl ICreateTypeLib {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn CreateTypeInfo(&self, szname: super::LPOLESTR, tkind: TYPEKIND) -> windows_core::Result<ICreateTypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTypeInfo)(windows_core::Interface::as_raw(self), szname, tkind, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetName(&self, szname: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), szname) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVersion(&self, wmajorvernum: super::WORD, wminorvernum: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), wmajorvernum, wminorvernum) }
    }
    pub unsafe fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetDocString(&self, szdoc: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDocString)(windows_core::Interface::as_raw(self), szdoc) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetHelpFileName(&self, szhelpfilename: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpFileName)(windows_core::Interface::as_raw(self), szhelpfilename) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetLcid(&self, lcid: super::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLcid)(windows_core::Interface::as_raw(self), lcid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetLibFlags(&self, ulibflags: super::UINT) -> windows_core::HRESULT {
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
    #[cfg(feature = "wtypesbase")]
    pub CreateTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, TYPEKIND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    CreateTypeInfo: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetName: usize,
    #[cfg(feature = "shared")]
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVersion: usize,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub SetDocString: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetDocString: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetHelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetHelpFileName: usize,
    #[cfg(feature = "shared")]
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHelpContext: usize,
    #[cfg(feature = "shared")]
    pub SetLcid: unsafe extern "system" fn(*mut core::ffi::c_void, super::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetLcid: usize,
    #[cfg(feature = "shared")]
    pub SetLibFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetLibFlags: usize,
    pub SaveAllChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait ICreateTypeLib_Impl: windows_core::IUnknownImpl {
    fn CreateTypeInfo(&self, szname: super::LPOLESTR, tkind: TYPEKIND) -> windows_core::Result<ICreateTypeInfo>;
    fn SetName(&self, szname: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: super::WORD, wminorvernum: super::WORD) -> windows_core::Result<()>;
    fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetDocString(&self, szdoc: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetHelpFileName(&self, szhelpfilename: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: super::DWORD) -> windows_core::Result<()>;
    fn SetLcid(&self, lcid: super::LCID) -> windows_core::Result<()>;
    fn SetLibFlags(&self, ulibflags: super::UINT) -> windows_core::Result<()>;
    fn SaveAllChanges(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl ICreateTypeLib_Vtbl {
    pub const fn new<Identity: ICreateTypeLib_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTypeInfo<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: super::LPOLESTR, tkind: TYPEKIND, ppctinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICreateTypeLib_Impl::CreateTypeInfo(this, core::mem::transmute_copy(&szname), core::mem::transmute_copy(&tkind)) {
                    Ok(ok__) => {
                        ppctinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetName(this, core::mem::transmute_copy(&szname)).into()
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmajorvernum: super::WORD, wminorvernum: super::WORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetDocString<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdoc: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetDocString(this, core::mem::transmute_copy(&szdoc)).into()
            }
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhelpfilename: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetHelpFileName(this, core::mem::transmute_copy(&szhelpfilename)).into()
            }
        }
        unsafe extern "system" fn SetHelpContext<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn SetLcid<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib_Impl::SetLcid(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn SetLibFlags<Identity: ICreateTypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulibflags: super::UINT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
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
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn DeleteTypeInfo(&self, szname: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteTypeInfo)(windows_core::Interface::as_raw(self), szname) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCustData)(windows_core::Interface::as_raw(self), guid, pvarval) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpStringContext)(windows_core::Interface::as_raw(self), dwhelpstringcontext) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetHelpStringDll(&self, szfilename: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpStringDll)(windows_core::Interface::as_raw(self), szfilename) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeLib2_Vtbl {
    pub base__: ICreateTypeLib_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub DeleteTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    DeleteTypeInfo: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetCustData: usize,
    #[cfg(feature = "shared")]
    pub SetHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHelpStringContext: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetHelpStringDll: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetHelpStringDll: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICreateTypeLib2_Impl: ICreateTypeLib_Impl {
    fn DeleteTypeInfo(&self, szname: super::LPOLESTR) -> windows_core::Result<()>;
    fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: super::ULONG) -> windows_core::Result<()>;
    fn SetHelpStringDll(&self, szfilename: super::LPOLESTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICreateTypeLib2_Vtbl {
    pub const fn new<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteTypeInfo<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::DeleteTypeInfo(this, core::mem::transmute_copy(&szname)).into()
            }
        }
        unsafe extern "system" fn SetCustData<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::SetCustData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
            }
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpstringcontext: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::SetHelpStringContext(this, core::mem::transmute_copy(&dwhelpstringcontext)).into()
            }
        }
        unsafe extern "system" fn SetHelpStringDll<Identity: ICreateTypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szfilename: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateTypeLib2_Impl::SetHelpStringDll(this, core::mem::transmute_copy(&szfilename)).into()
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICreateTypeLib2 {}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IDLDESC {
    pub dwReserved: super::ULONG_PTR,
    pub wIDLFlags: super::USHORT,
}
pub const IDLFLAG_FIN: i32 = 1;
pub const IDLFLAG_FLCID: i32 = 4;
pub const IDLFLAG_FOUT: i32 = 2;
pub const IDLFLAG_FRETVAL: i32 = 8;
pub const IDLFLAG_NONE: i32 = 0;
windows_core::imp::define_interface!(IDispatch, IDispatch_Vtbl, 0x00020400_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
impl IDispatch {
    #[cfg(feature = "shared")]
    pub unsafe fn GetTypeInfoCount(&self) -> windows_core::Result<super::UINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTypeInfo(&self, itinfo: super::UINT, lcid: super::LCID) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const super::IID, rgsznames: *const super::LPOLESTR, cnames: super::UINT, lcid: super::LCID, rgdispid: *mut super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, rgdispid as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Invoke(&self, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *mut DISPPARAMS, pvarresult: Option<*mut VARIANT>, pexcepinfo: Option<*mut EXCEPINFO>, puargerr: Option<*mut super::UINT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams as _, pvarresult.unwrap_or(core::mem::zeroed()) as _, pexcepinfo.unwrap_or(core::mem::zeroed()) as _, puargerr.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTypeInfoCount: usize,
    #[cfg(feature = "shared")]
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTypeInfo: usize,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const super::LPOLESTR, super::UINT, super::LCID, *mut super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "wtypesbase")))]
    GetIDsOfNames: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, *const super::IID, super::LCID, super::WORD, *mut DISPPARAMS, *mut VARIANT, *mut EXCEPINFO, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Invoke: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispatch_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> windows_core::Result<super::UINT>;
    fn GetTypeInfo(&self, itinfo: super::UINT, lcid: super::LCID) -> windows_core::Result<ITypeInfo>;
    fn GetIDsOfNames(&self, riid: *const super::IID, rgsznames: *const super::LPOLESTR, cnames: super::UINT, lcid: super::LCID, rgdispid: *mut super::DISPID) -> windows_core::Result<()>;
    fn Invoke(&self, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispatch_Vtbl {
    pub const fn new<Identity: IDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctinfo: *mut super::UINT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetTypeInfo<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itinfo: super::UINT, lcid: super::LCID, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetIDsOfNames<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, rgsznames: *const super::LPOLESTR, cnames: super::UINT, lcid: super::LCID, rgdispid: *mut super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatch_Impl::GetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&rgdispid)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(IEnumVARIANT, IEnumVARIANT_Vtbl, 0x00020404_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumVARIANT, windows_core::IUnknown);
impl IEnumVARIANT {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: super::ULONG, rgvar: *mut VARIANT, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgvar, pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumVARIANT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut VARIANT, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IEnumVARIANT_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgvar: *mut VARIANT, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumVARIANT>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IEnumVARIANT_Vtbl {
    pub const fn new<Identity: IEnumVARIANT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgvar: *mut VARIANT, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumVARIANT_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetHelpContext(&self) -> windows_core::Result<super::DWORD> {
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
    #[cfg(feature = "shared")]
    pub GetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetHelpContext: usize,
}
#[cfg(feature = "shared")]
pub trait IErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpContext(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(feature = "shared")]
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
        unsafe extern "system" fn GetHelpContext<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhelpcontext: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IErrorInfo {}
windows_core::imp::define_interface!(IErrorLog, IErrorLog_Vtbl, 0x3127ca40_446e_11ce_8135_00aa004bb851);
windows_core::imp::interface_hierarchy!(IErrorLog, windows_core::IUnknown);
impl IErrorLog {
    #[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
    pub unsafe fn AddError(&self, pszpropname: super::LPCOLESTR, pexcepinfo: *const EXCEPINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddError)(windows_core::Interface::as_raw(self), pszpropname, pexcepinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
    pub AddError: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCOLESTR, *const EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt", feature = "wtypesbase")))]
    AddError: usize,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub trait IErrorLog_Impl: windows_core::IUnknownImpl {
    fn AddError(&self, pszpropname: super::LPCOLESTR, pexcepinfo: *const EXCEPINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl IErrorLog_Vtbl {
    pub const fn new<Identity: IErrorLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddError<Identity: IErrorLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: super::LPCOLESTR, pexcepinfo: *const EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorLog_Impl::AddError(this, core::mem::transmute_copy(&pszpropname), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddError: AddError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorLog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IErrorLog {}
pub const IMPLTYPEFLAG_FDEFAULT: i32 = 1;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: i32 = 8;
pub const IMPLTYPEFLAG_FRESTRICTED: i32 = 4;
pub const IMPLTYPEFLAG_FSOURCE: i32 = 2;
pub type INVOKEKIND = i32;
pub const INVOKE_FUNC: INVOKEKIND = 1;
pub const INVOKE_PROPERTYGET: INVOKEKIND = 2;
pub const INVOKE_PROPERTYPUT: INVOKEKIND = 4;
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = 8;
windows_core::imp::define_interface!(IPropertyBag, IPropertyBag_Vtbl, 0x55272a00_42cb_11ce_8135_00aa004bb851);
windows_core::imp::interface_hierarchy!(IPropertyBag, windows_core::IUnknown);
impl IPropertyBag {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Read<P2>(&self, pszpropname: super::LPCOLESTR, pvar: *mut VARIANT, perrorlog: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pszpropname, pvar, perrorlog.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Write(&self, pszpropname: super::LPCOLESTR, pvar: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pszpropname, pvar) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCOLESTR, *mut VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Read: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCOLESTR, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Write: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyBag_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pszpropname: super::LPCOLESTR, pvar: *mut VARIANT, perrorlog: windows_core::Ref<IErrorLog>) -> windows_core::Result<()>;
    fn Write(&self, pszpropname: super::LPCOLESTR, pvar: *const VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyBag_Vtbl {
    pub const fn new<Identity: IPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: super::LPCOLESTR, pvar: *mut VARIANT, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag_Impl::Read(this, core::mem::transmute_copy(&pszpropname), core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&perrorlog)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: super::LPCOLESTR, pvar: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag_Impl::Write(this, core::mem::transmute_copy(&pszpropname), core::mem::transmute_copy(&pvar)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyBag {}
windows_core::imp::define_interface!(IRecordInfo, IRecordInfo_Vtbl, 0x0000002f_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRecordInfo, windows_core::IUnknown);
impl IRecordInfo {
    #[cfg(feature = "winnt")]
    pub unsafe fn RecordInit(&self, pvnew: super::PVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordInit)(windows_core::Interface::as_raw(self), pvnew as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RecordClear(&self, pvexisting: super::PVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordClear)(windows_core::Interface::as_raw(self), pvexisting) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RecordCopy(&self, pvexisting: super::PVOID, pvnew: super::PVOID) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetSize(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeInfo(&self) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetField(&self, pvdata: super::PVOID, szfieldname: super::LPCOLESTR) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetField)(windows_core::Interface::as_raw(self), pvdata, szfieldname, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFieldNoCopy(&self, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *mut VARIANT, ppvdatacarray: *mut super::PVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFieldNoCopy)(windows_core::Interface::as_raw(self), pvdata, szfieldname, pvarfield, ppvdatacarray as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutField(&self, wflags: super::ULONG, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutField)(windows_core::Interface::as_raw(self), wflags, pvdata as _, szfieldname, pvarfield) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutFieldNoCopy(&self, wflags: super::ULONG, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *const VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutFieldNoCopy)(windows_core::Interface::as_raw(self), wflags, pvdata as _, szfieldname, pvarfield) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFieldNames(&self, pcnames: *mut super::ULONG, rgbstrnames: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFieldNames)(windows_core::Interface::as_raw(self), pcnames as _, core::mem::transmute(rgbstrnames)) }
    }
    pub unsafe fn IsMatchingType<P0>(&self, precordinfo: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsMatchingType)(windows_core::Interface::as_raw(self), precordinfo.param().abi()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RecordCreate(&self) -> super::PVOID {
        unsafe { (windows_core::Interface::vtable(self).RecordCreate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RecordCreateCopy(&self, pvsource: super::PVOID, ppvdest: *mut super::PVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordCreateCopy)(windows_core::Interface::as_raw(self), pvsource, ppvdest as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RecordDestroy(&self, pvrecord: super::PVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecordDestroy)(windows_core::Interface::as_raw(self), pvrecord) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecordInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub RecordInit: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RecordInit: usize,
    #[cfg(feature = "winnt")]
    pub RecordClear: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RecordClear: usize,
    #[cfg(feature = "winnt")]
    pub RecordCopy: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RecordCopy: usize,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSize: usize,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetField: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::LPCOLESTR, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetField: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::LPCOLESTR, *mut VARIANT, *mut super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetFieldNoCopy: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub PutField: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::PVOID, super::LPCOLESTR, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    PutField: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub PutFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::PVOID, super::LPCOLESTR, *const VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    PutFieldNoCopy: usize,
    #[cfg(feature = "shared")]
    pub GetFieldNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFieldNames: usize,
    pub IsMatchingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "winnt")]
    pub RecordCreate: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::PVOID,
    #[cfg(not(feature = "winnt"))]
    RecordCreate: usize,
    #[cfg(feature = "winnt")]
    pub RecordCreateCopy: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, *mut super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RecordCreateCopy: usize,
    #[cfg(feature = "winnt")]
    pub RecordDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RecordDestroy: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRecordInfo_Impl: windows_core::IUnknownImpl {
    fn RecordInit(&self, pvnew: super::PVOID) -> windows_core::Result<()>;
    fn RecordClear(&self, pvexisting: super::PVOID) -> windows_core::Result<()>;
    fn RecordCopy(&self, pvexisting: super::PVOID, pvnew: super::PVOID) -> windows_core::Result<()>;
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSize(&self) -> windows_core::Result<super::ULONG>;
    fn GetTypeInfo(&self) -> windows_core::Result<ITypeInfo>;
    fn GetField(&self, pvdata: super::PVOID, szfieldname: super::LPCOLESTR) -> windows_core::Result<VARIANT>;
    fn GetFieldNoCopy(&self, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *mut VARIANT, ppvdatacarray: *mut super::PVOID) -> windows_core::Result<()>;
    fn PutField(&self, wflags: super::ULONG, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *const VARIANT) -> windows_core::Result<()>;
    fn PutFieldNoCopy(&self, wflags: super::ULONG, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *const VARIANT) -> windows_core::Result<()>;
    fn GetFieldNames(&self, pcnames: *mut super::ULONG, rgbstrnames: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsMatchingType(&self, precordinfo: windows_core::Ref<IRecordInfo>) -> windows_core::BOOL;
    fn RecordCreate(&self) -> super::PVOID;
    fn RecordCreateCopy(&self, pvsource: super::PVOID, ppvdest: *mut super::PVOID) -> windows_core::Result<()>;
    fn RecordDestroy(&self, pvrecord: super::PVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRecordInfo_Vtbl {
    pub const fn new<Identity: IRecordInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RecordInit<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvnew: super::PVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordInit(this, core::mem::transmute_copy(&pvnew)).into()
            }
        }
        unsafe extern "system" fn RecordClear<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvexisting: super::PVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordClear(this, core::mem::transmute_copy(&pvexisting)).into()
            }
        }
        unsafe extern "system" fn RecordCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvexisting: super::PVOID, pvnew: super::PVOID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetSize<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetField<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *mut VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetField(this, core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&szfieldname)) {
                    Ok(ok__) => {
                        pvarfield.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *mut VARIANT, ppvdatacarray: *mut super::PVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::GetFieldNoCopy(this, core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&szfieldname), core::mem::transmute_copy(&pvarfield), core::mem::transmute_copy(&ppvdatacarray)).into()
            }
        }
        unsafe extern "system" fn PutField<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wflags: super::ULONG, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::PutField(this, core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&szfieldname), core::mem::transmute_copy(&pvarfield)).into()
            }
        }
        unsafe extern "system" fn PutFieldNoCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wflags: super::ULONG, pvdata: super::PVOID, szfieldname: super::LPCOLESTR, pvarfield: *const VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::PutFieldNoCopy(this, core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&szfieldname), core::mem::transmute_copy(&pvarfield)).into()
            }
        }
        unsafe extern "system" fn GetFieldNames<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnames: *mut super::ULONG, rgbstrnames: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RecordCreate<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::PVOID {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCreate(this)
            }
        }
        unsafe extern "system" fn RecordCreateCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvsource: super::PVOID, ppvdest: *mut super::PVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCreateCopy(this, core::mem::transmute_copy(&pvsource), core::mem::transmute_copy(&ppvdest)).into()
            }
        }
        unsafe extern "system" fn RecordDestroy<Identity: IRecordInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvrecord: super::PVOID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRecordInfo {}
windows_core::imp::define_interface!(ISupportErrorInfo, ISupportErrorInfo_Vtbl, 0xdf0b3d60_548f_101b_8e65_08002b2bd119);
windows_core::imp::interface_hierarchy!(ISupportErrorInfo, windows_core::IUnknown);
impl ISupportErrorInfo {
    #[cfg(feature = "guiddef")]
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InterfaceSupportsErrorInfo)(windows_core::Interface::as_raw(self), riid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub InterfaceSupportsErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    InterfaceSupportsErrorInfo: usize,
}
#[cfg(feature = "guiddef")]
pub trait ISupportErrorInfo_Impl: windows_core::IUnknownImpl {
    fn InterfaceSupportsErrorInfo(&self, riid: *const super::IID) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl ISupportErrorInfo_Vtbl {
    pub const fn new<Identity: ISupportErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Identity: ISupportErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for ISupportErrorInfo {}
windows_core::imp::define_interface!(ITypeChangeEvents, ITypeChangeEvents_Vtbl, 0x00020410_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeChangeEvents, windows_core::IUnknown);
impl ITypeChangeEvents {
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn RequestTypeChange<P1>(&self, changekind: CHANGEKIND, ptinfobefore: P1, pstrname: super::LPOLESTR) -> windows_core::Result<super::INT>
    where
        P1: windows_core::Param<ITypeInfo>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestTypeChange)(windows_core::Interface::as_raw(self), changekind, ptinfobefore.param().abi(), pstrname, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn AfterTypeChange<P1>(&self, changekind: CHANGEKIND, ptinfoafter: P1, pstrname: super::LPOLESTR) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITypeInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).AfterTypeChange)(windows_core::Interface::as_raw(self), changekind, ptinfoafter.param().abi(), pstrname) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeChangeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub RequestTypeChange: unsafe extern "system" fn(*mut core::ffi::c_void, CHANGEKIND, *mut core::ffi::c_void, super::LPOLESTR, *mut super::INT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    RequestTypeChange: usize,
    #[cfg(feature = "wtypesbase")]
    pub AfterTypeChange: unsafe extern "system" fn(*mut core::ffi::c_void, CHANGEKIND, *mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    AfterTypeChange: usize,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait ITypeChangeEvents_Impl: windows_core::IUnknownImpl {
    fn RequestTypeChange(&self, changekind: CHANGEKIND, ptinfobefore: windows_core::Ref<ITypeInfo>, pstrname: super::LPOLESTR) -> windows_core::Result<super::INT>;
    fn AfterTypeChange(&self, changekind: CHANGEKIND, ptinfoafter: windows_core::Ref<ITypeInfo>, pstrname: super::LPOLESTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl ITypeChangeEvents_Vtbl {
    pub const fn new<Identity: ITypeChangeEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestTypeChange<Identity: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: *mut core::ffi::c_void, pstrname: super::LPOLESTR, pfcancel: *mut super::INT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeChangeEvents_Impl::RequestTypeChange(this, core::mem::transmute_copy(&changekind), core::mem::transmute_copy(&ptinfobefore), core::mem::transmute_copy(&pstrname)) {
                    Ok(ok__) => {
                        pfcancel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AfterTypeChange<Identity: ITypeChangeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: *mut core::ffi::c_void, pstrname: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeChangeEvents_Impl::AfterTypeChange(this, core::mem::transmute_copy(&changekind), core::mem::transmute_copy(&ptinfoafter), core::mem::transmute_copy(&pstrname)).into()
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITypeChangeEvents {}
windows_core::imp::define_interface!(ITypeComp, ITypeComp_Vtbl, 0x00020403_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeComp, windows_core::IUnknown);
impl ITypeComp {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Bind(&self, szname: super::LPOLESTR, lhashval: super::ULONG, wflags: super::WORD, pptinfo: *mut Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), szname, lhashval, wflags, core::mem::transmute(pptinfo), pdesckind as _, pbindptr) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn BindType(&self, szname: super::LPOLESTR, lhashval: super::ULONG, pptinfo: *mut Option<ITypeInfo>, pptcomp: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindType)(windows_core::Interface::as_raw(self), szname, lhashval, core::mem::transmute(pptinfo), core::mem::transmute(pptcomp)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeComp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, super::ULONG, super::WORD, *mut *mut core::ffi::c_void, *mut DESCKIND, *mut BINDPTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Bind: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub BindType: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, super::ULONG, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    BindType: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeComp_Impl: windows_core::IUnknownImpl {
    fn Bind(&self, szname: super::LPOLESTR, lhashval: super::ULONG, wflags: super::WORD, pptinfo: windows_core::OutRef<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::Result<()>;
    fn BindType(&self, szname: super::LPOLESTR, lhashval: super::ULONG, pptinfo: windows_core::OutRef<ITypeInfo>, pptcomp: windows_core::OutRef<ITypeComp>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITypeComp_Vtbl {
    pub const fn new<Identity: ITypeComp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Bind<Identity: ITypeComp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: super::LPOLESTR, lhashval: super::ULONG, wflags: super::WORD, pptinfo: *mut *mut core::ffi::c_void, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeComp_Impl::Bind(this, core::mem::transmute_copy(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pdesckind), core::mem::transmute_copy(&pbindptr)).into()
            }
        }
        unsafe extern "system" fn BindType<Identity: ITypeComp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: super::LPOLESTR, lhashval: super::ULONG, pptinfo: *mut *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeComp_Impl::BindType(this, core::mem::transmute_copy(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pptcomp)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, OFFSET>, BindType: BindType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeComp as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITypeComp {}
windows_core::imp::define_interface!(ITypeFactory, ITypeFactory_Vtbl, 0x0000002e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeFactory, windows_core::IUnknown);
impl ITypeFactory {
    #[cfg(feature = "guiddef")]
    pub unsafe fn CreateFromTypeInfo<P0>(&self, ptypeinfo: P0, riid: *const super::IID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<ITypeInfo>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFromTypeInfo)(windows_core::Interface::as_raw(self), ptypeinfo.param().abi(), riid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub CreateFromTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    CreateFromTypeInfo: usize,
}
#[cfg(feature = "guiddef")]
pub trait ITypeFactory_Impl: windows_core::IUnknownImpl {
    fn CreateFromTypeInfo(&self, ptypeinfo: windows_core::Ref<ITypeInfo>, riid: *const super::IID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "guiddef")]
impl ITypeFactory_Vtbl {
    pub const fn new<Identity: ITypeFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromTypeInfo<Identity: ITypeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeinfo: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeFactory_Impl::CreateFromTypeInfo(this, core::mem::transmute_copy(&ptypeinfo), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateFromTypeInfo: CreateFromTypeInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for ITypeFactory {}
windows_core::imp::define_interface!(ITypeInfo, ITypeInfo_Vtbl, 0x00020401_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeInfo, windows_core::IUnknown);
impl ITypeInfo {
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFuncDesc(&self, index: super::UINT) -> windows_core::Result<*mut FUNCDESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetVarDesc(&self, index: super::UINT) -> windows_core::Result<*mut VARDESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetNames(&self, memid: super::MEMBERID, rgbstrnames: *mut windows_core::BSTR, cmaxnames: super::UINT, pcnames: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), memid, core::mem::transmute(rgbstrnames), cmaxnames, pcnames as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRefTypeOfImplType(&self, index: super::UINT) -> windows_core::Result<HREFTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRefTypeOfImplType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetImplTypeFlags(&self, index: super::UINT) -> windows_core::Result<super::INT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImplTypeFlags)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn GetIDsOfNames(&self, rgsznames: &[super::LPOLESTR]) -> windows_core::Result<super::MEMBERID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), rgsznames.as_ptr(), rgsznames.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Invoke(&self, pvinstance: super::PVOID, memid: super::MEMBERID, wflags: super::WORD, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pvinstance, memid, wflags, pdispparams as _, pvarresult, pexcepinfo, puargerr as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDocumentation(&self, memid: super::MEMBERID, pbstrname: Option<*mut windows_core::BSTR>, pbstrdocstring: Option<*mut windows_core::BSTR>, pdwhelpcontext: *mut super::DWORD, pbstrhelpfile: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), memid, pbstrname.unwrap_or(core::mem::zeroed()) as _, pbstrdocstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpcontext as _, pbstrhelpfile.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDllEntry(&self, memid: super::MEMBERID, invkind: INVOKEKIND, pbstrdllname: Option<*mut windows_core::BSTR>, pbstrname: Option<*mut windows_core::BSTR>, pwordinal: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDllEntry)(windows_core::Interface::as_raw(self), memid, invkind, pbstrdllname.unwrap_or(core::mem::zeroed()) as _, pbstrname.unwrap_or(core::mem::zeroed()) as _, pwordinal as _) }
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: HREFTYPE) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRefTypeInfo)(windows_core::Interface::as_raw(self), hreftype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn AddressOfMember(&self, memid: super::MEMBERID, invkind: INVOKEKIND, ppv: *mut super::PVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddressOfMember)(windows_core::Interface::as_raw(self), memid, invkind, ppv as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub unsafe fn CreateInstance<P0>(&self, punkouter: P0, riid: *const super::IID, ppvobj: *mut super::PVOID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, ppvobj as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMops(&self, memid: super::MEMBERID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMops)(windows_core::Interface::as_raw(self), memid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut Option<ITypeLib>, pindex: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContainingTypeLib)(windows_core::Interface::as_raw(self), core::mem::transmute(pptlib), pindex as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseTypeAttr)(windows_core::Interface::as_raw(self), ptypeattr);
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseFuncDesc)(windows_core::Interface::as_raw(self), pfuncdesc);
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub GetTypeAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TYPEATTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    GetTypeAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut FUNCDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetFuncDesc: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut VARDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetVarDesc: usize,
    #[cfg(feature = "shared")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, *mut *mut core::ffi::c_void, super::UINT, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNames: usize,
    #[cfg(feature = "shared")]
    pub GetRefTypeOfImplType: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut HREFTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRefTypeOfImplType: usize,
    #[cfg(feature = "shared")]
    pub GetImplTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::INT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetImplTypeFlags: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::LPOLESTR, super::UINT, *mut super::MEMBERID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    GetIDsOfNames: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::MEMBERID, super::WORD, *mut DISPPARAMS, *mut VARIANT, *mut EXCEPINFO, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Invoke: usize,
    #[cfg(feature = "shared")]
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDocumentation: usize,
    #[cfg(feature = "shared")]
    pub GetDllEntry: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, INVOKEKIND, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDllEntry: usize,
    pub GetRefTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, HREFTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub AddressOfMember: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, INVOKEKIND, *mut super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    AddressOfMember: usize,
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "winnt")))]
    CreateInstance: usize,
    #[cfg(feature = "shared")]
    pub GetMops: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMops: usize,
    #[cfg(feature = "shared")]
    pub GetContainingTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetContainingTypeLib: usize,
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub ReleaseTypeAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TYPEATTR),
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    ReleaseTypeAttr: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ReleaseFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const FUNCDESC),
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ReleaseFuncDesc: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ReleaseVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const VARDESC),
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ReleaseVarDesc: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeInfo_Impl: windows_core::IUnknownImpl {
    fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetFuncDesc(&self, index: super::UINT) -> windows_core::Result<*mut FUNCDESC>;
    fn GetVarDesc(&self, index: super::UINT) -> windows_core::Result<*mut VARDESC>;
    fn GetNames(&self, memid: super::MEMBERID, rgbstrnames: *mut windows_core::BSTR, cmaxnames: super::UINT, pcnames: *mut super::UINT) -> windows_core::Result<()>;
    fn GetRefTypeOfImplType(&self, index: super::UINT) -> windows_core::Result<HREFTYPE>;
    fn GetImplTypeFlags(&self, index: super::UINT) -> windows_core::Result<super::INT>;
    fn GetIDsOfNames(&self, rgsznames: *const super::LPOLESTR, cnames: super::UINT) -> windows_core::Result<super::MEMBERID>;
    fn Invoke(&self, pvinstance: super::PVOID, memid: super::MEMBERID, wflags: super::WORD, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::Result<()>;
    fn GetDocumentation(&self, memid: super::MEMBERID, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut super::DWORD, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDllEntry(&self, memid: super::MEMBERID, invkind: INVOKEKIND, pbstrdllname: *mut windows_core::BSTR, pbstrname: *mut windows_core::BSTR, pwordinal: *mut super::WORD) -> windows_core::Result<()>;
    fn GetRefTypeInfo(&self, hreftype: HREFTYPE) -> windows_core::Result<ITypeInfo>;
    fn AddressOfMember(&self, memid: super::MEMBERID, invkind: INVOKEKIND, ppv: *mut super::PVOID) -> windows_core::Result<()>;
    fn CreateInstance(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const super::IID, ppvobj: *mut super::PVOID) -> windows_core::Result<()>;
    fn GetMops(&self, memid: super::MEMBERID) -> windows_core::Result<windows_core::BSTR>;
    fn GetContainingTypeLib(&self, pptlib: windows_core::OutRef<ITypeLib>, pindex: *mut super::UINT) -> windows_core::Result<()>;
    fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR);
    fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC);
    fn ReleaseVarDesc(&self, pvardesc: *const VARDESC);
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetFuncDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, ppfuncdesc: *mut *mut FUNCDESC) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetVarDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, ppvardesc: *mut *mut VARDESC) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetNames<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, rgbstrnames: *mut *mut core::ffi::c_void, cmaxnames: super::UINT, pcnames: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetNames(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&rgbstrnames), core::mem::transmute_copy(&cmaxnames), core::mem::transmute_copy(&pcnames)).into()
            }
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, preftype: *mut HREFTYPE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetImplTypeFlags<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pimpltypeflags: *mut super::INT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetIDsOfNames<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgsznames: *const super::LPOLESTR, cnames: super::UINT, pmemid: *mut super::MEMBERID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Invoke<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvinstance: super::PVOID, memid: super::MEMBERID, wflags: super::WORD, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::Invoke(this, core::mem::transmute_copy(&pvinstance), core::mem::transmute_copy(&memid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        unsafe extern "system" fn GetDocumentation<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, pbstrname: *mut *mut core::ffi::c_void, pbstrdocstring: *mut *mut core::ffi::c_void, pdwhelpcontext: *mut super::DWORD, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetDocumentation(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
            }
        }
        unsafe extern "system" fn GetDllEntry<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, invkind: INVOKEKIND, pbstrdllname: *mut *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void, pwordinal: *mut super::WORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddressOfMember<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, invkind: INVOKEKIND, ppv: *mut super::PVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::AddressOfMember(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const super::IID, ppvobj: *mut super::PVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::CreateInstance(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn GetMops<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, pbstrmops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetContainingTypeLib<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlib: *mut *mut core::ffi::c_void, pindex: *mut super::UINT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetTypeFlags(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: super::MEMBERID, invkind: INVOKEKIND) -> windows_core::Result<super::UINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncIndexOfMemId)(windows_core::Interface::as_raw(self), memid, invkind, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetVarIndexOfMemId(&self, memid: super::MEMBERID) -> windows_core::Result<super::UINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarIndexOfMemId)(windows_core::Interface::as_raw(self), memid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFuncCustData(&self, index: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetParamCustData(&self, indexfunc: super::UINT, indexparam: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetVarCustData(&self, index: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetImplTypeCustData(&self, index: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImplTypeCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDocumentation2(&self, memid: super::MEMBERID, lcid: super::LCID, pbstrhelpstring: Option<*mut windows_core::BSTR>, pdwhelpstringcontext: *mut super::DWORD, pbstrhelpstringdll: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), memid, lcid, pbstrhelpstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpstringcontext as _, pbstrhelpstringdll.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllCustData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllFuncCustData(&self, index: super::UINT) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllFuncCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: super::UINT, indexparam: super::UINT) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllVarCustData(&self, index: super::UINT) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllVarCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllImplTypeCustData(&self, index: super::UINT) -> windows_core::Result<CUSTDATA> {
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
    #[cfg(feature = "shared")]
    pub GetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTypeFlags: usize,
    #[cfg(feature = "shared")]
    pub GetFuncIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, INVOKEKIND, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFuncIndexOfMemId: usize,
    #[cfg(feature = "shared")]
    pub GetVarIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVarIndexOfMemId: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetFuncCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetParamCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetVarCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetImplTypeCustData: usize,
    #[cfg(feature = "shared")]
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, super::MEMBERID, super::LCID, *mut *mut core::ffi::c_void, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDocumentation2: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAllCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAllFuncCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAllParamCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAllVarCustData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAllImplTypeCustData: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeInfo2_Impl: ITypeInfo_Impl {
    fn GetTypeKind(&self) -> windows_core::Result<TYPEKIND>;
    fn GetTypeFlags(&self) -> windows_core::Result<super::ULONG>;
    fn GetFuncIndexOfMemId(&self, memid: super::MEMBERID, invkind: INVOKEKIND) -> windows_core::Result<super::UINT>;
    fn GetVarIndexOfMemId(&self, memid: super::MEMBERID) -> windows_core::Result<super::UINT>;
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetFuncCustData(&self, index: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetParamCustData(&self, indexfunc: super::UINT, indexparam: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetVarCustData(&self, index: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetImplTypeCustData(&self, index: super::UINT, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetDocumentation2(&self, memid: super::MEMBERID, lcid: super::LCID, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut super::DWORD, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
    fn GetAllFuncCustData(&self, index: super::UINT) -> windows_core::Result<CUSTDATA>;
    fn GetAllParamCustData(&self, indexfunc: super::UINT, indexparam: super::UINT) -> windows_core::Result<CUSTDATA>;
    fn GetAllVarCustData(&self, index: super::UINT) -> windows_core::Result<CUSTDATA>;
    fn GetAllImplTypeCustData(&self, index: super::UINT) -> windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetTypeFlags<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeflags: *mut super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetFuncIndexOfMemId<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, invkind: INVOKEKIND, pfuncindex: *mut super::UINT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetVarIndexOfMemId<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, pvarindex: *mut super::UINT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetFuncCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetParamCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: super::UINT, indexparam: super::UINT, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetVarCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetImplTypeCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, guid: *const windows_core::GUID, pvarval: *mut VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDocumentation2<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: super::MEMBERID, lcid: super::LCID, pbstrhelpstring: *mut *mut core::ffi::c_void, pdwhelpstringcontext: *mut super::DWORD, pbstrhelpstringdll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetAllFuncCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetAllParamCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: super::UINT, indexparam: super::UINT, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetAllVarCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetAllImplTypeCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITypeInfo2 {}
windows_core::imp::define_interface!(ITypeLib, ITypeLib_Vtbl, 0x00020402_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeLib, windows_core::IUnknown);
impl ITypeLib {
    #[cfg(feature = "shared")]
    pub unsafe fn GetTypeInfoCount(&self) -> super::UINT {
        unsafe { (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTypeInfo(&self, index: super::UINT) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTypeInfoType(&self, index: super::UINT) -> windows_core::Result<TYPEKIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoOfGuid)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLibAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDocumentation(&self, index: super::INT, pbstrname: Option<*mut windows_core::BSTR>, pbstrdocstring: Option<*mut windows_core::BSTR>, pdwhelpcontext: *mut super::DWORD, pbstrhelpfile: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), index, pbstrname.unwrap_or(core::mem::zeroed()) as _, pbstrdocstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpcontext as _, pbstrhelpfile.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn IsName(&self, sznamebuf: super::LPOLESTR, lhashval: super::ULONG, pfname: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsName)(windows_core::Interface::as_raw(self), sznamebuf as _, lhashval, pfname as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn FindName(&self, sznamebuf: super::LPOLESTR, lhashval: super::ULONG, pptinfo: *mut Option<ITypeInfo>, rgmemid: *mut super::MEMBERID, pcfound: *mut super::USHORT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindName)(windows_core::Interface::as_raw(self), sznamebuf as _, lhashval, core::mem::transmute(pptinfo), rgmemid as _, pcfound as _) }
    }
    #[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::UINT,
    #[cfg(not(feature = "shared"))]
    GetTypeInfoCount: usize,
    #[cfg(feature = "shared")]
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTypeInfo: usize,
    #[cfg(feature = "shared")]
    pub GetTypeInfoType: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut TYPEKIND) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTypeInfoType: usize,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TLIBATTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLibAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, super::INT, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDocumentation: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub IsName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, super::ULONG, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    IsName: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub FindName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, super::ULONG, *mut *mut core::ffi::c_void, *mut super::MEMBERID, *mut super::USHORT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    FindName: usize,
    #[cfg(feature = "shared")]
    pub ReleaseTLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TLIBATTR),
    #[cfg(not(feature = "shared"))]
    ReleaseTLibAttr: usize,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait ITypeLib_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> super::UINT;
    fn GetTypeInfo(&self, index: super::UINT) -> windows_core::Result<ITypeInfo>;
    fn GetTypeInfoType(&self, index: super::UINT) -> windows_core::Result<TYPEKIND>;
    fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo>;
    fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetDocumentation(&self, index: super::INT, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut super::DWORD, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsName(&self, sznamebuf: super::LPOLESTR, lhashval: super::ULONG, pfname: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn FindName(&self, sznamebuf: super::LPOLESTR, lhashval: super::ULONG, pptinfo: windows_core::OutRef<ITypeInfo>, rgmemid: *mut super::MEMBERID, pcfound: *mut super::USHORT) -> windows_core::Result<()>;
    fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR);
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl ITypeLib_Vtbl {
    pub const fn new<Identity: ITypeLib_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::UINT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::GetTypeInfoCount(this)
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetTypeInfoType<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, ptkind: *mut TYPEKIND) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDocumentation<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::INT, pbstrname: *mut *mut core::ffi::c_void, pbstrdocstring: *mut *mut core::ffi::c_void, pdwhelpcontext: *mut super::DWORD, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::GetDocumentation(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
            }
        }
        unsafe extern "system" fn IsName<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: super::LPOLESTR, lhashval: super::ULONG, pfname: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::IsName(this, core::mem::transmute_copy(&sznamebuf), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pfname)).into()
            }
        }
        unsafe extern "system" fn FindName<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: super::LPOLESTR, lhashval: super::ULONG, pptinfo: *mut *mut core::ffi::c_void, rgmemid: *mut super::MEMBERID, pcfound: *mut super::USHORT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut super::ULONG, pcchuniquenames: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLibStatistics)(windows_core::Interface::as_raw(self), pcuniquenames as _, pcchuniquenames as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDocumentation2(&self, index: super::INT, lcid: super::LCID, pbstrhelpstring: Option<*mut windows_core::BSTR>, pdwhelpstringcontext: *mut super::DWORD, pbstrhelpstringdll: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), index, lcid, pbstrhelpstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpstringcontext as _, pbstrhelpstringdll.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCustData: usize,
    #[cfg(feature = "shared")]
    pub GetLibStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLibStatistics: usize,
    #[cfg(feature = "shared")]
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, super::INT, super::LCID, *mut *mut core::ffi::c_void, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDocumentation2: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAllCustData: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITypeLib2_Impl: ITypeLib_Impl {
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<VARIANT>;
    fn GetLibStatistics(&self, pcuniquenames: *mut super::ULONG, pcchuniquenames: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetDocumentation2(&self, index: super::INT, lcid: super::LCID, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut super::DWORD, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetLibStatistics<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcuniquenames: *mut super::ULONG, pcchuniquenames: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib2_Impl::GetLibStatistics(this, core::mem::transmute_copy(&pcuniquenames), core::mem::transmute_copy(&pcchuniquenames)).into()
            }
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::INT, lcid: super::LCID, pbstrhelpstring: *mut *mut core::ffi::c_void, pdwhelpstringcontext: *mut super::DWORD, pbstrhelpstringdll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetLcid(&self) -> windows_core::Result<super::LCID> {
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetFlags(&self) -> windows_core::Result<super::DWORD> {
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
    #[cfg(feature = "shared")]
    pub GetLcid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLcid: usize,
    pub GetWin32Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWin64Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFlags: usize,
    pub GetHelpDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ITypeLibRegistration_Impl: windows_core::IUnknownImpl {
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcid(&self) -> windows_core::Result<super::LCID>;
    fn GetWin32Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetWin64Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFlags(&self) -> windows_core::Result<super::DWORD>;
    fn GetHelpDir(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "shared")]
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
        unsafe extern "system" fn GetLcid<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut super::LCID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetFlags<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITypeLibRegistration {}
windows_core::imp::define_interface!(ITypeLibRegistrationReader, ITypeLibRegistrationReader_Vtbl, 0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
windows_core::imp::interface_hierarchy!(ITypeLibRegistrationReader, windows_core::IUnknown);
impl ITypeLibRegistrationReader {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn EnumTypeLibRegistrations(&self) -> windows_core::Result<super::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumTypeLibRegistrations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    fn EnumTypeLibRegistrations(&self) -> windows_core::Result<super::IEnumUnknown>;
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
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Size(&self, pvtype: super::PVOID, dwdestcontext: super::DWORD, pvdestcontext: super::PVOID) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), pvtype, dwdestcontext, pvdestcontext, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Marshal(&self, pvtype: super::PVOID, dwdestcontext: super::DWORD, pvdestcontext: super::PVOID, cbbufferlength: super::ULONG, pbuffer: *mut super::BYTE, pcbwritten: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Marshal)(windows_core::Interface::as_raw(self), pvtype, dwdestcontext, pvdestcontext, cbbufferlength, pbuffer as _, pcbwritten as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Unmarshal(&self, pvtype: super::PVOID, dwflags: super::DWORD, pbuffer: &[super::BYTE], pcbread: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unmarshal)(windows_core::Interface::as_raw(self), pvtype as _, dwflags, pbuffer.len().try_into().unwrap(), pbuffer.as_ptr(), pcbread as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn Free(&self, pvtype: super::PVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pvtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::DWORD, super::PVOID, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Size: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Marshal: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::DWORD, super::PVOID, super::ULONG, *mut super::BYTE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Marshal: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Unmarshal: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::DWORD, super::ULONG, *const super::BYTE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Unmarshal: usize,
    #[cfg(feature = "winnt")]
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Free: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait ITypeMarshal_Impl: windows_core::IUnknownImpl {
    fn Size(&self, pvtype: super::PVOID, dwdestcontext: super::DWORD, pvdestcontext: super::PVOID) -> windows_core::Result<super::ULONG>;
    fn Marshal(&self, pvtype: super::PVOID, dwdestcontext: super::DWORD, pvdestcontext: super::PVOID, cbbufferlength: super::ULONG, pbuffer: *mut super::BYTE, pcbwritten: *mut super::ULONG) -> windows_core::Result<()>;
    fn Unmarshal(&self, pvtype: super::PVOID, dwflags: super::DWORD, cbbufferlength: super::ULONG, pbuffer: *const super::BYTE, pcbread: *mut super::ULONG) -> windows_core::Result<()>;
    fn Free(&self, pvtype: super::PVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl ITypeMarshal_Vtbl {
    pub const fn new<Identity: ITypeMarshal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Size<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: super::PVOID, dwdestcontext: super::DWORD, pvdestcontext: super::PVOID, psize: *mut super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Marshal<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: super::PVOID, dwdestcontext: super::DWORD, pvdestcontext: super::PVOID, cbbufferlength: super::ULONG, pbuffer: *mut super::BYTE, pcbwritten: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeMarshal_Impl::Marshal(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&cbbufferlength), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Unmarshal<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: super::PVOID, dwflags: super::DWORD, cbbufferlength: super::ULONG, pbuffer: *const super::BYTE, pcbread: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeMarshal_Impl::Unmarshal(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cbbufferlength), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn Free<Identity: ITypeMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: super::PVOID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ITypeMarshal {}
pub type LIBFLAGS = i32;
pub const LIBFLAG_FCONTROL: LIBFLAGS = 2;
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = 8;
pub const LIBFLAG_FHIDDEN: LIBFLAGS = 4;
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = 1;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPBINDPTR = *mut BINDPTR;
pub type LPCREATEERRORINFO = ICreateErrorInfo;
pub type LPCREATETYPEINFO = ICreateTypeInfo;
pub type LPCREATETYPEINFO2 = ICreateTypeInfo2;
pub type LPCREATETYPELIB = ICreateTypeLib;
pub type LPCREATETYPELIB2 = ICreateTypeLib2;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPCUSTDATA = *mut CUSTDATA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPCUSTDATAITEM = *mut CUSTDATAITEM;
pub type LPDISPATCH = IDispatch;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPELEMDESC = *mut ELEMDESC;
pub type LPENUMVARIANT = IEnumVARIANT;
pub type LPERRORINFO = IErrorInfo;
pub type LPERRORLOG = IErrorLog;
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub type LPEXCEPINFO = *mut EXCEPINFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPFUNCDESC = *mut FUNCDESC;
#[cfg(feature = "shared")]
pub type LPIDLDESC = *mut IDLDESC;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDESC = *mut PARAMDESC;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDESCEX = *mut PARAMDESCEX;
pub type LPPROPERTYBAG = IPropertyBag;
pub type LPRECORDINFO = IRecordInfo;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSAFEARRAY = *mut SAFEARRAY;
#[cfg(feature = "shared")]
pub type LPSAFEARRAYBOUND = *mut SAFEARRAYBOUND;
pub type LPSUPPORTERRORINFO = ISupportErrorInfo;
#[cfg(feature = "shared")]
pub type LPTLIBATTR = *mut TLIBATTR;
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type LPTYPEATTR = *mut TYPEATTR;
pub type LPTYPECHANGEEVENTS = ITypeChangeEvents;
pub type LPTYPECOMP = ITypeComp;
pub type LPTYPEINFO = ITypeInfo;
pub type LPTYPEINFO2 = ITypeInfo2;
pub type LPTYPELIB = ITypeLib;
pub type LPTYPELIB2 = ITypeLib2;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARDESC = *mut VARDESC;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARIANT = *mut tagVARIANT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARIANTARG = *mut tagVARIANT;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct PARAMDESC {
    pub pparamdescex: LPPARAMDESCEX,
    pub wParamFlags: super::USHORT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct PARAMDESCEX {
    pub cBytes: super::ULONG,
    pub varDefaultValue: VARIANTARG,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PARAMFLAG_FHASCUSTDATA: i32 = 64;
pub const PARAMFLAG_FHASDEFAULT: i32 = 32;
pub const PARAMFLAG_FIN: i32 = 1;
pub const PARAMFLAG_FLCID: i32 = 4;
pub const PARAMFLAG_FOPT: i32 = 16;
pub const PARAMFLAG_FOUT: i32 = 2;
pub const PARAMFLAG_FRETVAL: i32 = 8;
pub const PARAMFLAG_NONE: i32 = 0;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SAFEARRAY {
    pub cDims: super::USHORT,
    pub fFeatures: super::USHORT,
    pub cbElements: super::ULONG,
    pub cLocks: super::ULONG,
    pub pvData: super::PVOID,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SAFEARRAYBOUND {
    pub cElements: super::ULONG,
    pub lLbound: super::LONG,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SAFEARRAYUNION {
    pub sfType: super::ULONG,
    pub u: SAFEARRAYUNION_0,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union SAFEARRAYUNION_0 {
    pub BstrStr: SAFEARR_BSTR,
    pub UnknownStr: SAFEARR_UNKNOWN,
    pub DispatchStr: SAFEARR_DISPATCH,
    pub VariantStr: SAFEARR_VARIANT,
    pub RecordStr: SAFEARR_BRECORD,
    pub HaveIidStr: SAFEARR_HAVEIID,
    pub ByteStr: super::BYTE_SIZEDARR,
    pub WordStr: super::WORD_SIZEDARR,
    pub LongStr: super::DWORD_SIZEDARR,
    pub HyperStr: super::HYPER_SIZEDARR,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARRAYUNION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_BRECORD {
    pub Size: super::ULONG,
    pub aRecord: *mut wireBRECORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_BSTR {
    pub Size: super::ULONG,
    pub aBstr: *mut super::wireBSTR,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_DISPATCH {
    pub Size: super::ULONG,
    pub apDispatch: *mut Option<IDispatch>,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_HAVEIID {
    pub Size: super::ULONG,
    pub apUnknown: *mut Option<windows_core::IUnknown>,
    pub iid: super::IID,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_UNKNOWN {
    pub Size: super::ULONG,
    pub apUnknown: *mut Option<windows_core::IUnknown>,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_VARIANT {
    pub Size: super::ULONG,
    pub aVariant: *mut wireVARIANT,
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
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TLIBATTR {
    pub guid: windows_core::GUID,
    pub lcid: super::LCID,
    pub syskind: SYSKIND,
    pub wMajorVerNum: super::WORD,
    pub wMinorVerNum: super::WORD,
    pub wLibFlags: super::WORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct TYPEATTR {
    pub guid: windows_core::GUID,
    pub lcid: super::LCID,
    pub dwReserved: super::DWORD,
    pub memidConstructor: super::MEMBERID,
    pub memidDestructor: super::MEMBERID,
    pub lpstrSchema: super::LPOLESTR,
    pub cbSizeInstance: super::ULONG,
    pub typekind: TYPEKIND,
    pub cFuncs: super::WORD,
    pub cVars: super::WORD,
    pub cImplTypes: super::WORD,
    pub cbSizeVft: super::WORD,
    pub cbAlignment: super::WORD,
    pub wTypeFlags: super::WORD,
    pub wMajorVerNum: super::WORD,
    pub wMinorVerNum: super::WORD,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TYPEATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: super::VARTYPE,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl Default for TYPEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: HREFTYPE,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
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
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct VARDESC {
    pub memid: super::MEMBERID,
    pub lpstrSchema: super::LPOLESTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: super::WORD,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union VARDESC_0 {
    pub oInst: super::ULONG,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type VARIANT = tagVARIANT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type VARIANTARG = tagVARIANT;
pub type VARKIND = i32;
pub const VAR_CONST: VARKIND = 2;
pub const VAR_DISPATCH: VARKIND = 3;
pub const VAR_PERINSTANCE: VARKIND = 0;
pub const VAR_STATIC: VARKIND = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Default)]
pub struct _wireBRECORD {
    pub fFlags: super::ULONG,
    pub clSize: super::ULONG,
    pub pRecInfo: core::mem::ManuallyDrop<Option<IRecordInfo>>,
    pub pRecord: *mut super::byte,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct _wireSAFEARRAY {
    pub cDims: super::USHORT,
    pub fFeatures: super::USHORT,
    pub cbElements: super::ULONG,
    pub cLocks: super::ULONG,
    pub uArrayStructs: SAFEARRAYUNION,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub struct _wireVARIANT {
    pub clSize: super::DWORD,
    pub rpcReserved: super::DWORD,
    pub vt: super::USHORT,
    pub wReserved1: super::USHORT,
    pub wReserved2: super::USHORT,
    pub wReserved3: super::USHORT,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for _wireVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub union _wireVARIANT_0 {
    pub llVal: super::LONGLONG,
    pub lVal: super::LONG,
    pub bVal: super::BYTE,
    pub iVal: super::SHORT,
    pub fltVal: super::FLOAT,
    pub dblVal: super::DOUBLE,
    pub boolVal: super::VARIANT_BOOL,
    pub scode: super::SCODE,
    pub cyVal: super::CY,
    pub date: super::DATE,
    pub bstrVal: super::wireBSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub parray: wirePSAFEARRAY,
    pub brecVal: wireBRECORD,
    pub pbVal: *mut super::BYTE,
    pub piVal: *mut super::SHORT,
    pub plVal: *mut super::LONG,
    pub pllVal: *mut super::LONGLONG,
    pub pfltVal: *mut super::FLOAT,
    pub pdblVal: *mut super::DOUBLE,
    pub pboolVal: *mut super::VARIANT_BOOL,
    pub pscode: *mut super::SCODE,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut super::DATE,
    pub pbstrVal: *mut super::wireBSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut wirePSAFEARRAY,
    pub pvarVal: *mut wireVARIANT,
    pub cVal: super::CHAR,
    pub uiVal: super::USHORT,
    pub ulVal: super::ULONG,
    pub ullVal: super::ULONGLONG,
    pub intVal: super::INT,
    pub uintVal: super::UINT,
    pub decVal: super::DECIMAL,
    pub pdecVal: *mut super::DECIMAL,
    pub pcVal: *mut super::CHAR,
    pub puiVal: *mut super::USHORT,
    pub pulVal: *mut super::ULONG,
    pub pullVal: *mut super::ULONGLONG,
    pub pintVal: *mut super::INT,
    pub puintVal: *mut super::UINT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for _wireVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct tagVARIANT {
    pub Anonymous: tagVARIANT_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub union tagVARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<tagVARIANT_0_0>,
    pub decVal: super::DECIMAL,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct tagVARIANT_0_0 {
    pub vt: super::VARTYPE,
    pub wReserved1: super::WORD,
    pub wReserved2: super::WORD,
    pub wReserved3: super::WORD,
    pub Anonymous: tagVARIANT_0_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub union tagVARIANT_0_0_0 {
    pub llVal: super::LONGLONG,
    pub lVal: super::LONG,
    pub bVal: super::BYTE,
    pub iVal: super::SHORT,
    pub fltVal: super::FLOAT,
    pub dblVal: super::DOUBLE,
    pub boolVal: super::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::VARIANT_BOOL,
    pub scode: super::SCODE,
    pub cyVal: super::CY,
    pub date: super::DATE,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut super::BYTE,
    pub piVal: *mut super::SHORT,
    pub plVal: *mut super::LONG,
    pub pllVal: *mut super::LONGLONG,
    pub pfltVal: *mut super::FLOAT,
    pub pdblVal: *mut super::DOUBLE,
    pub pboolVal: *mut super::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::VARIANT_BOOL,
    pub pscode: *mut super::SCODE,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut super::DATE,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: super::PVOID,
    pub cVal: super::CHAR,
    pub uiVal: super::USHORT,
    pub ulVal: super::ULONG,
    pub ullVal: super::ULONGLONG,
    pub intVal: super::INT,
    pub uintVal: super::UINT,
    pub pdecVal: *mut super::DECIMAL,
    pub pcVal: *mut super::CHAR,
    pub puiVal: *mut super::USHORT,
    pub pulVal: *mut super::ULONG,
    pub pullVal: *mut super::ULONGLONG,
    pub pintVal: *mut super::INT,
    pub puintVal: *mut super::UINT,
    pub Anonymous: core::mem::ManuallyDrop<tagVARIANT_0_0_0_0>,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Default)]
pub struct tagVARIANT_0_0_0_0 {
    pub pvRecord: super::PVOID,
    pub pRecInfo: core::mem::ManuallyDrop<Option<IRecordInfo>>,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub type wireBRECORD = *mut _wireBRECORD;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type wirePSAFEARRAY = *mut *mut _wireSAFEARRAY;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type wireSAFEARRAY = *mut _wireSAFEARRAY;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type wireVARIANT = *mut _wireVARIANT;
