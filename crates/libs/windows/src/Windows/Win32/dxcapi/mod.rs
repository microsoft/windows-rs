#[cfg(all(feature = "guiddef", feature = "minwindef"))]
#[inline]
pub unsafe fn DxcCreateInstance(rclsid: *const super::IID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("dxcompiler.dll" "system" fn DxcCreateInstance(rclsid : *const super::IID, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { DxcCreateInstance(rclsid, riid, ppv as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
#[inline]
pub unsafe fn DxcCreateInstance2<P0>(pmalloc: P0, rclsid: *const super::IID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IMalloc>,
{
    windows_core::link!("dxcompiler.dll" "system" fn DxcCreateInstance2(pmalloc : *mut core::ffi::c_void, rclsid : *const super::IID, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { DxcCreateInstance2(pmalloc.param().abi(), rclsid, riid, ppv as _) }
}
pub const CLSID_DxcAssembler: windows_core::GUID = windows_core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: windows_core::GUID = windows_core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: windows_core::GUID = windows_core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: windows_core::GUID = windows_core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: windows_core::GUID = windows_core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: windows_core::GUID = windows_core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: windows_core::GUID = windows_core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: windows_core::GUID = windows_core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: windows_core::GUID = windows_core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcValidator: windows_core::GUID = windows_core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
pub const DXC_ARG_ALL_RESOURCES_BOUND: windows_core::PCWSTR = windows_core::w!("-all_resources_bound");
pub const DXC_ARG_AVOID_FLOW_CONTROL: windows_core::PCWSTR = windows_core::w!("-Gfa");
pub const DXC_ARG_DEBUG: windows_core::PCWSTR = windows_core::w!("-Zi");
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: windows_core::PCWSTR = windows_core::w!("-Zsb");
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: windows_core::PCWSTR = windows_core::w!("-Zss");
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: windows_core::PCWSTR = windows_core::w!("-Gec");
pub const DXC_ARG_ENABLE_STRICTNESS: windows_core::PCWSTR = windows_core::w!("-Ges");
pub const DXC_ARG_IEEE_STRICTNESS: windows_core::PCWSTR = windows_core::w!("-Gis");
pub const DXC_ARG_OPTIMIZATION_LEVEL0: windows_core::PCWSTR = windows_core::w!("-O0");
pub const DXC_ARG_OPTIMIZATION_LEVEL1: windows_core::PCWSTR = windows_core::w!("-O1");
pub const DXC_ARG_OPTIMIZATION_LEVEL2: windows_core::PCWSTR = windows_core::w!("-O2");
pub const DXC_ARG_OPTIMIZATION_LEVEL3: windows_core::PCWSTR = windows_core::w!("-O3");
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: windows_core::PCWSTR = windows_core::w!("-Zpc");
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: windows_core::PCWSTR = windows_core::w!("-Zpr");
pub const DXC_ARG_PREFER_FLOW_CONTROL: windows_core::PCWSTR = windows_core::w!("-Gfp");
pub const DXC_ARG_RESOURCES_MAY_ALIAS: windows_core::PCWSTR = windows_core::w!("-res_may_alias");
pub const DXC_ARG_SKIP_OPTIMIZATIONS: windows_core::PCWSTR = windows_core::w!("-Od");
pub const DXC_ARG_SKIP_VALIDATION: windows_core::PCWSTR = windows_core::w!("-Vd");
pub const DXC_ARG_WARNINGS_ARE_ERRORS: windows_core::PCWSTR = windows_core::w!("-WX");
pub const DXC_CP_ACP: i32 = 0;
pub const DXC_CP_UTF16: i32 = 1200;
pub const DXC_CP_UTF32: i32 = 12000;
pub const DXC_CP_UTF8: i32 = 65001;
pub const DXC_CP_WIDE: i32 = 1200;
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: windows_core::PCWSTR = windows_core::w!("*stderr*");
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: windows_core::PCWSTR = windows_core::w!("*stdout*");
pub const DXC_HASHFLAG_INCLUDES_SOURCE: i32 = 1;
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = 5;
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = 2;
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = 10;
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = -1;
pub const DXC_OUT_HLSL: DXC_OUT_KIND = 6;
pub type DXC_OUT_KIND = i32;
pub const DXC_OUT_LAST: DXC_OUT_KIND = 13;
pub const DXC_OUT_NONE: DXC_OUT_KIND = 0;
pub const DXC_OUT_NUM_ENUMS: DXC_OUT_KIND = 14;
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = 1;
pub const DXC_OUT_PDB: DXC_OUT_KIND = 3;
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = 8;
pub const DXC_OUT_REMARKS: DXC_OUT_KIND = 11;
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = 9;
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = 4;
pub const DXC_OUT_TEXT: DXC_OUT_KIND = 7;
pub const DXC_OUT_TIME_REPORT: DXC_OUT_KIND = 12;
pub const DXC_OUT_TIME_TRACE: DXC_OUT_KIND = 13;
#[cfg(feature = "shared")]
pub const DXC_PART_DXIL: super::UINT32 = 1279875140;
#[cfg(feature = "shared")]
pub const DXC_PART_INPUT_SIGNATURE: super::UINT32 = 826757961;
#[cfg(feature = "shared")]
pub const DXC_PART_OUTPUT_SIGNATURE: super::UINT32 = 826757967;
#[cfg(feature = "shared")]
pub const DXC_PART_PATCH_CONSTANT_SIGNATURE: super::UINT32 = 826757968;
#[cfg(feature = "shared")]
pub const DXC_PART_PDB: super::UINT32 = 1111772233;
#[cfg(feature = "shared")]
pub const DXC_PART_PDB_NAME: super::UINT32 = 1313098825;
#[cfg(feature = "shared")]
pub const DXC_PART_PRIVATE_DATA: super::UINT32 = 1447645776;
#[cfg(feature = "shared")]
pub const DXC_PART_REFLECTION_DATA: super::UINT32 = 1413567571;
#[cfg(feature = "shared")]
pub const DXC_PART_ROOT_SIGNATURE: super::UINT32 = 810767442;
#[cfg(feature = "shared")]
pub const DXC_PART_SHADER_HASH: super::UINT32 = 1213415752;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DxcArgPair {
    pub pName: *const super::WCHAR,
    pub pValue: *const super::WCHAR,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
pub type DxcCreateInstance2Proc = Option<unsafe extern "system" fn(pmalloc: windows_core::Ref<super::IMalloc>, rclsid: *const super::IID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT>;
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub type DxcCreateInstanceProc = Option<unsafe extern "system" fn(rclsid: *const super::IID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DxcDefine {
    pub Name: windows_core::PCWSTR,
    pub Value: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DxcShaderHash {
    pub Flags: super::UINT32,
    pub HashDigest: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DxcText {
    pub Ptr: super::LPCVOID,
    pub Size: super::SIZE_T,
    pub Encoding: super::UINT,
}
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_Default: super::UINT32 = 0;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_InPlaceEdit: super::UINT32 = 1;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_ModuleOnly: super::UINT32 = 4;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_RootSignatureOnly: super::UINT32 = 2;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_ValidMask: super::UINT32 = 7;
#[cfg(feature = "shared")]
pub const DxcVersionInfoFlags_Debug: super::UINT32 = 1;
#[cfg(feature = "shared")]
pub const DxcVersionInfoFlags_Internal: super::UINT32 = 2;
#[cfg(feature = "shared")]
pub const DxcVersionInfoFlags_None: super::UINT32 = 0;
windows_core::imp::define_interface!(IDxcAssembler, IDxcAssembler_Vtbl, 0x091f7a26_1c1f_4948_904b_e6e3a8a771d5);
windows_core::imp::interface_hierarchy!(IDxcAssembler, windows_core::IUnknown);
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<P0>(&self, pshader: P0) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssembleToContainer)(windows_core::Interface::as_raw(self), pshader.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssembleToContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcAssembler_Impl: windows_core::IUnknownImpl {
    fn AssembleToContainer(&self, pshader: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcOperationResult>;
}
impl IDxcAssembler_Vtbl {
    pub const fn new<Identity: IDxcAssembler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AssembleToContainer<Identity: IDxcAssembler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcAssembler_Impl::AssembleToContainer(this, core::mem::transmute_copy(&pshader)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AssembleToContainer: AssembleToContainer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcAssembler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcAssembler {}
windows_core::imp::define_interface!(IDxcBlob, IDxcBlob_Vtbl, 0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
windows_core::imp::interface_hierarchy!(IDxcBlob, windows_core::IUnknown);
impl IDxcBlob {
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetBufferPointer(&self) -> super::LPVOID {
        unsafe { (windows_core::Interface::vtable(self).GetBufferPointer)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferSize(&self) -> super::SIZE_T {
        unsafe { (windows_core::Interface::vtable(self).GetBufferSize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub GetBufferPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::LPVOID,
    #[cfg(not(feature = "minwindef"))]
    GetBufferPointer: usize,
    #[cfg(feature = "shared")]
    pub GetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::SIZE_T,
    #[cfg(not(feature = "shared"))]
    GetBufferSize: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDxcBlob_Impl: windows_core::IUnknownImpl {
    fn GetBufferPointer(&self) -> super::LPVOID;
    fn GetBufferSize(&self) -> super::SIZE_T;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDxcBlob_Vtbl {
    pub const fn new<Identity: IDxcBlob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBufferPointer<Identity: IDxcBlob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::LPVOID {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlob_Impl::GetBufferPointer(this)
            }
        }
        unsafe extern "system" fn GetBufferSize<Identity: IDxcBlob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::SIZE_T {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlob_Impl::GetBufferSize(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Identity, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlob as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDxcBlob {}
windows_core::imp::define_interface!(IDxcBlobEncoding, IDxcBlobEncoding_Vtbl, 0x7241d424_2646_4191_97c0_98e96e42fc68);
impl core::ops::Deref for IDxcBlobEncoding {
    type Target = IDxcBlob;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcBlobEncoding, windows_core::IUnknown, IDxcBlob);
impl IDxcBlobEncoding {
    #[cfg(feature = "shared")]
    pub unsafe fn GetEncoding(&self, pknown: *mut windows_core::BOOL, pcodepage: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEncoding)(windows_core::Interface::as_raw(self), pknown as _, pcodepage as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_Vtbl {
    pub base__: IDxcBlob_Vtbl,
    #[cfg(feature = "shared")]
    pub GetEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEncoding: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDxcBlobEncoding_Impl: IDxcBlob_Impl {
    fn GetEncoding(&self, pknown: *mut windows_core::BOOL, pcodepage: *mut super::UINT32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDxcBlobEncoding_Vtbl {
    pub const fn new<Identity: IDxcBlobEncoding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEncoding<Identity: IDxcBlobEncoding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknown: *mut windows_core::BOOL, pcodepage: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobEncoding_Impl::GetEncoding(this, core::mem::transmute_copy(&pknown), core::mem::transmute_copy(&pcodepage)).into()
            }
        }
        Self { base__: IDxcBlob_Vtbl::new::<Identity, OFFSET>(), GetEncoding: GetEncoding::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobEncoding as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDxcBlobEncoding {}
pub type IDxcBlobUtf16 = IDxcBlobWide;
windows_core::imp::define_interface!(IDxcBlobUtf8, IDxcBlobUtf8_Vtbl, 0x3da636c9_ba71_4024_a301_30cbf125305b);
impl core::ops::Deref for IDxcBlobUtf8 {
    type Target = IDxcBlobEncoding;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcBlobUtf8, windows_core::IUnknown, IDxcBlob, IDxcBlobEncoding);
impl IDxcBlobUtf8 {
    pub unsafe fn GetStringPointer(&self) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetStringPointer)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStringLength(&self) -> super::SIZE_T {
        unsafe { (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::PCSTR,
    #[cfg(feature = "shared")]
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::SIZE_T,
    #[cfg(not(feature = "shared"))]
    GetStringLength: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDxcBlobUtf8_Impl: IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> windows_core::PCSTR;
    fn GetStringLength(&self) -> super::SIZE_T;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDxcBlobUtf8_Vtbl {
    pub const fn new<Identity: IDxcBlobUtf8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStringPointer<Identity: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PCSTR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobUtf8_Impl::GetStringPointer(this)
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::SIZE_T {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobUtf8_Impl::GetStringLength(this)
            }
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobUtf8 as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID || iid == &<IDxcBlobEncoding as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDxcBlobUtf8 {}
windows_core::imp::define_interface!(IDxcBlobWide, IDxcBlobWide_Vtbl, 0xa3f84eab_0faa_497e_a39c_ee6ed60b2d84);
impl core::ops::Deref for IDxcBlobWide {
    type Target = IDxcBlobEncoding;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcBlobWide, windows_core::IUnknown, IDxcBlob, IDxcBlobEncoding);
impl IDxcBlobWide {
    pub unsafe fn GetStringPointer(&self) -> windows_core::PCWSTR {
        unsafe { (windows_core::Interface::vtable(self).GetStringPointer)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStringLength(&self) -> super::SIZE_T {
        unsafe { (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobWide_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::PCWSTR,
    #[cfg(feature = "shared")]
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::SIZE_T,
    #[cfg(not(feature = "shared"))]
    GetStringLength: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDxcBlobWide_Impl: IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> windows_core::PCWSTR;
    fn GetStringLength(&self) -> super::SIZE_T;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDxcBlobWide_Vtbl {
    pub const fn new<Identity: IDxcBlobWide_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStringPointer<Identity: IDxcBlobWide_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PCWSTR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobWide_Impl::GetStringPointer(this)
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IDxcBlobWide_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::SIZE_T {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobWide_Impl::GetStringLength(this)
            }
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobWide as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID || iid == &<IDxcBlobEncoding as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDxcBlobWide {}
windows_core::imp::define_interface!(IDxcCompiler, IDxcCompiler_Vtbl, 0x8c210bf3_011f_4422_8d70_6f9acb8db617);
windows_core::imp::interface_hierarchy!(IDxcCompiler, windows_core::IUnknown);
impl IDxcCompiler {
    #[cfg(feature = "shared")]
    pub unsafe fn Compile<P0, P1, P2, P3, P8>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P8) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P8: windows_core::Param<IDxcIncludeHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compile)(windows_core::Interface::as_raw(self), psource.param().abi(), psourcename.param().abi(), pentrypoint.param().abi(), ptargetprofile.param().abi(), parguments.map_or(core::ptr::null(), |slice| slice.as_ptr()), parguments.map_or(0, |slice| slice.len().try_into().unwrap()), pdefines.as_ptr(), pdefines.len().try_into().unwrap(), pincludehandler.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Preprocess<P0, P1, P6>(&self, psource: P0, psourcename: P1, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P6) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<IDxcIncludeHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Preprocess)(windows_core::Interface::as_raw(self), psource.param().abi(), psourcename.param().abi(), parguments.map_or(core::ptr::null(), |slice| slice.as_ptr()), parguments.map_or(0, |slice| slice.len().try_into().unwrap()), pdefines.as_ptr(), pdefines.len().try_into().unwrap(), pincludehandler.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Disassemble<P0>(&self, psource: P0) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Disassemble)(windows_core::Interface::as_raw(self), psource.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Compile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::PCWSTR, super::UINT32, *const DxcDefine, super::UINT32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Compile: usize,
    #[cfg(feature = "shared")]
    pub Preprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR, super::UINT32, *const DxcDefine, super::UINT32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Preprocess: usize,
    pub Disassemble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDxcCompiler_Impl: windows_core::IUnknownImpl {
    fn Compile(&self, psource: windows_core::Ref<IDxcBlob>, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>) -> windows_core::Result<IDxcOperationResult>;
    fn Preprocess(&self, psource: windows_core::Ref<IDxcBlob>, psourcename: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>) -> windows_core::Result<IDxcOperationResult>;
    fn Disassemble(&self, psource: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
}
#[cfg(feature = "shared")]
impl IDxcCompiler_Vtbl {
    pub const fn new<Identity: IDxcCompiler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compile<Identity: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcCompiler_Impl::Compile(this, core::mem::transmute_copy(&psource), core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), core::mem::transmute_copy(&pincludehandler)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Preprocess<Identity: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcCompiler_Impl::Preprocess(this, core::mem::transmute_copy(&psource), core::mem::transmute(&psourcename), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), core::mem::transmute_copy(&pincludehandler)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Disassemble<Identity: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, ppdisassembly: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcCompiler_Impl::Disassemble(this, core::mem::transmute_copy(&psource)) {
                    Ok(ok__) => {
                        ppdisassembly.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppdisassembly.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, OFFSET>,
            Preprocess: Preprocess::<Identity, OFFSET>,
            Disassemble: Disassemble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcCompiler {}
windows_core::imp::define_interface!(IDxcCompiler2, IDxcCompiler2_Vtbl, 0xa005a9d9_b8bb_4594_b5c9_0e633bec4d37);
impl core::ops::Deref for IDxcCompiler2 {
    type Target = IDxcCompiler;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcCompiler2, windows_core::IUnknown, IDxcCompiler);
impl IDxcCompiler2 {
    #[cfg(feature = "shared")]
    pub unsafe fn CompileWithDebug<P0, P1, P2, P3, P8>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P8, ppresult: *mut Option<IDxcOperationResult>, ppdebugblobname: Option<*mut windows_core::PWSTR>, ppdebugblob: Option<*mut Option<IDxcBlob>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDxcBlob>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P8: windows_core::Param<IDxcIncludeHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CompileWithDebug)(
                windows_core::Interface::as_raw(self),
                psource.param().abi(),
                psourcename.param().abi(),
                pentrypoint.param().abi(),
                ptargetprofile.param().abi(),
                parguments.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                parguments.map_or(0, |slice| slice.len().try_into().unwrap()),
                pdefines.as_ptr(),
                pdefines.len().try_into().unwrap(),
                pincludehandler.param().abi(),
                core::mem::transmute(ppresult),
                ppdebugblobname.unwrap_or(core::mem::zeroed()) as _,
                ppdebugblob.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_Vtbl {
    pub base__: IDxcCompiler_Vtbl,
    #[cfg(feature = "shared")]
    pub CompileWithDebug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::PCWSTR, super::UINT32, *const DxcDefine, super::UINT32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::PWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CompileWithDebug: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcCompiler2_Impl: IDxcCompiler_Impl {
    fn CompileWithDebug(&self, psource: windows_core::Ref<IDxcBlob>, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>, ppresult: windows_core::OutRef<IDxcOperationResult>, ppdebugblobname: *mut windows_core::PWSTR, ppdebugblob: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDxcCompiler2_Vtbl {
    pub const fn new<Identity: IDxcCompiler2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompileWithDebug<Identity: IDxcCompiler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void, ppdebugblobname: *mut windows_core::PWSTR, ppdebugblob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompiler2_Impl::CompileWithDebug(this, core::mem::transmute_copy(&psource), core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), core::mem::transmute_copy(&pincludehandler), core::mem::transmute_copy(&ppresult), core::mem::transmute_copy(&ppdebugblobname), core::mem::transmute_copy(&ppdebugblob)).into()
            }
        }
        Self { base__: IDxcCompiler_Vtbl::new::<Identity, OFFSET>(), CompileWithDebug: CompileWithDebug::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler2 as windows_core::Interface>::IID || iid == &<IDxcCompiler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcCompiler2 {}
windows_core::imp::define_interface!(IDxcCompiler3, IDxcCompiler3_Vtbl, 0x228b4687_5a6a_4730_900c_9702b2203f54);
windows_core::imp::interface_hierarchy!(IDxcCompiler3, windows_core::IUnknown);
impl IDxcCompiler3 {
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub unsafe fn Compile<P3>(&self, psource: *const DxcText, parguments: Option<&[windows_core::PCWSTR]>, pincludehandler: P3, riid: *const super::IID, ppresult: *mut super::LPVOID) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IDxcIncludeHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).Compile)(windows_core::Interface::as_raw(self), psource, parguments.map_or(core::ptr::null(), |slice| slice.as_ptr()), parguments.map_or(0, |slice| slice.len().try_into().unwrap()), pincludehandler.param().abi(), riid, ppresult as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub unsafe fn Disassemble(&self, pobject: *const DxcText, riid: *const super::IID, ppresult: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disassemble)(windows_core::Interface::as_raw(self), pobject, riid, ppresult as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub Compile: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcText, *const windows_core::PCWSTR, super::UINT32, *mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared")))]
    Compile: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub Disassemble: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcText, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared")))]
    Disassemble: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub trait IDxcCompiler3_Impl: windows_core::IUnknownImpl {
    fn Compile(&self, psource: *const DxcText, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>, riid: *const super::IID, ppresult: *mut super::LPVOID) -> windows_core::Result<()>;
    fn Disassemble(&self, pobject: *const DxcText, riid: *const super::IID, ppresult: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl IDxcCompiler3_Vtbl {
    pub const fn new<Identity: IDxcCompiler3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compile<Identity: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *const DxcText, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pincludehandler: *mut core::ffi::c_void, riid: *const super::IID, ppresult: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompiler3_Impl::Compile(this, core::mem::transmute_copy(&psource), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pincludehandler), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn Disassemble<Identity: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const DxcText, riid: *const super::IID, ppresult: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompiler3_Impl::Disassemble(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, OFFSET>,
            Disassemble: Disassemble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDxcCompiler3 {}
windows_core::imp::define_interface!(IDxcCompilerArgs, IDxcCompilerArgs_Vtbl, 0x73effe2a_70dc_45f8_9690_eff64c02429d);
windows_core::imp::interface_hierarchy!(IDxcCompilerArgs, windows_core::IUnknown);
impl IDxcCompilerArgs {
    pub unsafe fn GetArguments(&self) -> *mut windows_core::PCWSTR {
        unsafe { (windows_core::Interface::vtable(self).GetArguments)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self) -> super::UINT32 {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddArguments(&self, parguments: Option<&[windows_core::PCWSTR]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddArguments)(windows_core::Interface::as_raw(self), parguments.map_or(core::ptr::null(), |slice| slice.as_ptr()), parguments.map_or(0, |slice| slice.len().try_into().unwrap())) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddArgumentsUTF8(&self, parguments: Option<&[windows_core::PCSTR]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddArgumentsUTF8)(windows_core::Interface::as_raw(self), parguments.map_or(core::ptr::null(), |slice| slice.as_ptr()), parguments.map_or(0, |slice| slice.len().try_into().unwrap())) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddDefines(&self, pdefines: &[DxcDefine]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDefines)(windows_core::Interface::as_raw(self), pdefines.as_ptr(), pdefines.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetArguments: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut windows_core::PCWSTR,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::UINT32,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(feature = "shared")]
    pub AddArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddArguments: usize,
    #[cfg(feature = "shared")]
    pub AddArgumentsUTF8: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCSTR, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddArgumentsUTF8: usize,
    #[cfg(feature = "shared")]
    pub AddDefines: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcDefine, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddDefines: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcCompilerArgs_Impl: windows_core::IUnknownImpl {
    fn GetArguments(&self) -> *mut windows_core::PCWSTR;
    fn GetCount(&self) -> super::UINT32;
    fn AddArguments(&self, parguments: *const windows_core::PCWSTR, argcount: super::UINT32) -> windows_core::Result<()>;
    fn AddArgumentsUTF8(&self, parguments: *const windows_core::PCSTR, argcount: super::UINT32) -> windows_core::Result<()>;
    fn AddDefines(&self, pdefines: *const DxcDefine, definecount: super::UINT32) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDxcCompilerArgs_Vtbl {
    pub const fn new<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetArguments<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut windows_core::PCWSTR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::GetArguments(this)
            }
        }
        unsafe extern "system" fn GetCount<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::UINT32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::GetCount(this)
            }
        }
        unsafe extern "system" fn AddArguments<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parguments: *const windows_core::PCWSTR, argcount: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::AddArguments(this, core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)).into()
            }
        }
        unsafe extern "system" fn AddArgumentsUTF8<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parguments: *const windows_core::PCSTR, argcount: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::AddArgumentsUTF8(this, core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)).into()
            }
        }
        unsafe extern "system" fn AddDefines<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefines: *const DxcDefine, definecount: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::AddDefines(this, core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetArguments: GetArguments::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            AddArguments: AddArguments::<Identity, OFFSET>,
            AddArgumentsUTF8: AddArgumentsUTF8::<Identity, OFFSET>,
            AddDefines: AddDefines::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompilerArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcCompilerArgs {}
windows_core::imp::define_interface!(IDxcContainerBuilder, IDxcContainerBuilder_Vtbl, 0x334b1f50_2292_4b35_99a1_25588d8c17fe);
windows_core::imp::interface_hierarchy!(IDxcContainerBuilder, windows_core::IUnknown);
impl IDxcContainerBuilder {
    pub unsafe fn Load<P0>(&self, pdxilcontainerheader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pdxilcontainerheader.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddPart<P1>(&self, fourcc: super::UINT32, psource: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPart)(windows_core::Interface::as_raw(self), fourcc, psource.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemovePart(&self, fourcc: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemovePart)(windows_core::Interface::as_raw(self), fourcc) }
    }
    pub unsafe fn SerializeContainer(&self, ppresult: *mut Option<IDxcOperationResult>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SerializeContainer)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddPart: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddPart: usize,
    #[cfg(feature = "shared")]
    pub RemovePart: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemovePart: usize,
    pub SerializeContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDxcContainerBuilder_Impl: windows_core::IUnknownImpl {
    fn Load(&self, pdxilcontainerheader: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn AddPart(&self, fourcc: super::UINT32, psource: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn RemovePart(&self, fourcc: super::UINT32) -> windows_core::Result<()>;
    fn SerializeContainer(&self, ppresult: windows_core::OutRef<IDxcOperationResult>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDxcContainerBuilder_Vtbl {
    pub const fn new<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Load<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdxilcontainerheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerBuilder_Impl::Load(this, core::mem::transmute_copy(&pdxilcontainerheader)).into()
            }
        }
        unsafe extern "system" fn AddPart<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fourcc: super::UINT32, psource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerBuilder_Impl::AddPart(this, core::mem::transmute_copy(&fourcc), core::mem::transmute_copy(&psource)).into()
            }
        }
        unsafe extern "system" fn RemovePart<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fourcc: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerBuilder_Impl::RemovePart(this, core::mem::transmute_copy(&fourcc)).into()
            }
        }
        unsafe extern "system" fn SerializeContainer<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerBuilder_Impl::SerializeContainer(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, OFFSET>,
            AddPart: AddPart::<Identity, OFFSET>,
            RemovePart: RemovePart::<Identity, OFFSET>,
            SerializeContainer: SerializeContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcContainerBuilder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcContainerBuilder {}
windows_core::imp::define_interface!(IDxcContainerReflection, IDxcContainerReflection_Vtbl, 0xd2c21b26_8350_4bdc_976a_331ce6f4c54c);
windows_core::imp::interface_hierarchy!(IDxcContainerReflection, windows_core::IUnknown);
impl IDxcContainerReflection {
    pub unsafe fn Load<P0>(&self, pcontainer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pcontainer.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPartCount(&self, presult: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPartCount)(windows_core::Interface::as_raw(self), presult as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPartKind(&self, idx: super::UINT32, presult: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPartKind)(windows_core::Interface::as_raw(self), idx, presult as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPartContent(&self, idx: super::UINT32) -> windows_core::Result<IDxcBlob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartContent)(windows_core::Interface::as_raw(self), idx, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FindFirstPartKind(&self, kind: super::UINT32, presult: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindFirstPartKind)(windows_core::Interface::as_raw(self), kind, presult as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetPartReflection(&self, idx: super::UINT32, iid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPartReflection)(windows_core::Interface::as_raw(self), idx, iid, ppvobject as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetPartCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPartCount: usize,
    #[cfg(feature = "shared")]
    pub GetPartKind: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPartKind: usize,
    #[cfg(feature = "shared")]
    pub GetPartContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPartContent: usize,
    #[cfg(feature = "shared")]
    pub FindFirstPartKind: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FindFirstPartKind: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetPartReflection: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetPartReflection: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IDxcContainerReflection_Impl: windows_core::IUnknownImpl {
    fn Load(&self, pcontainer: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn GetPartCount(&self, presult: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetPartKind(&self, idx: super::UINT32, presult: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetPartContent(&self, idx: super::UINT32) -> windows_core::Result<IDxcBlob>;
    fn FindFirstPartKind(&self, kind: super::UINT32, presult: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetPartReflection(&self, idx: super::UINT32, iid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IDxcContainerReflection_Vtbl {
    pub const fn new<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Load<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontainer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerReflection_Impl::Load(this, core::mem::transmute_copy(&pcontainer)).into()
            }
        }
        unsafe extern "system" fn GetPartCount<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerReflection_Impl::GetPartCount(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn GetPartKind<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: super::UINT32, presult: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerReflection_Impl::GetPartKind(this, core::mem::transmute_copy(&idx), core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn GetPartContent<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcContainerReflection_Impl::GetPartContent(this, core::mem::transmute_copy(&idx)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn FindFirstPartKind<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kind: super::UINT32, presult: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerReflection_Impl::FindFirstPartKind(this, core::mem::transmute_copy(&kind), core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn GetPartReflection<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: super::UINT32, iid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerReflection_Impl::GetPartReflection(this, core::mem::transmute_copy(&idx), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, OFFSET>,
            GetPartCount: GetPartCount::<Identity, OFFSET>,
            GetPartKind: GetPartKind::<Identity, OFFSET>,
            GetPartContent: GetPartContent::<Identity, OFFSET>,
            FindFirstPartKind: FindFirstPartKind::<Identity, OFFSET>,
            GetPartReflection: GetPartReflection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcContainerReflection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IDxcContainerReflection {}
windows_core::imp::define_interface!(IDxcExtraOutputs, IDxcExtraOutputs_Vtbl, 0x319b37a2_a5c2_494a_a5de_4801b2faf989);
windows_core::imp::interface_hierarchy!(IDxcExtraOutputs, windows_core::IUnknown);
impl IDxcExtraOutputs {
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputCount(&self) -> super::UINT32 {
        unsafe { (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetOutput<T>(&self, uindex: super::UINT32, ppoutputtype: Option<*mut Option<IDxcBlobWide>>, ppoutputname: Option<*mut Option<IDxcBlobWide>>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), uindex, &T::IID, result__ as *mut _ as *mut _, ppoutputtype.unwrap_or(core::mem::zeroed()) as _, ppoutputname.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::UINT32,
    #[cfg(not(feature = "shared"))]
    GetOutputCount: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *const super::IID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetOutput: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IDxcExtraOutputs_Impl: windows_core::IUnknownImpl {
    fn GetOutputCount(&self) -> super::UINT32;
    fn GetOutput(&self, uindex: super::UINT32, iid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void, ppoutputtype: windows_core::OutRef<IDxcBlobWide>, ppoutputname: windows_core::OutRef<IDxcBlobWide>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IDxcExtraOutputs_Vtbl {
    pub const fn new<Identity: IDxcExtraOutputs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputCount<Identity: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::UINT32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcExtraOutputs_Impl::GetOutputCount(this)
            }
        }
        unsafe extern "system" fn GetOutput<Identity: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, iid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void, ppoutputtype: *mut *mut core::ffi::c_void, ppoutputname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcExtraOutputs_Impl::GetOutput(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject), core::mem::transmute_copy(&ppoutputtype), core::mem::transmute_copy(&ppoutputname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            GetOutput: GetOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcExtraOutputs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IDxcExtraOutputs {}
windows_core::imp::define_interface!(IDxcIncludeHandler, IDxcIncludeHandler_Vtbl, 0x7f61fc7d_950d_467f_b3e3_3c02fb49187c);
windows_core::imp::interface_hierarchy!(IDxcIncludeHandler, windows_core::IUnknown);
impl IDxcIncludeHandler {
    pub unsafe fn LoadSource<P0>(&self, pfilename: P0, ppincludesource: *mut Option<IDxcBlob>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadSource)(windows_core::Interface::as_raw(self), pfilename.param().abi(), core::mem::transmute(ppincludesource)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcIncludeHandler_Impl: windows_core::IUnknownImpl {
    fn LoadSource(&self, pfilename: &windows_core::PCWSTR, ppincludesource: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
}
impl IDxcIncludeHandler_Vtbl {
    pub const fn new<Identity: IDxcIncludeHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadSource<Identity: IDxcIncludeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, ppincludesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcIncludeHandler_Impl::LoadSource(this, core::mem::transmute(&pfilename), core::mem::transmute_copy(&ppincludesource)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadSource: LoadSource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcIncludeHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcIncludeHandler {}
windows_core::imp::define_interface!(IDxcLibrary, IDxcLibrary_Vtbl, 0xe5204dc7_d18c_4c3c_bdfb_851673980fe7);
windows_core::imp::interface_hierarchy!(IDxcLibrary, windows_core::IUnknown);
impl IDxcLibrary {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SetMalloc<P0>(&self, pmalloc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMalloc>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMalloc)(windows_core::Interface::as_raw(self), pmalloc.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateBlobFromBlob<P0>(&self, pblob: P0, offset: super::UINT32, length: super::UINT32) -> windows_core::Result<IDxcBlob>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromBlob)(windows_core::Interface::as_raw(self), pblob.param().abi(), offset, length, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateBlobFromFile<P0>(&self, pfilename: P0, codepage: Option<*const super::UINT32>) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromFile)(windows_core::Interface::as_raw(self), pfilename.param().abi(), codepage.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobWithEncodingFromPinned)(windows_core::Interface::as_raw(self), ptext, size, codepage, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobWithEncodingOnHeapCopy)(windows_core::Interface::as_raw(self), ptext, size, codepage, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<P1>(&self, ptext: super::LPCVOID, pimalloc: P1, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>
    where
        P1: windows_core::Param<super::IMalloc>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobWithEncodingOnMalloc)(windows_core::Interface::as_raw(self), ptext, pimalloc.param().abi(), size, codepage, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateIncludeHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateStreamFromBlobReadOnly<P0>(&self, pblob: P0) -> windows_core::Result<super::IStream>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStreamFromBlobReadOnly)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsUtf8<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsUtf8)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsWide<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsWide)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub SetMalloc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SetMalloc: usize,
    #[cfg(feature = "shared")]
    pub CreateBlobFromBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateBlobFromBlob: usize,
    #[cfg(feature = "shared")]
    pub CreateBlobFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateBlobFromFile: usize,
    #[cfg(feature = "shared")]
    pub CreateBlobWithEncodingFromPinned: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCVOID, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateBlobWithEncodingFromPinned: usize,
    #[cfg(feature = "shared")]
    pub CreateBlobWithEncodingOnHeapCopy: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCVOID, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateBlobWithEncodingOnHeapCopy: usize,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub CreateBlobWithEncodingOnMalloc: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCVOID, *mut core::ffi::c_void, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    CreateBlobWithEncodingOnMalloc: usize,
    pub CreateIncludeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub CreateStreamFromBlobReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateStreamFromBlobReadOnly: usize,
    pub GetBlobAsUtf8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlobAsWide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
pub trait IDxcLibrary_Impl: windows_core::IUnknownImpl {
    fn SetMalloc(&self, pmalloc: windows_core::Ref<super::IMalloc>) -> windows_core::Result<()>;
    fn CreateBlobFromBlob(&self, pblob: windows_core::Ref<IDxcBlob>, offset: super::UINT32, length: super::UINT32) -> windows_core::Result<IDxcBlob>;
    fn CreateBlobFromFile(&self, pfilename: &windows_core::PCWSTR, codepage: *const super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingFromPinned(&self, ptext: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnMalloc(&self, ptext: super::LPCVOID, pimalloc: windows_core::Ref<super::IMalloc>, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler>;
    fn CreateStreamFromBlobReadOnly(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<super::IStream>;
    fn GetBlobAsUtf8(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
    fn GetBlobAsWide(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl IDxcLibrary_Vtbl {
    pub const fn new<Identity: IDxcLibrary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMalloc<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmalloc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcLibrary_Impl::SetMalloc(this, core::mem::transmute_copy(&pmalloc)).into()
            }
        }
        unsafe extern "system" fn CreateBlobFromBlob<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, offset: super::UINT32, length: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobFromBlob(this, core::mem::transmute_copy(&pblob), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&length)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateBlobFromFile<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, codepage: *const super::UINT32, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobFromFile(this, core::mem::transmute(&pfilename), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: super::LPCVOID, size: super::UINT32, codepage: super::UINT32, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobWithEncodingFromPinned(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: super::LPCVOID, size: super::UINT32, codepage: super::UINT32, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobWithEncodingOnHeapCopy(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: super::LPCVOID, pimalloc: *mut core::ffi::c_void, size: super::UINT32, codepage: super::UINT32, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobWithEncodingOnMalloc(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&pimalloc), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateIncludeHandler<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateIncludeHandler(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateStreamFromBlobReadOnly(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppstream.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::GetBlobAsUtf8(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetBlobAsWide<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::GetBlobAsWide(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        pblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMalloc: SetMalloc::<Identity, OFFSET>,
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, OFFSET>,
            CreateBlobFromFile: CreateBlobFromFile::<Identity, OFFSET>,
            CreateBlobWithEncodingFromPinned: CreateBlobWithEncodingFromPinned::<Identity, OFFSET>,
            CreateBlobWithEncodingOnHeapCopy: CreateBlobWithEncodingOnHeapCopy::<Identity, OFFSET>,
            CreateBlobWithEncodingOnMalloc: CreateBlobWithEncodingOnMalloc::<Identity, OFFSET>,
            CreateIncludeHandler: CreateIncludeHandler::<Identity, OFFSET>,
            CreateStreamFromBlobReadOnly: CreateStreamFromBlobReadOnly::<Identity, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, OFFSET>,
            GetBlobAsWide: GetBlobAsWide::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcLibrary as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl windows_core::RuntimeName for IDxcLibrary {}
windows_core::imp::define_interface!(IDxcOperationResult, IDxcOperationResult_Vtbl, 0xcedb484a_d4e9_445a_b991_ca21ca157dc2);
windows_core::imp::interface_hierarchy!(IDxcOperationResult, windows_core::IUnknown);
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self, pstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pstatus as _) }
    }
    pub unsafe fn GetResult(&self, ppresult: *mut Option<IDxcBlob>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn GetErrorBuffer(&self, pperrors: *mut Option<IDxcBlobEncoding>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(pperrors)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcOperationResult_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, pstatus: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetResult(&self, ppresult: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
    fn GetErrorBuffer(&self, pperrors: windows_core::OutRef<IDxcBlobEncoding>) -> windows_core::Result<()>;
}
impl IDxcOperationResult_Vtbl {
    pub const fn new<Identity: IDxcOperationResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcOperationResult_Impl::GetStatus(this, core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn GetResult<Identity: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcOperationResult_Impl::GetResult(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn GetErrorBuffer<Identity: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcOperationResult_Impl::GetErrorBuffer(this, core::mem::transmute_copy(&pperrors)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
            GetErrorBuffer: GetErrorBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOperationResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcOperationResult {}
windows_core::imp::define_interface!(IDxcOptimizer, IDxcOptimizer_Vtbl, 0x25740e2e_9cba_401b_9119_4fb42f39f270);
windows_core::imp::interface_hierarchy!(IDxcOptimizer, windows_core::IUnknown);
impl IDxcOptimizer {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAvailablePassCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailablePassCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAvailablePass(&self, index: super::UINT32) -> windows_core::Result<IDxcOptimizerPass> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailablePass)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RunOptimizer<P0>(&self, pblob: P0, ppoptions: &[windows_core::PCWSTR], poutputmodule: *mut Option<IDxcBlob>, ppoutputtext: Option<*mut Option<IDxcBlobEncoding>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).RunOptimizer)(windows_core::Interface::as_raw(self), pblob.param().abi(), ppoptions.as_ptr(), ppoptions.len().try_into().unwrap(), core::mem::transmute(poutputmodule), ppoutputtext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAvailablePassCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAvailablePassCount: usize,
    #[cfg(feature = "shared")]
    pub GetAvailablePass: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAvailablePass: usize,
    #[cfg(feature = "shared")]
    pub RunOptimizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::PCWSTR, super::UINT32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RunOptimizer: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcOptimizer_Impl: windows_core::IUnknownImpl {
    fn GetAvailablePassCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetAvailablePass(&self, index: super::UINT32) -> windows_core::Result<IDxcOptimizerPass>;
    fn RunOptimizer(&self, pblob: windows_core::Ref<IDxcBlob>, ppoptions: *const windows_core::PCWSTR, optioncount: super::UINT32, poutputmodule: windows_core::OutRef<IDxcBlob>, ppoutputtext: windows_core::OutRef<IDxcBlobEncoding>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDxcOptimizer_Vtbl {
    pub const fn new<Identity: IDxcOptimizer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailablePassCount<Identity: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcOptimizer_Impl::GetAvailablePassCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetAvailablePass<Identity: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizer_Impl::GetAvailablePass(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn RunOptimizer<Identity: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppoptions: *const windows_core::PCWSTR, optioncount: super::UINT32, poutputmodule: *mut *mut core::ffi::c_void, ppoutputtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcOptimizer_Impl::RunOptimizer(this, core::mem::transmute_copy(&pblob), core::mem::transmute_copy(&ppoptions), core::mem::transmute_copy(&optioncount), core::mem::transmute_copy(&poutputmodule), core::mem::transmute_copy(&ppoutputtext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailablePassCount: GetAvailablePassCount::<Identity, OFFSET>,
            GetAvailablePass: GetAvailablePass::<Identity, OFFSET>,
            RunOptimizer: RunOptimizer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOptimizer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcOptimizer {}
windows_core::imp::define_interface!(IDxcOptimizerPass, IDxcOptimizerPass_Vtbl, 0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
windows_core::imp::interface_hierarchy!(IDxcOptimizerPass, windows_core::IUnknown);
impl IDxcOptimizerPass {
    pub unsafe fn GetOptionName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOptionArgCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOptionArgCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOptionArgName(&self, argindex: super::UINT32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionArgName)(windows_core::Interface::as_raw(self), argindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOptionArgDescription(&self, argindex: super::UINT32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionArgDescription)(windows_core::Interface::as_raw(self), argindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetOptionArgCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOptionArgCount: usize,
    #[cfg(feature = "shared")]
    pub GetOptionArgName: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOptionArgName: usize,
    #[cfg(feature = "shared")]
    pub GetOptionArgDescription: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOptionArgDescription: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcOptimizerPass_Impl: windows_core::IUnknownImpl {
    fn GetOptionName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionArgCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetOptionArgName(&self, argindex: super::UINT32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionArgDescription(&self, argindex: super::UINT32) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "shared")]
impl IDxcOptimizerPass_Vtbl {
    pub const fn new<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionName<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetOptionName(this) {
                    Ok(ok__) => {
                        ppresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        ppresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetOptionArgCount<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcOptimizerPass_Impl::GetOptionArgCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetOptionArgName<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argindex: super::UINT32, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetOptionArgName(this, core::mem::transmute_copy(&argindex)) {
                    Ok(ok__) => {
                        ppresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetOptionArgDescription<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argindex: super::UINT32, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetOptionArgDescription(this, core::mem::transmute_copy(&argindex)) {
                    Ok(ok__) => {
                        ppresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionName: GetOptionName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetOptionArgCount: GetOptionArgCount::<Identity, OFFSET>,
            GetOptionArgName: GetOptionArgName::<Identity, OFFSET>,
            GetOptionArgDescription: GetOptionArgDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOptimizerPass as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcOptimizerPass {}
windows_core::imp::define_interface!(IDxcPdbUtils2, IDxcPdbUtils2_Vtbl, 0x4315d938_f369_4f93_95a2_252017cc3807);
windows_core::imp::interface_hierarchy!(IDxcPdbUtils2, windows_core::IUnknown);
impl IDxcPdbUtils2 {
    pub unsafe fn Load<P0>(&self, ppdbordxil: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ppdbordxil.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSourceCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSourceCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSource(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSourceName(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceName)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLibraryPDBCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLibraryPDBCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLibraryPDB(&self, uindex: super::UINT32, ppoutpdbutils: *mut Option<Self>, pplibraryname: Option<*mut Option<IDxcBlobWide>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLibraryPDB)(windows_core::Interface::as_raw(self), uindex, core::mem::transmute(ppoutpdbutils), pplibraryname.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFlagCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFlagCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFlag(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlag)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetArgCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetArgCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetArg(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArg)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetArgPairCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetArgPairCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetArgPair(&self, uindex: super::UINT32, ppname: *mut Option<IDxcBlobWide>, ppvalue: *mut Option<IDxcBlobWide>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetArgPair)(windows_core::Interface::as_raw(self), uindex, core::mem::transmute(ppname), core::mem::transmute(ppvalue)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDefineCount(&self, pcount: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDefineCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDefine(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefine)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTargetProfile(&self, ppresult: *mut Option<IDxcBlobWide>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTargetProfile)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn GetEntryPoint(&self, ppresult: *mut Option<IDxcBlobWide>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEntryPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn GetMainFileName(&self, ppresult: *mut Option<IDxcBlobWide>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMainFileName)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn GetHash(&self, ppresult: *mut Option<IDxcBlob>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHash)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn GetName(&self, ppresult: *mut Option<IDxcBlobWide>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn GetVersionInfo(&self, ppversioninfo: *mut Option<IDxcVersionInfo>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVersionInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(ppversioninfo)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCustomToolchainID(&self, pid: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCustomToolchainID)(windows_core::Interface::as_raw(self), pid as _) }
    }
    pub unsafe fn GetCustomToolchainData(&self, ppblob: *mut Option<IDxcBlob>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCustomToolchainData)(windows_core::Interface::as_raw(self), core::mem::transmute(ppblob)) }
    }
    pub unsafe fn GetWholeDxil(&self, ppresult: *mut Option<IDxcBlob>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWholeDxil)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresult)) }
    }
    pub unsafe fn IsFullPDB(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsFullPDB)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsPDBRef(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsPDBRef)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetSourceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSourceCount: usize,
    #[cfg(feature = "shared")]
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSource: usize,
    #[cfg(feature = "shared")]
    pub GetSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSourceName: usize,
    #[cfg(feature = "shared")]
    pub GetLibraryPDBCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLibraryPDBCount: usize,
    #[cfg(feature = "shared")]
    pub GetLibraryPDB: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLibraryPDB: usize,
    #[cfg(feature = "shared")]
    pub GetFlagCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFlagCount: usize,
    #[cfg(feature = "shared")]
    pub GetFlag: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFlag: usize,
    #[cfg(feature = "shared")]
    pub GetArgCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetArgCount: usize,
    #[cfg(feature = "shared")]
    pub GetArg: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetArg: usize,
    #[cfg(feature = "shared")]
    pub GetArgPairCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetArgPairCount: usize,
    #[cfg(feature = "shared")]
    pub GetArgPair: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetArgPair: usize,
    #[cfg(feature = "shared")]
    pub GetDefineCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDefineCount: usize,
    #[cfg(feature = "shared")]
    pub GetDefine: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDefine: usize,
    pub GetTargetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEntryPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMainFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVersionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCustomToolchainID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCustomToolchainID: usize,
    pub GetCustomToolchainData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWholeDxil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFullPDB: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub IsPDBRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(feature = "shared")]
pub trait IDxcPdbUtils2_Impl: windows_core::IUnknownImpl {
    fn Load(&self, ppdbordxil: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn GetSourceCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetSource(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn GetSourceName(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide>;
    fn GetLibraryPDBCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetLibraryPDB(&self, uindex: super::UINT32, ppoutpdbutils: windows_core::OutRef<IDxcPdbUtils2>, pplibraryname: windows_core::OutRef<IDxcBlobWide>) -> windows_core::Result<()>;
    fn GetFlagCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetFlag(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide>;
    fn GetArgCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetArg(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide>;
    fn GetArgPairCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetArgPair(&self, uindex: super::UINT32, ppname: windows_core::OutRef<IDxcBlobWide>, ppvalue: windows_core::OutRef<IDxcBlobWide>) -> windows_core::Result<()>;
    fn GetDefineCount(&self, pcount: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetDefine(&self, uindex: super::UINT32) -> windows_core::Result<IDxcBlobWide>;
    fn GetTargetProfile(&self, ppresult: windows_core::OutRef<IDxcBlobWide>) -> windows_core::Result<()>;
    fn GetEntryPoint(&self, ppresult: windows_core::OutRef<IDxcBlobWide>) -> windows_core::Result<()>;
    fn GetMainFileName(&self, ppresult: windows_core::OutRef<IDxcBlobWide>) -> windows_core::Result<()>;
    fn GetHash(&self, ppresult: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
    fn GetName(&self, ppresult: windows_core::OutRef<IDxcBlobWide>) -> windows_core::Result<()>;
    fn GetVersionInfo(&self, ppversioninfo: windows_core::OutRef<IDxcVersionInfo>) -> windows_core::Result<()>;
    fn GetCustomToolchainID(&self, pid: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetCustomToolchainData(&self, ppblob: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
    fn GetWholeDxil(&self, ppresult: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
    fn IsFullPDB(&self) -> windows_core::BOOL;
    fn IsPDBRef(&self) -> windows_core::BOOL;
}
#[cfg(feature = "shared")]
impl IDxcPdbUtils2_Vtbl {
    pub const fn new<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Load<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdbordxil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::Load(this, core::mem::transmute_copy(&ppdbordxil)).into()
            }
        }
        unsafe extern "system" fn GetSourceCount<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetSourceCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetSource<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils2_Impl::GetSource(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetSourceName<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils2_Impl::GetSourceName(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetLibraryPDBCount<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetLibraryPDBCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetLibraryPDB<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, ppoutpdbutils: *mut *mut core::ffi::c_void, pplibraryname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetLibraryPDB(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&ppoutpdbutils), core::mem::transmute_copy(&pplibraryname)).into()
            }
        }
        unsafe extern "system" fn GetFlagCount<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetFlagCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetFlag<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils2_Impl::GetFlag(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetArgCount<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetArgCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetArg<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils2_Impl::GetArg(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetArgPairCount<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetArgPairCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetArgPair<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, ppname: *mut *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetArgPair(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&ppname), core::mem::transmute_copy(&ppvalue)).into()
            }
        }
        unsafe extern "system" fn GetDefineCount<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetDefineCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetDefine<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils2_Impl::GetDefine(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetTargetProfile<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetTargetProfile(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn GetEntryPoint<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetEntryPoint(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn GetMainFileName<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetMainFileName(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn GetHash<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetHash(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetName(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn GetVersionInfo<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppversioninfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetVersionInfo(this, core::mem::transmute_copy(&ppversioninfo)).into()
            }
        }
        unsafe extern "system" fn GetCustomToolchainID<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetCustomToolchainID(this, core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn GetCustomToolchainData<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetCustomToolchainData(this, core::mem::transmute_copy(&ppblob)).into()
            }
        }
        unsafe extern "system" fn GetWholeDxil<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::GetWholeDxil(this, core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn IsFullPDB<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::IsFullPDB(this)
            }
        }
        unsafe extern "system" fn IsPDBRef<Identity: IDxcPdbUtils2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils2_Impl::IsPDBRef(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetSourceName: GetSourceName::<Identity, OFFSET>,
            GetLibraryPDBCount: GetLibraryPDBCount::<Identity, OFFSET>,
            GetLibraryPDB: GetLibraryPDB::<Identity, OFFSET>,
            GetFlagCount: GetFlagCount::<Identity, OFFSET>,
            GetFlag: GetFlag::<Identity, OFFSET>,
            GetArgCount: GetArgCount::<Identity, OFFSET>,
            GetArg: GetArg::<Identity, OFFSET>,
            GetArgPairCount: GetArgPairCount::<Identity, OFFSET>,
            GetArgPair: GetArgPair::<Identity, OFFSET>,
            GetDefineCount: GetDefineCount::<Identity, OFFSET>,
            GetDefine: GetDefine::<Identity, OFFSET>,
            GetTargetProfile: GetTargetProfile::<Identity, OFFSET>,
            GetEntryPoint: GetEntryPoint::<Identity, OFFSET>,
            GetMainFileName: GetMainFileName::<Identity, OFFSET>,
            GetHash: GetHash::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetVersionInfo: GetVersionInfo::<Identity, OFFSET>,
            GetCustomToolchainID: GetCustomToolchainID::<Identity, OFFSET>,
            GetCustomToolchainData: GetCustomToolchainData::<Identity, OFFSET>,
            GetWholeDxil: GetWholeDxil::<Identity, OFFSET>,
            IsFullPDB: IsFullPDB::<Identity, OFFSET>,
            IsPDBRef: IsPDBRef::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcPdbUtils2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcPdbUtils2 {}
windows_core::imp::define_interface!(IDxcUtils, IDxcUtils_Vtbl, 0x4605c4cb_2019_492a_ada4_65f20bb7d67f);
windows_core::imp::interface_hierarchy!(IDxcUtils, windows_core::IUnknown);
impl IDxcUtils {
    #[cfg(feature = "shared")]
    pub unsafe fn CreateBlobFromBlob<P0>(&self, pblob: P0, offset: super::UINT32, length: super::UINT32) -> windows_core::Result<IDxcBlob>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromBlob)(windows_core::Interface::as_raw(self), pblob.param().abi(), offset, length, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateBlobFromPinned(&self, pdata: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromPinned)(windows_core::Interface::as_raw(self), pdata, size, codepage, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn MoveToBlob<P1>(&self, pdata: super::LPCVOID, pimalloc: P1, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>
    where
        P1: windows_core::Param<super::IMalloc>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveToBlob)(windows_core::Interface::as_raw(self), pdata, pimalloc.param().abi(), size, codepage, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateBlob(&self, pdata: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlob)(windows_core::Interface::as_raw(self), pdata, size, codepage, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn LoadFile<P0>(&self, pfilename: P0, pcodepage: Option<*const super::UINT32>) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadFile)(windows_core::Interface::as_raw(self), pfilename.param().abi(), pcodepage.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateReadOnlyStreamFromBlob<P0>(&self, pblob: P0) -> windows_core::Result<super::IStream>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateReadOnlyStreamFromBlob)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDefaultIncludeHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsUtf8<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobUtf8>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsUtf8)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsWide<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobWide>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsWide)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcText, dxcpart: super::UINT32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDxilContainerPart)(windows_core::Interface::as_raw(self), pshader, dxcpart, pppartdata as _, ppartsizeinbytes as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn CreateReflection(&self, pdata: *const DxcText, iid: *const super::IID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateReflection)(windows_core::Interface::as_raw(self), pdata, iid, ppvreflection as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn BuildArguments<P0, P1, P2>(&self, psourcename: P0, pentrypoint: P1, ptargetprofile: P2, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine]) -> windows_core::Result<IDxcCompilerArgs>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuildArguments)(windows_core::Interface::as_raw(self), psourcename.param().abi(), pentrypoint.param().abi(), ptargetprofile.param().abi(), parguments.map_or(core::ptr::null(), |slice| slice.as_ptr()), parguments.map_or(0, |slice| slice.len().try_into().unwrap()), pdefines.as_ptr(), pdefines.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPDBContents<P0>(&self, ppdbblob: P0, pphash: *mut Option<IDxcBlob>) -> windows_core::Result<IDxcBlob>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPDBContents)(windows_core::Interface::as_raw(self), ppdbblob.param().abi(), core::mem::transmute(pphash), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub CreateBlobFromBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateBlobFromBlob: usize,
    #[cfg(feature = "shared")]
    pub CreateBlobFromPinned: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCVOID, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateBlobFromPinned: usize,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub MoveToBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCVOID, *mut core::ffi::c_void, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    MoveToBlob: usize,
    #[cfg(feature = "shared")]
    pub CreateBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCVOID, super::UINT32, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateBlob: usize,
    #[cfg(feature = "shared")]
    pub LoadFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LoadFile: usize,
    #[cfg(feature = "objidlbase")]
    pub CreateReadOnlyStreamFromBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateReadOnlyStreamFromBlob: usize,
    pub CreateDefaultIncludeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlobAsUtf8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlobAsWide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetDxilContainerPart: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcText, super::UINT32, *mut *mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDxilContainerPart: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub CreateReflection: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcText, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    CreateReflection: usize,
    #[cfg(feature = "shared")]
    pub BuildArguments: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::PCWSTR, super::UINT32, *const DxcDefine, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    BuildArguments: usize,
    pub GetPDBContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "shared"))]
pub trait IDxcUtils_Impl: windows_core::IUnknownImpl {
    fn CreateBlobFromBlob(&self, pblob: windows_core::Ref<IDxcBlob>, offset: super::UINT32, length: super::UINT32) -> windows_core::Result<IDxcBlob>;
    fn CreateBlobFromPinned(&self, pdata: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn MoveToBlob(&self, pdata: super::LPCVOID, pimalloc: windows_core::Ref<super::IMalloc>, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlob(&self, pdata: super::LPCVOID, size: super::UINT32, codepage: super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn LoadFile(&self, pfilename: &windows_core::PCWSTR, pcodepage: *const super::UINT32) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateReadOnlyStreamFromBlob(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<super::IStream>;
    fn CreateDefaultIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler>;
    fn GetBlobAsUtf8(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobUtf8>;
    fn GetBlobAsWide(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobWide>;
    fn GetDxilContainerPart(&self, pshader: *const DxcText, dxcpart: super::UINT32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut super::UINT32) -> windows_core::Result<()>;
    fn CreateReflection(&self, pdata: *const DxcText, iid: *const super::IID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn BuildArguments(&self, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32) -> windows_core::Result<IDxcCompilerArgs>;
    fn GetPDBContents(&self, ppdbblob: windows_core::Ref<IDxcBlob>, pphash: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<IDxcBlob>;
}
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "shared"))]
impl IDxcUtils_Vtbl {
    pub const fn new<Identity: IDxcUtils_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBlobFromBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, offset: super::UINT32, length: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateBlobFromBlob(this, core::mem::transmute_copy(&pblob), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&length)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateBlobFromPinned<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: super::LPCVOID, size: super::UINT32, codepage: super::UINT32, ppblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateBlobFromPinned(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        ppblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn MoveToBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: super::LPCVOID, pimalloc: *mut core::ffi::c_void, size: super::UINT32, codepage: super::UINT32, ppblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::MoveToBlob(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pimalloc), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        ppblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: super::LPCVOID, size: super::UINT32, codepage: super::UINT32, ppblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateBlob(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        ppblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn LoadFile<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, pcodepage: *const super::UINT32, ppblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::LoadFile(this, core::mem::transmute(&pfilename), core::mem::transmute_copy(&pcodepage)) {
                    Ok(ok__) => {
                        ppblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateReadOnlyStreamFromBlob(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppstream.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateDefaultIncludeHandler(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::GetBlobAsUtf8(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        ppblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetBlobAsWide<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::GetBlobAsWide(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        ppblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppblobencoding.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetDxilContainerPart<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *const DxcText, dxcpart: super::UINT32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcUtils_Impl::GetDxilContainerPart(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&dxcpart), core::mem::transmute_copy(&pppartdata), core::mem::transmute_copy(&ppartsizeinbytes)).into()
            }
        }
        unsafe extern "system" fn CreateReflection<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const DxcText, iid: *const super::IID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcUtils_Impl::CreateReflection(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvreflection)).into()
            }
        }
        unsafe extern "system" fn BuildArguments<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: super::UINT32, pdefines: *const DxcDefine, definecount: super::UINT32, ppargs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::BuildArguments(this, core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount)) {
                    Ok(ok__) => {
                        ppargs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppargs.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn GetPDBContents<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdbblob: *mut core::ffi::c_void, pphash: *mut *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::GetPDBContents(this, core::mem::transmute_copy(&ppdbblob), core::mem::transmute_copy(&pphash)) {
                    Ok(ok__) => {
                        ppcontainer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppcontainer.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, OFFSET>,
            CreateBlobFromPinned: CreateBlobFromPinned::<Identity, OFFSET>,
            MoveToBlob: MoveToBlob::<Identity, OFFSET>,
            CreateBlob: CreateBlob::<Identity, OFFSET>,
            LoadFile: LoadFile::<Identity, OFFSET>,
            CreateReadOnlyStreamFromBlob: CreateReadOnlyStreamFromBlob::<Identity, OFFSET>,
            CreateDefaultIncludeHandler: CreateDefaultIncludeHandler::<Identity, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, OFFSET>,
            GetBlobAsWide: GetBlobAsWide::<Identity, OFFSET>,
            GetDxilContainerPart: GetDxilContainerPart::<Identity, OFFSET>,
            CreateReflection: CreateReflection::<Identity, OFFSET>,
            BuildArguments: BuildArguments::<Identity, OFFSET>,
            GetPDBContents: GetPDBContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcUtils as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "shared"))]
impl windows_core::RuntimeName for IDxcUtils {}
windows_core::imp::define_interface!(IDxcValidator, IDxcValidator_Vtbl, 0xa6e82bd2_1fd7_4826_9811_2857e797f49a);
windows_core::imp::interface_hierarchy!(IDxcValidator, windows_core::IUnknown);
impl IDxcValidator {
    #[cfg(feature = "shared")]
    pub unsafe fn Validate<P0>(&self, pshader: P0, flags: super::UINT32) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), pshader.param().abi(), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Validate: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcValidator_Impl: windows_core::IUnknownImpl {
    fn Validate(&self, pshader: windows_core::Ref<IDxcBlob>, flags: super::UINT32) -> windows_core::Result<IDxcOperationResult>;
}
#[cfg(feature = "shared")]
impl IDxcValidator_Vtbl {
    pub const fn new<Identity: IDxcValidator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Validate<Identity: IDxcValidator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, flags: super::UINT32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcValidator_Impl::Validate(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Validate: Validate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcValidator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcValidator {}
windows_core::imp::define_interface!(IDxcValidator2, IDxcValidator2_Vtbl, 0x458e1fd1_b1b2_4750_a6e1_9c10f03bed92);
impl core::ops::Deref for IDxcValidator2 {
    type Target = IDxcValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcValidator2, windows_core::IUnknown, IDxcValidator);
impl IDxcValidator2 {
    #[cfg(feature = "shared")]
    pub unsafe fn ValidateWithDebug<P0>(&self, pshader: P0, flags: super::UINT32, poptdebugbitcode: Option<*const DxcText>) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateWithDebug)(windows_core::Interface::as_raw(self), pshader.param().abi(), flags, poptdebugbitcode.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_Vtbl {
    pub base__: IDxcValidator_Vtbl,
    #[cfg(feature = "shared")]
    pub ValidateWithDebug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT32, *const DxcText, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ValidateWithDebug: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcValidator2_Impl: IDxcValidator_Impl {
    fn ValidateWithDebug(&self, pshader: windows_core::Ref<IDxcBlob>, flags: super::UINT32, poptdebugbitcode: *const DxcText) -> windows_core::Result<IDxcOperationResult>;
}
#[cfg(feature = "shared")]
impl IDxcValidator2_Vtbl {
    pub const fn new<Identity: IDxcValidator2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValidateWithDebug<Identity: IDxcValidator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, flags: super::UINT32, poptdebugbitcode: *const DxcText, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcValidator2_Impl::ValidateWithDebug(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&poptdebugbitcode)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppresult.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self { base__: IDxcValidator_Vtbl::new::<Identity, OFFSET>(), ValidateWithDebug: ValidateWithDebug::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcValidator2 as windows_core::Interface>::IID || iid == &<IDxcValidator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcValidator2 {}
windows_core::imp::define_interface!(IDxcVersionInfo, IDxcVersionInfo_Vtbl, 0xb04f5b50_2059_4f12_a8ff_a1e0cde1cc7e);
windows_core::imp::interface_hierarchy!(IDxcVersionInfo, windows_core::IUnknown);
impl IDxcVersionInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn GetVersion(&self, pmajor: *mut super::UINT32, pminor: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), pmajor as _, pminor as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFlags(&self, pflags: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), pflags as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVersion: usize,
    #[cfg(feature = "shared")]
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFlags: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcVersionInfo_Impl: windows_core::IUnknownImpl {
    fn GetVersion(&self, pmajor: *mut super::UINT32, pminor: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetFlags(&self, pflags: *mut super::UINT32) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDxcVersionInfo_Vtbl {
    pub const fn new<Identity: IDxcVersionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersion<Identity: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmajor: *mut super::UINT32, pminor: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcVersionInfo_Impl::GetVersion(this, core::mem::transmute_copy(&pmajor), core::mem::transmute_copy(&pminor)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcVersionInfo_Impl::GetFlags(this, core::mem::transmute_copy(&pflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcVersionInfo {}
windows_core::imp::define_interface!(IDxcVersionInfo2, IDxcVersionInfo2_Vtbl, 0xfb6904c4_42f0_4b62_9c46_983af7da7c83);
impl core::ops::Deref for IDxcVersionInfo2 {
    type Target = IDxcVersionInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcVersionInfo2, windows_core::IUnknown, IDxcVersionInfo);
impl IDxcVersionInfo2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut super::UINT32, pcommithash: *mut *mut i8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCommitInfo)(windows_core::Interface::as_raw(self), pcommitcount as _, pcommithash as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_Vtbl {
    pub base__: IDxcVersionInfo_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCommitInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32, *mut *mut i8) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCommitInfo: usize,
}
#[cfg(feature = "shared")]
pub trait IDxcVersionInfo2_Impl: IDxcVersionInfo_Impl {
    fn GetCommitInfo(&self, pcommitcount: *mut super::UINT32, pcommithash: *mut *mut i8) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDxcVersionInfo2_Vtbl {
    pub const fn new<Identity: IDxcVersionInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCommitInfo<Identity: IDxcVersionInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommitcount: *mut super::UINT32, pcommithash: *mut *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcVersionInfo2_Impl::GetCommitInfo(this, core::mem::transmute_copy(&pcommitcount), core::mem::transmute_copy(&pcommithash)).into()
            }
        }
        Self { base__: IDxcVersionInfo_Vtbl::new::<Identity, OFFSET>(), GetCommitInfo: GetCommitInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo2 as windows_core::Interface>::IID || iid == &<IDxcVersionInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDxcVersionInfo2 {}
windows_core::imp::define_interface!(IDxcVersionInfo3, IDxcVersionInfo3_Vtbl, 0x5e13e843_9d25_473c_9ad2_03b2d0b44b1e);
windows_core::imp::interface_hierarchy!(IDxcVersionInfo3, windows_core::IUnknown);
impl IDxcVersionInfo3 {
    pub unsafe fn GetCustomVersionString(&self, pversionstring: *mut *mut i8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCustomVersionString)(windows_core::Interface::as_raw(self), pversionstring as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCustomVersionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut i8) -> windows_core::HRESULT,
}
pub trait IDxcVersionInfo3_Impl: windows_core::IUnknownImpl {
    fn GetCustomVersionString(&self, pversionstring: *mut *mut i8) -> windows_core::Result<()>;
}
impl IDxcVersionInfo3_Vtbl {
    pub const fn new<Identity: IDxcVersionInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCustomVersionString<Identity: IDxcVersionInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversionstring: *mut *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcVersionInfo3_Impl::GetCustomVersionString(this, core::mem::transmute_copy(&pversionstring)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCustomVersionString: GetCustomVersionString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcVersionInfo3 {}
