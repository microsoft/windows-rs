#[inline]
pub unsafe fn DxcCreateInstance<T>(rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<T>
where
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("dxcompiler.dll" "system" fn DxcCreateInstance(rclsid : *const ::windows_core::GUID, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    DxcCreateInstance(rclsid, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DxcCreateInstance2<P0, T>(pmalloc: P0, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<T>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::IMalloc>,
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("dxcompiler.dll" "system" fn DxcCreateInstance2(pmalloc : * mut::core::ffi::c_void, rclsid : *const ::windows_core::GUID, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    DxcCreateInstance2(pmalloc.into_param().abi(), rclsid, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcAssembler(::windows_core::IUnknown);
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<P0>(&self, pshader: P0) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssembleToContainer)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcAssembler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcAssembler {
    type Vtable = IDxcAssembler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcAssembler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x091f7a26_1c1f_4948_904b_e6e3a8a771d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AssembleToContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcBlob(::windows_core::IUnknown);
impl IDxcBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows_core::Interface::vtable(self).GetBufferPointer)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows_core::Interface::vtable(self).GetBufferSize)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IDxcBlob, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcBlob {
    type Vtable = IDxcBlob_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcBlob {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetBufferPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcBlobEncoding(::windows_core::IUnknown);
impl IDxcBlobEncoding {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows_core::Interface::vtable(self).base__.GetBufferPointer)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows_core::Interface::vtable(self).base__.GetBufferSize)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEncoding)(::windows_core::Interface::as_raw(self), pknown, pcodepage).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcBlobEncoding, ::windows_core::IUnknown, IDxcBlob);
unsafe impl ::windows_core::Interface for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcBlobEncoding {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7241d424_2646_4191_97c0_98e96e42fc68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_Vtbl {
    pub base__: IDxcBlob_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEncoding: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcBlobUtf16(::windows_core::IUnknown);
impl IDxcBlobUtf16 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows_core::Interface::vtable(self).base__.base__.GetBufferPointer)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows_core::Interface::vtable(self).base__.base__.GetBufferSize)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEncoding)(::windows_core::Interface::as_raw(self), pknown, pcodepage).ok()
    }
    pub unsafe fn GetStringPointer(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).GetStringPointer)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (::windows_core::Interface::vtable(self).GetStringLength)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IDxcBlobUtf16, ::windows_core::IUnknown, IDxcBlob, IDxcBlobEncoding);
unsafe impl ::windows_core::Interface for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcBlobUtf16 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3f84eab_0faa_497e_a39c_ee6ed60b2d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcBlobUtf8(::windows_core::IUnknown);
impl IDxcBlobUtf8 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows_core::Interface::vtable(self).base__.base__.GetBufferPointer)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows_core::Interface::vtable(self).base__.base__.GetBufferSize)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEncoding)(::windows_core::Interface::as_raw(self), pknown, pcodepage).ok()
    }
    pub unsafe fn GetStringPointer(&self) -> ::windows_core::PCSTR {
        (::windows_core::Interface::vtable(self).GetStringPointer)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (::windows_core::Interface::vtable(self).GetStringLength)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IDxcBlobUtf8, ::windows_core::IUnknown, IDxcBlob, IDxcBlobEncoding);
unsafe impl ::windows_core::Interface for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcBlobUtf8 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3da636c9_ba71_4024_a301_30cbf125305b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::PCSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcCompiler(::windows_core::IUnknown);
impl IDxcCompiler {
    pub unsafe fn Compile<P0, P1, P2, P3, P4>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P4) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<IDxcIncludeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Compile)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), pincludehandler.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Preprocess<P0, P1, P2>(&self, psource: P0, psourcename: P1, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P2) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<IDxcIncludeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Preprocess)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), pincludehandler.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Disassemble<P0>(&self, psource: P0) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Disassemble)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcCompiler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcCompiler {
    type Vtable = IDxcCompiler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcCompiler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c210bf3_011f_4422_8d70_6f9acb8db617);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcCompiler2(::windows_core::IUnknown);
impl IDxcCompiler2 {
    pub unsafe fn Compile<P0, P1, P2, P3, P4>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P4) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<IDxcIncludeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Compile)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), pincludehandler.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Preprocess<P0, P1, P2>(&self, psource: P0, psourcename: P1, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P2) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<IDxcIncludeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Preprocess)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), pincludehandler.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Disassemble<P0>(&self, psource: P0) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Disassemble)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CompileWithDebug<P0, P1, P2, P3, P4>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P4, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: ::core::option::Option<*mut ::windows_core::PWSTR>, ppdebugblob: ::core::option::Option<*mut ::core::option::Option<IDxcBlob>>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<IDxcIncludeHandler>,
    {
        (::windows_core::Interface::vtable(self).CompileWithDebug)(
            ::windows_core::Interface::as_raw(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            ::core::mem::transmute(pdefines.as_ptr()),
            pdefines.len().try_into().unwrap(),
            pincludehandler.into_param().abi(),
            ::core::mem::transmute(ppresult),
            ::core::mem::transmute(ppdebugblobname.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppdebugblob.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcCompiler2, ::windows_core::IUnknown, IDxcCompiler);
unsafe impl ::windows_core::Interface for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcCompiler2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa005a9d9_b8bb_4594_b5c9_0e633bec4d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_Vtbl {
    pub base__: IDxcCompiler_Vtbl,
    pub CompileWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void, ppdebugblobname: *mut ::windows_core::PWSTR, ppdebugblob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcCompiler3(::windows_core::IUnknown);
impl IDxcCompiler3 {
    pub unsafe fn Compile<P0, T>(&self, psource: *const DxcBuffer, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>, pincludehandler: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<IDxcIncludeHandler>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Compile)(::windows_core::Interface::as_raw(self), psource, ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pincludehandler.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Disassemble<T>(&self, pobject: *const DxcBuffer) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).Disassemble)(::windows_core::Interface::as_raw(self), pobject, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcCompiler3, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcCompiler3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x228b4687_5a6a_4730_900c_9702b2203f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const ::windows_core::PCWSTR, argcount: u32, pincludehandler: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcCompilerArgs(::windows_core::IUnknown);
impl IDxcCompilerArgs {
    pub unsafe fn GetArguments(&self) -> *mut ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).GetArguments)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCount(&self) -> u32 {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddArguments(&self, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddArguments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn AddArgumentsUTF8(&self, parguments: ::core::option::Option<&[::windows_core::PCSTR]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddArgumentsUTF8)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn AddDefines(&self, pdefines: &[DxcDefine]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDefines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcCompilerArgs, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcCompilerArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73effe2a_70dc_45f8_9690_eff64c02429d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::windows_core::PCWSTR,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows_core::PCWSTR, argcount: u32) -> ::windows_core::HRESULT,
    pub AddArgumentsUTF8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows_core::PCSTR, argcount: u32) -> ::windows_core::HRESULT,
    pub AddDefines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcContainerBuilder(::windows_core::IUnknown);
impl IDxcContainerBuilder {
    pub unsafe fn Load<P0>(&self, pdxilcontainerheader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pdxilcontainerheader.into_param().abi()).ok()
    }
    pub unsafe fn AddPart<P0>(&self, fourcc: u32, psource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        (::windows_core::Interface::vtable(self).AddPart)(::windows_core::Interface::as_raw(self), fourcc, psource.into_param().abi()).ok()
    }
    pub unsafe fn RemovePart(&self, fourcc: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemovePart)(::windows_core::Interface::as_raw(self), fourcc).ok()
    }
    pub unsafe fn SerializeContainer(&self) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SerializeContainer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcContainerBuilder, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcContainerBuilder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x334b1f50_2292_4b35_99a1_25588d8c17fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdxilcontainerheader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32, psource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemovePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows_core::HRESULT,
    pub SerializeContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcContainerReflection(::windows_core::IUnknown);
impl IDxcContainerReflection {
    pub unsafe fn Load<P0>(&self, pcontainer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pcontainer.into_param().abi()).ok()
    }
    pub unsafe fn GetPartCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPartKind(&self, idx: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartKind)(::windows_core::Interface::as_raw(self), idx, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPartContent(&self, idx: u32) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartContent)(::windows_core::Interface::as_raw(self), idx, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFirstPartKind(&self, kind: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstPartKind)(::windows_core::Interface::as_raw(self), kind, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPartReflection(&self, idx: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPartReflection)(::windows_core::Interface::as_raw(self), idx, iid, ppvobject).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcContainerReflection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcContainerReflection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2c21b26_8350_4bdc_976a_331ce6f4c54c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontainer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows_core::HRESULT,
    pub GetPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows_core::HRESULT,
    pub GetPartContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows_core::HRESULT,
    pub GetPartReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcExtraOutputs(::windows_core::IUnknown);
impl IDxcExtraOutputs {
    pub unsafe fn GetOutputCount(&self) -> u32 {
        (::windows_core::Interface::vtable(self).GetOutputCount)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetOutput<T>(&self, uindex: u32, ppoutputtype: ::core::option::Option<*mut ::core::option::Option<IDxcBlobUtf16>>, ppoutputname: ::core::option::Option<*mut ::core::option::Option<IDxcBlobUtf16>>, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()>
    where
        T: ::windows_core::ComInterface,
    {
        (::windows_core::Interface::vtable(self).GetOutput)(::windows_core::Interface::as_raw(self), uindex, &<T as ::windows_core::ComInterface>::IID, result__ as *mut _ as *mut _, ::core::mem::transmute(ppoutputtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppoutputname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcExtraOutputs, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcExtraOutputs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x319b37a2_a5c2_494a_a5de_4801b2faf989);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcIncludeHandler(::windows_core::IUnknown);
impl IDxcIncludeHandler {
    pub unsafe fn LoadSource<P0>(&self, pfilename: P0) -> ::windows_core::Result<IDxcBlob>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoadSource)(::windows_core::Interface::as_raw(self), pfilename.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcIncludeHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcIncludeHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f61fc7d_950d_467f_b3e3_3c02fb49187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LoadSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, ppincludesource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcLibrary(::windows_core::IUnknown);
impl IDxcLibrary {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMalloc<P0>(&self, pmalloc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IMalloc>,
    {
        (::windows_core::Interface::vtable(self).SetMalloc)(::windows_core::Interface::as_raw(self), pmalloc.into_param().abi()).ok()
    }
    pub unsafe fn CreateBlobFromBlob<P0>(&self, pblob: P0, offset: u32, length: u32) -> ::windows_core::Result<IDxcBlob>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromBlob)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), offset, length, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBlobFromFile<P0>(&self, pfilename: P0, codepage: ::core::option::Option<*const DXC_CP>) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromFile)(::windows_core::Interface::as_raw(self), pfilename.into_param().abi(), ::core::mem::transmute(codepage.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobWithEncodingFromPinned)(::windows_core::Interface::as_raw(self), ptext, size, codepage, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobWithEncodingOnHeapCopy)(::windows_core::Interface::as_raw(self), ptext, size, codepage, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<P0>(&self, ptext: *const ::core::ffi::c_void, pimalloc: P0, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IMalloc>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobWithEncodingOnMalloc)(::windows_core::Interface::as_raw(self), ptext, pimalloc.into_param().abi(), size, codepage, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateIncludeHandler(&self) -> ::windows_core::Result<IDxcIncludeHandler> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateIncludeHandler)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStreamFromBlobReadOnly<P0>(&self, pblob: P0) -> ::windows_core::Result<super::super::super::System::Com::IStream>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStreamFromBlobReadOnly)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBlobAsUtf8<P0>(&self, pblob: P0) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf8)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBlobAsUtf16<P0>(&self, pblob: P0) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf16)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcLibrary, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcLibrary {
    type Vtable = IDxcLibrary_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcLibrary {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5204dc7_d18c_4c3c_bdfb_851673980fe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmalloc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMalloc: usize,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateBlobFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateBlobWithEncodingFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateBlobWithEncodingOnHeapCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlobWithEncodingOnMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlobWithEncodingOnMalloc: usize,
    pub CreateIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStreamFromBlobReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStreamFromBlobReadOnly: usize,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcLinker(::windows_core::IUnknown);
impl IDxcLinker {
    pub unsafe fn RegisterLibrary<P0, P1>(&self, plibname: P0, plib: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDxcBlob>,
    {
        (::windows_core::Interface::vtable(self).RegisterLibrary)(::windows_core::Interface::as_raw(self), plibname.into_param().abi(), plib.into_param().abi()).ok()
    }
    pub unsafe fn Link<P0, P1>(&self, pentryname: P0, ptargetprofile: P1, plibnames: &[::windows_core::PCWSTR], parguments: ::core::option::Option<&[::windows_core::PCWSTR]>) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Link)(::windows_core::Interface::as_raw(self), pentryname.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(plibnames.as_ptr()), plibnames.len().try_into().unwrap(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcLinker, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcLinker {
    type Vtable = IDxcLinker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcLinker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1b5be2a_62dd_4327_a1c2_42ac1e1e78e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibname: ::windows_core::PCWSTR, plib: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pentryname: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, plibnames: *const ::windows_core::PCWSTR, libcount: u32, parguments: *const ::windows_core::PCWSTR, argcount: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcOperationResult(::windows_core::IUnknown);
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetResult)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcOperationResult, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcOperationResult {
    type Vtable = IDxcOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcOperationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcedb484a_d4e9_445a_b991_ca21ca157dc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetErrorBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcOptimizer(::windows_core::IUnknown);
impl IDxcOptimizer {
    pub unsafe fn GetAvailablePassCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAvailablePassCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAvailablePass(&self, index: u32) -> ::windows_core::Result<IDxcOptimizerPass> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAvailablePass)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn RunOptimizer<P0>(&self, pblob: P0, ppoptions: &[::windows_core::PCWSTR], poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: ::core::option::Option<*mut ::core::option::Option<IDxcBlobEncoding>>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        (::windows_core::Interface::vtable(self).RunOptimizer)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(ppoptions.as_ptr()), ppoptions.len().try_into().unwrap(), ::core::mem::transmute(poutputmodule), ::core::mem::transmute(ppoutputtext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcOptimizer, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcOptimizer {
    type Vtable = IDxcOptimizer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcOptimizer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25740e2e_9cba_401b_9119_4fb42f39f270);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetAvailablePassCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetAvailablePass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RunOptimizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppoptions: *const ::windows_core::PCWSTR, optioncount: u32, poutputmodule: *mut *mut ::core::ffi::c_void, ppoutputtext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcOptimizerPass(::windows_core::IUnknown);
impl IDxcOptimizerPass {
    pub unsafe fn GetOptionName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOptionArgCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionArgCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOptionArgName(&self, argindex: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionArgName)(::windows_core::Interface::as_raw(self), argindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOptionArgDescription(&self, argindex: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionArgDescription)(::windows_core::Interface::as_raw(self), argindex, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcOptimizerPass, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcOptimizerPass {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetOptionArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetOptionArgName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetOptionArgDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcPdbUtils(::windows_core::IUnknown);
impl IDxcPdbUtils {
    pub unsafe fn Load<P0>(&self, ppdbordxil: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), ppdbordxil.into_param().abi()).ok()
    }
    pub unsafe fn GetSourceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSource(&self, uindex: u32) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSource)(::windows_core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceName(&self, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceName)(::windows_core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlagCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFlagCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlag(&self, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFlag)(::windows_core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArgCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetArgCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArg(&self, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetArg)(::windows_core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArgPairCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetArgPairCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArgPair(&self, uindex: u32, pname: *mut ::windows_core::BSTR, pvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetArgPair)(::windows_core::Interface::as_raw(self), uindex, ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetDefineCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefineCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefine(&self, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefine)(::windows_core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTargetProfile(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetProfile)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEntryPoint(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEntryPoint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMainFileName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMainFileName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetHash(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHash)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullPDB(&self) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).IsFullPDB)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFullPDB(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFullPDB)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersionInfo(&self) -> ::windows_core::Result<IDxcVersionInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVersionInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCompiler<P0>(&self, pcompiler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcCompiler3>,
    {
        (::windows_core::Interface::vtable(self).SetCompiler)(::windows_core::Interface::as_raw(self), pcompiler.into_param().abi()).ok()
    }
    pub unsafe fn CompileForFullPDB(&self) -> ::windows_core::Result<IDxcResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CompileForFullPDB)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OverrideArgs)(::windows_core::Interface::as_raw(self), pargpairs, unumargpairs).ok()
    }
    pub unsafe fn OverrideRootSignature<P0>(&self, prootsignature: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OverrideRootSignature)(::windows_core::Interface::as_raw(self), prootsignature.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcPdbUtils, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcPdbUtils {
    type Vtable = IDxcPdbUtils_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcPdbUtils {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6c9647e_9d6a_4c3b_b94c_524b5a6c343d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbordxil: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetFlagCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetArg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetArgPairCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetArgPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetDefineCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetDefine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetTargetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetMainFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFullPDB: usize,
    pub GetFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfullpdb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppversioninfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCompiler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompiler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CompileForFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OverrideArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows_core::HRESULT,
    pub OverrideRootSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prootsignature: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcResult(::windows_core::IUnknown);
impl IDxcResult {
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetResult)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetErrorBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).HasOutput)(::windows_core::Interface::as_raw(self), dxcoutkind)
    }
    pub unsafe fn GetOutput<T>(&self, dxcoutkind: DXC_OUT_KIND, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()>
    where
        T: ::windows_core::ComInterface,
    {
        (::windows_core::Interface::vtable(self).GetOutput)(::windows_core::Interface::as_raw(self), dxcoutkind, &<T as ::windows_core::ComInterface>::IID, result__ as *mut _ as *mut _, ::core::mem::transmute(ppoutputname)).ok()
    }
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        (::windows_core::Interface::vtable(self).GetNumOutputs)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        (::windows_core::Interface::vtable(self).GetOutputByIndex)(::windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        (::windows_core::Interface::vtable(self).PrimaryOutput)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IDxcResult, ::windows_core::IUnknown, IDxcOperationResult);
unsafe impl ::windows_core::Interface for IDxcResult {
    type Vtable = IDxcResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58346cda_dde7_4497_9461_6f87af5e0659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_Vtbl {
    pub base__: IDxcOperationResult_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HasOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasOutput: usize,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNumOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutputByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND,
    pub PrimaryOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcUtils(::windows_core::IUnknown);
impl IDxcUtils {
    pub unsafe fn CreateBlobFromBlob<P0>(&self, pblob: P0, offset: u32, length: u32) -> ::windows_core::Result<IDxcBlob>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromBlob)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), offset, length, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBlobFromPinned(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromPinned)(::windows_core::Interface::as_raw(self), pdata, size, codepage, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveToBlob<P0>(&self, pdata: *const ::core::ffi::c_void, pimalloc: P0, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IMalloc>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveToBlob)(::windows_core::Interface::as_raw(self), pdata, pimalloc.into_param().abi(), size, codepage, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBlob(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlob)(::windows_core::Interface::as_raw(self), pdata, size, codepage, &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadFile<P0>(&self, pfilename: P0, pcodepage: ::core::option::Option<*const DXC_CP>) -> ::windows_core::Result<IDxcBlobEncoding>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoadFile)(::windows_core::Interface::as_raw(self), pfilename.into_param().abi(), ::core::mem::transmute(pcodepage.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamFromBlob<P0>(&self, pblob: P0) -> ::windows_core::Result<super::super::super::System::Com::IStream>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateReadOnlyStreamFromBlob)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> ::windows_core::Result<IDxcIncludeHandler> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDefaultIncludeHandler)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBlobAsUtf8<P0>(&self, pblob: P0) -> ::windows_core::Result<IDxcBlobUtf8>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf8)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBlobAsUtf16<P0>(&self, pblob: P0) -> ::windows_core::Result<IDxcBlobUtf16>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf16)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDxilContainerPart)(::windows_core::Interface::as_raw(self), pshader, dxcpart, pppartdata, ppartsizeinbytes).ok()
    }
    pub unsafe fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const ::windows_core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateReflection)(::windows_core::Interface::as_raw(self), pdata, iid, ppvreflection).ok()
    }
    pub unsafe fn BuildArguments<P0, P1, P2>(&self, psourcename: P0, pentrypoint: P1, ptargetprofile: P2, parguments: ::core::option::Option<&[::windows_core::PCWSTR]>, pdefines: &[DxcDefine]) -> ::windows_core::Result<IDxcCompilerArgs>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BuildArguments)(::windows_core::Interface::as_raw(self), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPDBContents<P0>(&self, ppdbblob: P0, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        (::windows_core::Interface::vtable(self).GetPDBContents)(::windows_core::Interface::as_raw(self), ppdbblob.into_param().abi(), ::core::mem::transmute(pphash), ::core::mem::transmute(ppcontainer)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcUtils, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcUtils {
    type Vtable = IDxcUtils_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcUtils {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4605c4cb_2019_492a_ada4_65f20bb7d67f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateBlobFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveToBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveToBlob: usize,
    pub CreateBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LoadFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamFromBlob: usize,
    pub CreateDefaultIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDxilContainerPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows_core::HRESULT,
    pub CreateReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows_core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BuildArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPDBContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbblob: *mut ::core::ffi::c_void, pphash: *mut *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcValidator(::windows_core::IUnknown);
impl IDxcValidator {
    pub unsafe fn Validate<P0>(&self, pshader: P0, flags: u32) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcValidator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcValidator {
    type Vtable = IDxcValidator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcValidator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6e82bd2_1fd7_4826_9811_2857e797f49a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcValidator2(::windows_core::IUnknown);
impl IDxcValidator2 {
    pub unsafe fn Validate<P0>(&self, pshader: P0, flags: u32) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Validate)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn ValidateWithDebug<P0>(&self, pshader: P0, flags: u32, poptdebugbitcode: ::core::option::Option<*const DxcBuffer>) -> ::windows_core::Result<IDxcOperationResult>
    where
        P0: ::windows_core::IntoParam<IDxcBlob>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ValidateWithDebug)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), flags, ::core::mem::transmute(poptdebugbitcode.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcValidator2, ::windows_core::IUnknown, IDxcValidator);
unsafe impl ::windows_core::Interface for IDxcValidator2 {
    type Vtable = IDxcValidator2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcValidator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x458e1fd1_b1b2_4750_a6e1_9c10f03bed92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_Vtbl {
    pub base__: IDxcValidator_Vtbl,
    pub ValidateWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcVersionInfo(::windows_core::IUnknown);
impl IDxcVersionInfo {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), pmajor, pminor).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcVersionInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcVersionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb04f5b50_2059_4f12_a8ff_a1e0cde1cc7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcVersionInfo2(::windows_core::IUnknown);
impl IDxcVersionInfo2 {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), pmajor, pminor).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCommitInfo)(::windows_core::Interface::as_raw(self), pcommitcount, pcommithash).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDxcVersionInfo2, ::windows_core::IUnknown, IDxcVersionInfo);
unsafe impl ::windows_core::Interface for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcVersionInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb6904c4_42f0_4b62_9c46_983af7da7c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_Vtbl {
    pub base__: IDxcVersionInfo_Vtbl,
    pub GetCommitInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDxcVersionInfo3(::windows_core::IUnknown);
impl IDxcVersionInfo3 {
    pub unsafe fn GetCustomVersionString(&self) -> ::windows_core::Result<*mut i8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomVersionString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDxcVersionInfo3, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDxcVersionInfo3 {
    type Vtable = IDxcVersionInfo3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDxcVersionInfo3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e13e843_9d25_473c_9ad2_03b2d0b44b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCustomVersionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows_core::HRESULT,
}
pub const CLSID_DxcAssembler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcValidator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
pub const DXC_ARG_ALL_RESOURCES_BOUND: ::windows_core::PCWSTR = ::windows_core::w!("-all_resources_bound");
pub const DXC_ARG_AVOID_FLOW_CONTROL: ::windows_core::PCWSTR = ::windows_core::w!("-Gfa");
pub const DXC_ARG_DEBUG: ::windows_core::PCWSTR = ::windows_core::w!("-Zi");
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: ::windows_core::PCWSTR = ::windows_core::w!("-Zsb");
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: ::windows_core::PCWSTR = ::windows_core::w!("-Zss");
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: ::windows_core::PCWSTR = ::windows_core::w!("-Gec");
pub const DXC_ARG_ENABLE_STRICTNESS: ::windows_core::PCWSTR = ::windows_core::w!("-Ges");
pub const DXC_ARG_IEEE_STRICTNESS: ::windows_core::PCWSTR = ::windows_core::w!("-Gis");
pub const DXC_ARG_OPTIMIZATION_LEVEL0: ::windows_core::PCWSTR = ::windows_core::w!("-O0");
pub const DXC_ARG_OPTIMIZATION_LEVEL1: ::windows_core::PCWSTR = ::windows_core::w!("-O1");
pub const DXC_ARG_OPTIMIZATION_LEVEL2: ::windows_core::PCWSTR = ::windows_core::w!("-O2");
pub const DXC_ARG_OPTIMIZATION_LEVEL3: ::windows_core::PCWSTR = ::windows_core::w!("-O3");
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: ::windows_core::PCWSTR = ::windows_core::w!("-Zpc");
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: ::windows_core::PCWSTR = ::windows_core::w!("-Zpr");
pub const DXC_ARG_PREFER_FLOW_CONTROL: ::windows_core::PCWSTR = ::windows_core::w!("-Gfp");
pub const DXC_ARG_RESOURCES_MAY_ALIAS: ::windows_core::PCWSTR = ::windows_core::w!("-res_may_alias");
pub const DXC_ARG_SKIP_OPTIMIZATIONS: ::windows_core::PCWSTR = ::windows_core::w!("-Od");
pub const DXC_ARG_SKIP_VALIDATION: ::windows_core::PCWSTR = ::windows_core::w!("-Vd");
pub const DXC_ARG_WARNINGS_ARE_ERRORS: ::windows_core::PCWSTR = ::windows_core::w!("-WX");
pub const DXC_CP_ACP: DXC_CP = DXC_CP(0u32);
pub const DXC_CP_UTF16: DXC_CP = DXC_CP(1200u32);
pub const DXC_CP_UTF8: DXC_CP = DXC_CP(65001u32);
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: ::windows_core::PCWSTR = ::windows_core::w!("*stderr*");
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: ::windows_core::PCWSTR = ::windows_core::w!("*stdout*");
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = DXC_OUT_KIND(5i32);
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = DXC_OUT_KIND(2i32);
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = DXC_OUT_KIND(10i32);
pub const DXC_OUT_HLSL: DXC_OUT_KIND = DXC_OUT_KIND(6i32);
pub const DXC_OUT_NONE: DXC_OUT_KIND = DXC_OUT_KIND(0i32);
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = DXC_OUT_KIND(1i32);
pub const DXC_OUT_PDB: DXC_OUT_KIND = DXC_OUT_KIND(3i32);
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = DXC_OUT_KIND(8i32);
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = DXC_OUT_KIND(9i32);
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = DXC_OUT_KIND(4i32);
pub const DXC_OUT_TEXT: DXC_OUT_KIND = DXC_OUT_KIND(7i32);
pub const DxcValidatorFlags_Default: u32 = 0u32;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXC_CP(pub u32);
impl ::core::marker::Copy for DXC_CP {}
impl ::core::clone::Clone for DXC_CP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXC_CP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DXC_CP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXC_CP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_CP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXC_OUT_KIND(pub i32);
impl ::core::marker::Copy for DXC_OUT_KIND {}
impl ::core::clone::Clone for DXC_OUT_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXC_OUT_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DXC_OUT_KIND {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXC_OUT_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_OUT_KIND").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DxcArgPair {
    pub pName: ::windows_core::PCWSTR,
    pub pValue: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for DxcArgPair {}
impl ::core::clone::Clone for DxcArgPair {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcArgPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcArgPair").field("pName", &self.pName).field("pValue", &self.pValue).finish()
    }
}
impl ::windows_core::TypeKind for DxcArgPair {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DxcArgPair {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName && self.pValue == other.pValue
    }
}
impl ::core::cmp::Eq for DxcArgPair {}
impl ::core::default::Default for DxcArgPair {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DxcBuffer {
    pub Ptr: *const ::core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl ::core::marker::Copy for DxcBuffer {}
impl ::core::clone::Clone for DxcBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcBuffer").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Encoding", &self.Encoding).finish()
    }
}
impl ::windows_core::TypeKind for DxcBuffer {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr && self.Size == other.Size && self.Encoding == other.Encoding
    }
}
impl ::core::cmp::Eq for DxcBuffer {}
impl ::core::default::Default for DxcBuffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DxcDefine {
    pub Name: ::windows_core::PCWSTR,
    pub Value: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for DxcDefine {}
impl ::core::clone::Clone for DxcDefine {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcDefine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcDefine").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
impl ::windows_core::TypeKind for DxcDefine {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DxcDefine {}
impl ::core::default::Default for DxcDefine {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl ::core::marker::Copy for DxcShaderHash {}
impl ::core::clone::Clone for DxcShaderHash {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcShaderHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcShaderHash").field("Flags", &self.Flags).field("HashDigest", &self.HashDigest).finish()
    }
}
impl ::windows_core::TypeKind for DxcShaderHash {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.HashDigest == other.HashDigest
    }
}
impl ::core::cmp::Eq for DxcShaderHash {}
impl ::core::default::Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = ::core::option::Option<unsafe extern "system" fn(pmalloc: ::core::option::Option<super::super::super::System::Com::IMalloc>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type DxcCreateInstanceProc = ::core::option::Option<unsafe extern "system" fn(rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
