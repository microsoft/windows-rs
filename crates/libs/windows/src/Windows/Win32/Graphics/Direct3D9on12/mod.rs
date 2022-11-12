#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9> {
    ::windows::core::link ! ( "d3d9.dll""system" fn Direct3DCreate9On12 ( sdkversion : u32 , poverridelist : *mut ::core::mem::ManuallyDrop < D3D9ON12_ARGS > , numoverrideentries : u32 ) -> ::core::option::Option < super::Direct3D9:: IDirect3D9 > );
    Direct3DCreate9On12(sdkversion, ::core::mem::transmute(poverridelist), numoverrideentries)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "d3d9.dll""system" fn Direct3DCreate9On12Ex ( sdkversion : u32 , poverridelist : *mut ::core::mem::ManuallyDrop < D3D9ON12_ARGS > , numoverrideentries : u32 , ppoutputinterface : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    Direct3DCreate9On12Ex(sdkversion, ::core::mem::transmute(poverridelist), numoverrideentries, ::core::mem::transmute(ppoutputinterface)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`*"]
#[repr(transparent)]
pub struct IDirect3DDevice9On12(::windows::core::IUnknown);
impl IDirect3DDevice9On12 {
    pub unsafe fn GetD3D12Device(&self, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetD3D12Device)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppvdevice)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn UnwrapUnderlyingResource<'a, P0, P1>(&self, presource: P0, pcommandqueue: P1, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D9::IDirect3DResource9>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D12::ID3D12CommandQueue>>,
    {
        (::windows::core::Vtable::vtable(self).UnwrapUnderlyingResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcommandqueue.into().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvresource12)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn ReturnUnderlyingResource<'a, P0>(&self, presource: P0, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D9::IDirect3DResource9>>,
    {
        (::windows::core::Vtable::vtable(self).ReturnUnderlyingResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), numsync, ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
::windows::core::interface_hierarchy!(IDirect3DDevice9On12, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IDirect3DDevice9On12 {
    type Vtable = IDirect3DDevice9On12_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirect3DDevice9On12 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7fda234_b589_4049_940d_8878977531c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9On12_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetD3D12Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *mut u64, ppfences: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    ReturnUnderlyingResource: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`*"]
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
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
pub type PFN_Direct3DCreate9On12 = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12Ex = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
