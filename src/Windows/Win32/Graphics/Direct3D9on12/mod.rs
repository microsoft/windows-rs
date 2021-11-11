#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`*"]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: super::super::Foundation::BOOL,
    pub pD3D12Device: ::core::option::Option<::windows::core::IUnknown>,
    pub ppD3D12Queues: [::core::option::Option<::windows::core::IUnknown>; 2],
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
unsafe impl ::windows::core::Abi for D3D9ON12_ARGS {
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
pub unsafe fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32, ppoutputinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        Direct3DCreate9On12Ex(::core::mem::transmute(sdkversion), ::core::mem::transmute(poverridelist), ::core::mem::transmute(numoverrideentries), ::core::mem::transmute(ppoutputinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirect3DDevice9On12(pub ::windows::core::IUnknown);
impl IDirect3DDevice9On12 {
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`*"]
    pub unsafe fn GetD3D12Device(&self, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppvdevice)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn UnwrapUnderlyingResource<'a, Param0: ::windows::core::IntoParam<'a, super::Direct3D9::IDirect3DResource9>, Param1: ::windows::core::IntoParam<'a, super::Direct3D12::ID3D12CommandQueue>>(&self, presource: Param0, pcommandqueue: Param1, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), presource.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvresource12)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn ReturnUnderlyingResource<'a, Param0: ::windows::core::IntoParam<'a, super::Direct3D9::IDirect3DResource9>>(&self, presource: Param0, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DDevice9On12 {
    type Vtable = IDirect3DDevice9On12_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7fda234_b589_4049_940d_8878977531c8);
}
impl ::core::convert::From<IDirect3DDevice9On12> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDevice9On12) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirect3DDevice9On12> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDevice9On12) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9On12_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presource: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presource: ::windows::core::RawPtr, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`*"]
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12 = unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>;
#[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12Ex = unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32, ppoutputinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
