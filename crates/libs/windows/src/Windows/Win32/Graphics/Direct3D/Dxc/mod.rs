#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[inline]
pub unsafe fn DxcCreateInstance<T>(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
where
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DxcCreateInstance(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    DxcCreateInstance(::core::mem::transmute(rclsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DxcCreateInstance2<'a, P0, T>(pmalloc: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IMalloc>>,
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DxcCreateInstance2(pmalloc: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    DxcCreateInstance2(pmalloc.into().abi(), ::core::mem::transmute(rclsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcAssembler(::windows::core::IUnknown);
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<'a, P0>(&self, pshader: P0) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AssembleToContainer)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcAssembler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcAssembler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcAssembler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcAssembler {}
impl ::core::fmt::Debug for IDxcAssembler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcAssembler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcAssembler {
    type Vtable = IDxcAssembler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcAssembler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x091f7a26_1c1f_4948_904b_e6e3a8a771d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AssembleToContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlob(::windows::core::IUnknown);
impl IDxcBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).GetBufferPointer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::core::Vtable::vtable(self).GetBufferSize)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IDxcBlob, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcBlob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlob {}
impl ::core::fmt::Debug for IDxcBlob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcBlob {
    type Vtable = IDxcBlob_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcBlob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBufferPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlobEncoding(::windows::core::IUnknown);
impl IDxcBlobEncoding {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).base__.GetBufferPointer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::core::Vtable::vtable(self).base__.GetBufferSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetEncoding)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcBlobEncoding, ::windows::core::IUnknown, IDxcBlob);
impl ::core::clone::Clone for IDxcBlobEncoding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobEncoding {}
impl ::core::fmt::Debug for IDxcBlobEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcBlobEncoding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7241d424_2646_4191_97c0_98e96e42fc68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_Vtbl {
    pub base__: IDxcBlob_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEncoding: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlobUtf16(::windows::core::IUnknown);
impl IDxcBlobUtf16 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferPointer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEncoding)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    pub unsafe fn GetStringPointer(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).GetStringPointer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (::windows::core::Vtable::vtable(self).GetStringLength)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IDxcBlobUtf16, ::windows::core::IUnknown, IDxcBlob, IDxcBlobEncoding);
impl ::core::clone::Clone for IDxcBlobUtf16 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobUtf16 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobUtf16 {}
impl ::core::fmt::Debug for IDxcBlobUtf16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobUtf16").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcBlobUtf16 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3f84eab_0faa_497e_a39c_ee6ed60b2d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlobUtf8(::windows::core::IUnknown);
impl IDxcBlobUtf8 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferPointer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEncoding)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    pub unsafe fn GetStringPointer(&self) -> ::windows::core::PSTR {
        (::windows::core::Vtable::vtable(self).GetStringPointer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (::windows::core::Vtable::vtable(self).GetStringLength)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IDxcBlobUtf8, ::windows::core::IUnknown, IDxcBlob, IDxcBlobEncoding);
impl ::core::clone::Clone for IDxcBlobUtf8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobUtf8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobUtf8 {}
impl ::core::fmt::Debug for IDxcBlobUtf8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobUtf8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcBlobUtf8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da636c9_ba71_4024_a301_30cbf125305b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompiler(::windows::core::IUnknown);
impl IDxcCompiler {
    pub unsafe fn Compile<'a, P0, P1, P2, P3, P4>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: ::core::option::Option<&[::windows::core::PWSTR]>, pdefines: &[DxcDefine], pincludehandler: P4) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, IDxcIncludeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Compile)(::windows::core::Vtable::as_raw(self), psource.into().abi(), psourcename.into(), pentrypoint.into(), ptargetprofile.into(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len() as _, pincludehandler.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Preprocess<'a, P0, P1, P2>(&self, psource: P0, psourcename: P1, parguments: ::core::option::Option<&[::windows::core::PWSTR]>, pdefines: &[DxcDefine], pincludehandler: P2) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IDxcIncludeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Preprocess)(::windows::core::Vtable::as_raw(self), psource.into().abi(), psourcename.into(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len() as _, pincludehandler.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Disassemble<'a, P0>(&self, psource: P0) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Disassemble)(::windows::core::Vtable::as_raw(self), psource.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcCompiler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcCompiler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler {}
impl ::core::fmt::Debug for IDxcCompiler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcCompiler {
    type Vtable = IDxcCompiler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcCompiler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c210bf3_011f_4422_8d70_6f9acb8db617);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompiler2(::windows::core::IUnknown);
impl IDxcCompiler2 {
    pub unsafe fn Compile<'a, P0, P1, P2, P3, P4>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: ::core::option::Option<&[::windows::core::PWSTR]>, pdefines: &[DxcDefine], pincludehandler: P4) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, IDxcIncludeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compile)(::windows::core::Vtable::as_raw(self), psource.into().abi(), psourcename.into(), pentrypoint.into(), ptargetprofile.into(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len() as _, pincludehandler.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Preprocess<'a, P0, P1, P2>(&self, psource: P0, psourcename: P1, parguments: ::core::option::Option<&[::windows::core::PWSTR]>, pdefines: &[DxcDefine], pincludehandler: P2) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IDxcIncludeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Preprocess)(::windows::core::Vtable::as_raw(self), psource.into().abi(), psourcename.into(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len() as _, pincludehandler.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Disassemble<'a, P0>(&self, psource: P0) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Disassemble)(::windows::core::Vtable::as_raw(self), psource.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CompileWithDebug<'a, P0, P1, P2, P3, P4>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: ::core::option::Option<&[::windows::core::PWSTR]>, pdefines: &[DxcDefine], pincludehandler: P4, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: ::core::option::Option<*mut ::windows::core::PWSTR>, ppdebugblob: ::core::option::Option<*mut ::core::option::Option<IDxcBlob>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, IDxcIncludeHandler>>,
    {
        (::windows::core::Vtable::vtable(self).CompileWithDebug)(
            ::windows::core::Vtable::as_raw(self),
            psource.into().abi(),
            psourcename.into(),
            pentrypoint.into(),
            ptargetprofile.into(),
            ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            parguments.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pdefines.as_ptr()),
            pdefines.len() as _,
            pincludehandler.into().abi(),
            ::core::mem::transmute(ppresult),
            ::core::mem::transmute(ppdebugblobname.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppdebugblob.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IDxcCompiler2, ::windows::core::IUnknown, IDxcCompiler);
impl ::core::clone::Clone for IDxcCompiler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler2 {}
impl ::core::fmt::Debug for IDxcCompiler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcCompiler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa005a9d9_b8bb_4594_b5c9_0e633bec4d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_Vtbl {
    pub base__: IDxcCompiler_Vtbl,
    pub CompileWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void, ppdebugblobname: *mut ::windows::core::PWSTR, ppdebugblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompiler3(::windows::core::IUnknown);
impl IDxcCompiler3 {
    pub unsafe fn Compile<'a, P0>(&self, psource: *const DxcBuffer, parguments: ::core::option::Option<&[::windows::core::PWSTR]>, pincludehandler: P0, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcIncludeHandler>>,
    {
        (::windows::core::Vtable::vtable(self).Compile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psource), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _), pincludehandler.into().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
    pub unsafe fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Disassemble)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pobject), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcCompiler3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcCompiler3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler3 {}
impl ::core::fmt::Debug for IDxcCompiler3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcCompiler3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x228b4687_5a6a_4730_900c_9702b2203f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const ::windows::core::PWSTR, argcount: u32, pincludehandler: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompilerArgs(::windows::core::IUnknown);
impl IDxcCompilerArgs {
    pub unsafe fn GetArguments(&self) -> *mut ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).GetArguments)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AddArguments(&self, parguments: ::core::option::Option<&[::windows::core::PWSTR]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddArguments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn AddArgumentsUTF8(&self, parguments: ::core::option::Option<&[::windows::core::PSTR]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddArgumentsUTF8)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn AddDefines(&self, pdefines: &[DxcDefine]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddDefines)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcCompilerArgs, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcCompilerArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompilerArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompilerArgs {}
impl ::core::fmt::Debug for IDxcCompilerArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompilerArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcCompilerArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73effe2a_70dc_45f8_9690_eff64c02429d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::windows::core::PWSTR,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows::core::PWSTR, argcount: u32) -> ::windows::core::HRESULT,
    pub AddArgumentsUTF8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows::core::PSTR, argcount: u32) -> ::windows::core::HRESULT,
    pub AddDefines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcContainerBuilder(::windows::core::IUnknown);
impl IDxcContainerBuilder {
    pub unsafe fn Load<'a, P0>(&self, pdxilcontainerheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), pdxilcontainerheader.into().abi()).ok()
    }
    pub unsafe fn AddPart<'a, P0>(&self, fourcc: u32, psource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        (::windows::core::Vtable::vtable(self).AddPart)(::windows::core::Vtable::as_raw(self), fourcc, psource.into().abi()).ok()
    }
    pub unsafe fn RemovePart(&self, fourcc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemovePart)(::windows::core::Vtable::as_raw(self), fourcc).ok()
    }
    pub unsafe fn SerializeContainer(&self) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SerializeContainer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcContainerBuilder, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcContainerBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcContainerBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcContainerBuilder {}
impl ::core::fmt::Debug for IDxcContainerBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcContainerBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcContainerBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334b1f50_2292_4b35_99a1_25588d8c17fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdxilcontainerheader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32, psource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemovePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows::core::HRESULT,
    pub SerializeContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcContainerReflection(::windows::core::IUnknown);
impl IDxcContainerReflection {
    pub unsafe fn Load<'a, P0>(&self, pcontainer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), pcontainer.into().abi()).ok()
    }
    pub unsafe fn GetPartCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartKind(&self, idx: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartKind)(::windows::core::Vtable::as_raw(self), idx, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartContent(&self, idx: u32) -> ::windows::core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartContent)(::windows::core::Vtable::as_raw(self), idx, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn FindFirstPartKind(&self, kind: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstPartKind)(::windows::core::Vtable::as_raw(self), kind, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartReflection(&self, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPartReflection)(::windows::core::Vtable::as_raw(self), idx, ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcContainerReflection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcContainerReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcContainerReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcContainerReflection {}
impl ::core::fmt::Debug for IDxcContainerReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcContainerReflection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcContainerReflection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2c21b26_8350_4bdc_976a_331ce6f4c54c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontainer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows::core::HRESULT,
    pub GetPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows::core::HRESULT,
    pub GetPartContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows::core::HRESULT,
    pub GetPartReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcExtraOutputs(::windows::core::IUnknown);
impl IDxcExtraOutputs {
    pub unsafe fn GetOutputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetOutputCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetOutput<T>(&self, uindex: u32, ppoutputtype: ::core::option::Option<*mut ::core::option::Option<IDxcBlobUtf16>>, ppoutputname: ::core::option::Option<*mut ::core::option::Option<IDxcBlobUtf16>>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetOutput)(::windows::core::Vtable::as_raw(self), uindex, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _, ::core::mem::transmute(ppoutputtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppoutputname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcExtraOutputs, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcExtraOutputs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcExtraOutputs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcExtraOutputs {}
impl ::core::fmt::Debug for IDxcExtraOutputs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcExtraOutputs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcExtraOutputs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x319b37a2_a5c2_494a_a5de_4801b2faf989);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcIncludeHandler(::windows::core::IUnknown);
impl IDxcIncludeHandler {
    pub unsafe fn LoadSource<'a, P0>(&self, pfilename: P0) -> ::windows::core::Result<IDxcBlob>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadSource)(::windows::core::Vtable::as_raw(self), pfilename.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcIncludeHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcIncludeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcIncludeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcIncludeHandler {}
impl ::core::fmt::Debug for IDxcIncludeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcIncludeHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcIncludeHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f61fc7d_950d_467f_b3e3_3c02fb49187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LoadSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, ppincludesource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcLibrary(::windows::core::IUnknown);
impl IDxcLibrary {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMalloc<'a, P0>(&self, pmalloc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IMalloc>>,
    {
        (::windows::core::Vtable::vtable(self).SetMalloc)(::windows::core::Vtable::as_raw(self), pmalloc.into().abi()).ok()
    }
    pub unsafe fn CreateBlobFromBlob<'a, P0>(&self, pblob: P0, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlobFromBlob)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), offset, length, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn CreateBlobFromFile<'a, P0>(&self, pfilename: P0, codepage: ::core::option::Option<*const DXC_CP>) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlobFromFile)(::windows::core::Vtable::as_raw(self), pfilename.into(), ::core::mem::transmute(codepage.unwrap_or(::std::ptr::null())), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlobWithEncodingFromPinned)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ptext), size, codepage, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlobWithEncodingOnHeapCopy)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ptext), size, codepage, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<'a, P0>(&self, ptext: *const ::core::ffi::c_void, pimalloc: P0, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IMalloc>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlobWithEncodingOnMalloc)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ptext), pimalloc.into().abi(), size, codepage, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateIncludeHandler)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcIncludeHandler>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStreamFromBlobReadOnly<'a, P0>(&self, pblob: P0) -> ::windows::core::Result<super::super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStreamFromBlobReadOnly)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn GetBlobAsUtf8<'a, P0>(&self, pblob: P0) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBlobAsUtf8)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn GetBlobAsUtf16<'a, P0>(&self, pblob: P0) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBlobAsUtf16)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcLibrary, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcLibrary {}
impl ::core::fmt::Debug for IDxcLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcLibrary {
    type Vtable = IDxcLibrary_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcLibrary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5204dc7_d18c_4c3c_bdfb_851673980fe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmalloc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMalloc: usize,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBlobFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBlobWithEncodingFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBlobWithEncodingOnHeapCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlobWithEncodingOnMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlobWithEncodingOnMalloc: usize,
    pub CreateIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStreamFromBlobReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStreamFromBlobReadOnly: usize,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcLinker(::windows::core::IUnknown);
impl IDxcLinker {
    pub unsafe fn RegisterLibrary<'a, P0, P1>(&self, plibname: P0, plib: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterLibrary)(::windows::core::Vtable::as_raw(self), plibname.into(), plib.into().abi()).ok()
    }
    pub unsafe fn Link<'a, P0, P1>(&self, pentryname: P0, ptargetprofile: P1, plibnames: &[::windows::core::PWSTR], parguments: ::core::option::Option<&[::windows::core::PWSTR]>) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Link)(::windows::core::Vtable::as_raw(self), pentryname.into(), ptargetprofile.into(), ::core::mem::transmute(plibnames.as_ptr()), plibnames.len() as _, ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcLinker, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcLinker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcLinker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcLinker {}
impl ::core::fmt::Debug for IDxcLinker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcLinker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcLinker {
    type Vtable = IDxcLinker_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcLinker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1b5be2a_62dd_4327_a1c2_42ac1e1e78e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibname: ::windows::core::PCWSTR, plib: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pentryname: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, plibnames: *const ::windows::core::PWSTR, libcount: u32, parguments: *const ::windows::core::PWSTR, argcount: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcOperationResult(::windows::core::IUnknown);
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetResult)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetErrorBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcOperationResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOperationResult {}
impl ::core::fmt::Debug for IDxcOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcOperationResult {
    type Vtable = IDxcOperationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcOperationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcedb484a_d4e9_445a_b991_ca21ca157dc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetErrorBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcOptimizer(::windows::core::IUnknown);
impl IDxcOptimizer {
    pub unsafe fn GetAvailablePassCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAvailablePassCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAvailablePass(&self, index: u32) -> ::windows::core::Result<IDxcOptimizerPass> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAvailablePass)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOptimizerPass>(result__)
    }
    pub unsafe fn RunOptimizer<'a, P0>(&self, pblob: P0, ppoptions: &[::windows::core::PWSTR], poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: ::core::option::Option<*mut ::core::option::Option<IDxcBlobEncoding>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        (::windows::core::Vtable::vtable(self).RunOptimizer)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), ::core::mem::transmute(ppoptions.as_ptr()), ppoptions.len() as _, ::core::mem::transmute(poutputmodule), ::core::mem::transmute(ppoutputtext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcOptimizer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcOptimizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOptimizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOptimizer {}
impl ::core::fmt::Debug for IDxcOptimizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOptimizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcOptimizer {
    type Vtable = IDxcOptimizer_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcOptimizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25740e2e_9cba_401b_9119_4fb42f39f270);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAvailablePassCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetAvailablePass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RunOptimizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppoptions: *const ::windows::core::PWSTR, optioncount: u32, poutputmodule: *mut *mut ::core::ffi::c_void, ppoutputtext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcOptimizerPass(::windows::core::IUnknown);
impl IDxcOptimizerPass {
    pub unsafe fn GetOptionName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptionName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetOptionArgCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptionArgCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptionArgName(&self, argindex: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptionArgName)(::windows::core::Vtable::as_raw(self), argindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetOptionArgDescription(&self, argindex: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptionArgDescription)(::windows::core::Vtable::as_raw(self), argindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcOptimizerPass, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcOptimizerPass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOptimizerPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOptimizerPass {}
impl ::core::fmt::Debug for IDxcOptimizerPass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOptimizerPass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcOptimizerPass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetOptionArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetOptionArgName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetOptionArgDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcPdbUtils(::windows::core::IUnknown);
impl IDxcPdbUtils {
    pub unsafe fn Load<'a, P0>(&self, ppdbordxil: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), ppdbordxil.into().abi()).ok()
    }
    pub unsafe fn GetSourceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSource(&self, uindex: u32) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSource)(::windows::core::Vtable::as_raw(self), uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn GetSourceName(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceName)(::windows::core::Vtable::as_raw(self), uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetFlagCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlagCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFlag(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlag)(::windows::core::Vtable::as_raw(self), uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetArgCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetArgCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetArg(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetArg)(::windows::core::Vtable::as_raw(self), uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetArgPairCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetArgPairCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetArgPair(&self, uindex: u32, pname: *mut ::windows::core::BSTR, pvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetArgPair)(::windows::core::Vtable::as_raw(self), uindex, ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetDefineCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefineCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDefine(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefine)(::windows::core::Vtable::as_raw(self), uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetTargetProfile(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTargetProfile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetEntryPoint(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEntryPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetMainFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMainFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetHash(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHash)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullPDB(&self) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsFullPDB)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFullPDB(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFullPDB)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetVersionInfo(&self) -> ::windows::core::Result<IDxcVersionInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVersionInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcVersionInfo>(result__)
    }
    pub unsafe fn SetCompiler<'a, P0>(&self, pcompiler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcCompiler3>>,
    {
        (::windows::core::Vtable::vtable(self).SetCompiler)(::windows::core::Vtable::as_raw(self), pcompiler.into().abi()).ok()
    }
    pub unsafe fn CompileForFullPDB(&self) -> ::windows::core::Result<IDxcResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompileForFullPDB)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcResult>(result__)
    }
    pub unsafe fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OverrideArgs)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pargpairs), unumargpairs).ok()
    }
    pub unsafe fn OverrideRootSignature<'a, P0>(&self, prootsignature: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).OverrideRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcPdbUtils, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcPdbUtils {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcPdbUtils {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcPdbUtils {}
impl ::core::fmt::Debug for IDxcPdbUtils {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcPdbUtils").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcPdbUtils {
    type Vtable = IDxcPdbUtils_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcPdbUtils {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6c9647e_9d6a_4c3b_b94c_524b5a6c343d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbordxil: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetFlagCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetArg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetArgPairCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetArgPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, pvalue: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDefineCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetDefine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetTargetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetMainFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFullPDB: usize,
    pub GetFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfullpdb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppversioninfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompiler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompiler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompileForFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OverrideArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::HRESULT,
    pub OverrideRootSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcResult(::windows::core::IUnknown);
impl IDxcResult {
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResult)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetErrorBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).HasOutput)(::windows::core::Vtable::as_raw(self), dxcoutkind)
    }
    pub unsafe fn GetOutput<T>(&self, dxcoutkind: DXC_OUT_KIND, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetOutput)(::windows::core::Vtable::as_raw(self), dxcoutkind, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _, ::core::mem::transmute(ppoutputname)).ok()
    }
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetNumOutputs)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        (::windows::core::Vtable::vtable(self).GetOutputByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        (::windows::core::Vtable::vtable(self).PrimaryOutput)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IDxcResult, ::windows::core::IUnknown, IDxcOperationResult);
impl ::core::clone::Clone for IDxcResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcResult {}
impl ::core::fmt::Debug for IDxcResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcResult {
    type Vtable = IDxcResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58346cda_dde7_4497_9461_6f87af5e0659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_Vtbl {
    pub base__: IDxcOperationResult_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HasOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasOutput: usize,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutputByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND,
    pub PrimaryOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcUtils(::windows::core::IUnknown);
impl IDxcUtils {
    pub unsafe fn CreateBlobFromBlob<'a, P0>(&self, pblob: P0, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlobFromBlob)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), offset, length, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn CreateBlobFromPinned(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlobFromPinned)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata), size, codepage, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveToBlob<'a, P0>(&self, pdata: *const ::core::ffi::c_void, pimalloc: P0, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IMalloc>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveToBlob)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata), pimalloc.into().abi(), size, codepage, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlob(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlob)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata), size, codepage, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn LoadFile<'a, P0>(&self, pfilename: P0, pcodepage: ::core::option::Option<*const DXC_CP>) -> ::windows::core::Result<IDxcBlobEncoding>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadFile)(::windows::core::Vtable::as_raw(self), pfilename.into(), ::core::mem::transmute(pcodepage.unwrap_or(::std::ptr::null())), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamFromBlob<'a, P0>(&self, pblob: P0) -> ::windows::core::Result<super::super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateReadOnlyStreamFromBlob)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDefaultIncludeHandler)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcIncludeHandler>(result__)
    }
    pub unsafe fn GetBlobAsUtf8<'a, P0>(&self, pblob: P0) -> ::windows::core::Result<IDxcBlobUtf8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBlobAsUtf8)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobUtf8>(result__)
    }
    pub unsafe fn GetBlobAsUtf16<'a, P0>(&self, pblob: P0) -> ::windows::core::Result<IDxcBlobUtf16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBlobAsUtf16)(::windows::core::Vtable::as_raw(self), pblob.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobUtf16>(result__)
    }
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDxilContainerPart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshader), dxcpart, ::core::mem::transmute(pppartdata), ::core::mem::transmute(ppartsizeinbytes)).ok()
    }
    pub unsafe fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CreateReflection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata), ::core::mem::transmute(iid), ::core::mem::transmute(ppvreflection)).ok()
    }
    pub unsafe fn BuildArguments<'a, P0, P1, P2>(&self, psourcename: P0, pentrypoint: P1, ptargetprofile: P2, parguments: ::core::option::Option<&[::windows::core::PWSTR]>, pdefines: &[DxcDefine]) -> ::windows::core::Result<IDxcCompilerArgs>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BuildArguments)(::windows::core::Vtable::as_raw(self), psourcename.into(), pentrypoint.into(), ptargetprofile.into(), ::core::mem::transmute(parguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pdefines.as_ptr()), pdefines.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcCompilerArgs>(result__)
    }
    pub unsafe fn GetPDBContents<'a, P0>(&self, ppdbblob: P0, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        (::windows::core::Vtable::vtable(self).GetPDBContents)(::windows::core::Vtable::as_raw(self), ppdbblob.into().abi(), ::core::mem::transmute(pphash), ::core::mem::transmute(ppcontainer)).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcUtils, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcUtils {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcUtils {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcUtils {}
impl ::core::fmt::Debug for IDxcUtils {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcUtils").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcUtils {
    type Vtable = IDxcUtils_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcUtils {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4605c4cb_2019_492a_ada4_65f20bb7d67f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBlobFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveToBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveToBlob: usize,
    pub CreateBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamFromBlob: usize,
    pub CreateDefaultIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDxilContainerPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::HRESULT,
    pub CreateReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BuildArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPDBContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbblob: *mut ::core::ffi::c_void, pphash: *mut *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcValidator(::windows::core::IUnknown);
impl IDxcValidator {
    pub unsafe fn Validate<'a, P0>(&self, pshader: P0, flags: u32) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Validate)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcValidator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcValidator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcValidator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcValidator {}
impl ::core::fmt::Debug for IDxcValidator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcValidator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcValidator {
    type Vtable = IDxcValidator_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcValidator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6e82bd2_1fd7_4826_9811_2857e797f49a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcValidator2(::windows::core::IUnknown);
impl IDxcValidator2 {
    pub unsafe fn Validate<'a, P0>(&self, pshader: P0, flags: u32) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Validate)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn ValidateWithDebug<'a, P0>(&self, pshader: P0, flags: u32, poptdebugbitcode: ::core::option::Option<*const DxcBuffer>) -> ::windows::core::Result<IDxcOperationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDxcBlob>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValidateWithDebug)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), flags, ::core::mem::transmute(poptdebugbitcode.unwrap_or(::std::ptr::null())), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcValidator2, ::windows::core::IUnknown, IDxcValidator);
impl ::core::clone::Clone for IDxcValidator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcValidator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcValidator2 {}
impl ::core::fmt::Debug for IDxcValidator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcValidator2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcValidator2 {
    type Vtable = IDxcValidator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcValidator2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458e1fd1_b1b2_4750_a6e1_9c10f03bed92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_Vtbl {
    pub base__: IDxcValidator_Vtbl,
    pub ValidateWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcVersionInfo(::windows::core::IUnknown);
impl IDxcVersionInfo {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcVersionInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo {}
impl ::core::fmt::Debug for IDxcVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb04f5b50_2059_4f12_a8ff_a1e0cde1cc7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcVersionInfo2(::windows::core::IUnknown);
impl IDxcVersionInfo2 {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFlags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCommitInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pcommitcount), ::core::mem::transmute(pcommithash)).ok()
    }
}
::windows::core::interface_hierarchy!(IDxcVersionInfo2, ::windows::core::IUnknown, IDxcVersionInfo);
impl ::core::clone::Clone for IDxcVersionInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo2 {}
impl ::core::fmt::Debug for IDxcVersionInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6904c4_42f0_4b62_9c46_983af7da7c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_Vtbl {
    pub base__: IDxcVersionInfo_Vtbl,
    pub GetCommitInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcVersionInfo3(::windows::core::IUnknown);
impl IDxcVersionInfo3 {
    pub unsafe fn GetCustomVersionString(&self) -> ::windows::core::Result<*mut i8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustomVersionString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut i8>(result__)
    }
}
::windows::core::interface_hierarchy!(IDxcVersionInfo3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDxcVersionInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo3 {}
impl ::core::fmt::Debug for IDxcVersionInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDxcVersionInfo3 {
    type Vtable = IDxcVersionInfo3_Vtbl;
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e13e843_9d25_473c_9ad2_03b2d0b44b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCustomVersionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows::core::HRESULT,
}
pub const CLSID_DxcAssembler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcValidator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ALL_RESOURCES_BOUND: ::windows::core::PCWSTR = ::windows::w!("-all_resources_bound");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_AVOID_FLOW_CONTROL: ::windows::core::PCWSTR = ::windows::w!("-Gfa");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG: ::windows::core::PCWSTR = ::windows::w!("-Zi");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: ::windows::core::PCWSTR = ::windows::w!("-Zsb");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: ::windows::core::PCWSTR = ::windows::w!("-Zss");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: ::windows::core::PCWSTR = ::windows::w!("-Gec");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ENABLE_STRICTNESS: ::windows::core::PCWSTR = ::windows::w!("-Ges");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_IEEE_STRICTNESS: ::windows::core::PCWSTR = ::windows::w!("-Gis");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL0: ::windows::core::PCWSTR = ::windows::w!("-O0");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL1: ::windows::core::PCWSTR = ::windows::w!("-O1");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL2: ::windows::core::PCWSTR = ::windows::w!("-O2");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL3: ::windows::core::PCWSTR = ::windows::w!("-O3");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: ::windows::core::PCWSTR = ::windows::w!("-Zpc");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: ::windows::core::PCWSTR = ::windows::w!("-Zpr");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PREFER_FLOW_CONTROL: ::windows::core::PCWSTR = ::windows::w!("-Gfp");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_RESOURCES_MAY_ALIAS: ::windows::core::PCWSTR = ::windows::w!("-res_may_alias");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_SKIP_OPTIMIZATIONS: ::windows::core::PCWSTR = ::windows::w!("-Od");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_SKIP_VALIDATION: ::windows::core::PCWSTR = ::windows::w!("-Vd");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_WARNINGS_ARE_ERRORS: ::windows::core::PCWSTR = ::windows::w!("-WX");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: ::windows::core::PCWSTR = ::windows::w!("*stderr*");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: ::windows::core::PCWSTR = ::windows::w!("*stdout*");
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_Default: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXC_CP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_ACP: DXC_CP = DXC_CP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_UTF16: DXC_CP = DXC_CP(1200u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_UTF8: DXC_CP = DXC_CP(65001u32);
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
unsafe impl ::windows::core::Abi for DXC_CP {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXC_CP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_CP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXC_OUT_KIND(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_NONE: DXC_OUT_KIND = DXC_OUT_KIND(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = DXC_OUT_KIND(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = DXC_OUT_KIND(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_PDB: DXC_OUT_KIND = DXC_OUT_KIND(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = DXC_OUT_KIND(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = DXC_OUT_KIND(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_HLSL: DXC_OUT_KIND = DXC_OUT_KIND(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_TEXT: DXC_OUT_KIND = DXC_OUT_KIND(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = DXC_OUT_KIND(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = DXC_OUT_KIND(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = DXC_OUT_KIND(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = DXC_OUT_KIND(-1i32);
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
unsafe impl ::windows::core::Abi for DXC_OUT_KIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXC_OUT_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_OUT_KIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub struct DxcArgPair {
    pub pName: ::windows::core::PCWSTR,
    pub pValue: ::windows::core::PCWSTR,
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
unsafe impl ::windows::core::Abi for DxcArgPair {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcArgPair {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcArgPair>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcArgPair {}
impl ::core::default::Default for DxcArgPair {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
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
unsafe impl ::windows::core::Abi for DxcBuffer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcBuffer>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcBuffer {}
impl ::core::default::Default for DxcBuffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub struct DxcDefine {
    pub Name: ::windows::core::PCWSTR,
    pub Value: ::windows::core::PCWSTR,
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
unsafe impl ::windows::core::Abi for DxcDefine {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcDefine>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcDefine {}
impl ::core::default::Default for DxcDefine {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
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
unsafe impl ::windows::core::Abi for DxcShaderHash {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcShaderHash>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcShaderHash {}
impl ::core::default::Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = ::core::option::Option<unsafe extern "system" fn(pmalloc: ::core::option::Option<super::super::super::System::Com::IMalloc>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub type DxcCreateInstanceProc = ::core::option::Option<unsafe extern "system" fn(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
