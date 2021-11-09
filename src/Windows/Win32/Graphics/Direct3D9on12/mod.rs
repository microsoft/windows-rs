#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`*"]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: super::super::Foundation::BOOL,
    pub pD3D12Device: ::core::option::Option<::windows::runtime::IUnknown>,
    pub ppD3D12Queues: [::core::option::Option<::windows::runtime::IUnknown>; 2],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl D3D9ON12_ARGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D9ON12_ARGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D9ON12_ARGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D9ON12_ARGS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for D3D9ON12_ARGS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>;
        }
        ::core::mem::transmute(Direct3DCreate9On12(::core::mem::transmute(sdkversion), ::core::mem::transmute(poverridelist), ::core::mem::transmute(numoverrideentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32, ppoutputinterface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        Direct3DCreate9On12Ex(::core::mem::transmute(sdkversion), ::core::mem::transmute(poverridelist), ::core::mem::transmute(numoverrideentries), ::core::mem::transmute(ppoutputinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirect3DDevice9On12(pub ::windows::runtime::IUnknown);
impl IDirect3DDevice9On12 {
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`*"]
    pub unsafe fn GetD3D12Device(&self, riid: *const ::windows::runtime::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppvdevice)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn UnwrapUnderlyingResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Direct3D9::IDirect3DResource9>, Param1: ::windows::runtime::IntoParam<'a, super::Direct3D12::ID3D12CommandQueue>>(&self, presource: Param0, pcommandqueue: Param1, riid: *const ::windows::runtime::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), presource.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvresource12)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn ReturnUnderlyingResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Direct3D9::IDirect3DResource9>>(&self, presource: Param0, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirect3DDevice9On12 {
    type Vtable = IDirect3DDevice9On12_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3892159028, 46473, 16457, [148, 13, 136, 120, 151, 117, 49, 200]);
}
impl ::core::convert::From<IDirect3DDevice9On12> for ::windows::runtime::IUnknown {
    fn from(value: IDirect3DDevice9On12) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirect3DDevice9On12> for ::windows::runtime::IUnknown {
    fn from(value: &IDirect3DDevice9On12) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9On12_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presource: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presource: ::windows::runtime::RawPtr, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`*"]
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12 = unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>;
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12Ex = unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32, ppoutputinterface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
