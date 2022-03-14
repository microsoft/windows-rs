#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: super::super::Foundation::BOOL,
    pub pD3D12Device: ::core::option::Option<::windows::core::IUnknown>,
    pub ppD3D12Queues: [::core::option::Option<::windows::core::IUnknown>; 2],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D9ON12_ARGS {
    fn clone(&self) -> Self {
        Self {
            Enable9On12: self.Enable9On12,
            pD3D12Device: self.pD3D12Device.clone(),
            ppD3D12Queues: self.ppD3D12Queues.clone(),
            NumQueues: self.NumQueues,
            NodeMask: self.NodeMask,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D9ON12_ARGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D9ON12_ARGS").field("Enable9On12", &self.Enable9On12).field("pD3D12Device", &self.pD3D12Device).field("ppD3D12Queues", &self.ppD3D12Queues).field("NumQueues", &self.NumQueues).field("NodeMask", &self.NodeMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D9ON12_ARGS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D9ON12_ARGS {
    fn eq(&self, other: &Self) -> bool {
        self.Enable9On12 == other.Enable9On12 && self.pD3D12Device == other.pD3D12Device && self.ppD3D12Queues == other.ppD3D12Queues && self.NumQueues == other.NumQueues && self.NodeMask == other.NodeMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D9ON12_ARGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D9ON12_ARGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>;
        }
        ::core::mem::transmute(Direct3DCreate9On12(::core::mem::transmute(sdkversion), ::core::mem::transmute(poverridelist), ::core::mem::transmute(numoverrideentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        Direct3DCreate9On12Ex(::core::mem::transmute(sdkversion), ::core::mem::transmute(poverridelist), ::core::mem::transmute(numoverrideentries), ::core::mem::transmute(ppoutputinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`*"]
#[repr(transparent)]
pub struct IDirect3DDevice9On12(::windows::core::IUnknown);
impl IDirect3DDevice9On12 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`*"]
    pub unsafe fn GetD3D12Device(&self, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetD3D12Device)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppvdevice)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn UnwrapUnderlyingResource<'a, Param0: ::windows::core::IntoParam<'a, super::Direct3D9::IDirect3DResource9>, Param1: ::windows::core::IntoParam<'a, super::Direct3D12::ID3D12CommandQueue>>(&self, presource: Param0, pcommandqueue: Param1, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnwrapUnderlyingResource)(::core::mem::transmute_copy(self), presource.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvresource12)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn ReturnUnderlyingResource<'a, Param0: ::windows::core::IntoParam<'a, super::Direct3D9::IDirect3DResource9>>(&self, presource: Param0, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReturnUnderlyingResource)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
impl ::core::convert::From<IDirect3DDevice9On12> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDevice9On12) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDevice9On12> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDevice9On12) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DDevice9On12 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice9On12 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice9On12 {}
impl ::core::fmt::Debug for IDirect3DDevice9On12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice9On12").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DDevice9On12 {
    type Vtable = IDirect3DDevice9On12_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7fda234_b589_4049_940d_8878977531c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9On12_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetD3D12Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    ReturnUnderlyingResource: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`*"]
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12 = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12Ex = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
