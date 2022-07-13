#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`*"]
#[repr(transparent)]
pub struct IHolographicCameraInterop(::windows::core::IUnknown);
impl IHolographicCameraInterop {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12BackBufferResource<'a, P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDirect3D12BackBufferResource)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(ptexture2ddesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<'a, P0, P1>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P1) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDirect3D12HardwareProtectedBackBufferResource)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>,
    {
        (::windows::core::Interface::vtable(self).AcquireDirect3D12BufferResource)(::windows::core::Interface::as_raw(self), presourcetoacquire.into().abi(), pcommandqueue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>,
    {
        (::windows::core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(::windows::core::Interface::as_raw(self), presourcetoacquire.into().abi(), pcommandqueue.into().abi(), duration).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, P0>(&self, presourcetounacquire: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(::windows::core::Interface::as_raw(self), presourcetounacquire.into().abi()).ok()
    }
}
impl ::core::convert::From<IHolographicCameraInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicCameraInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicCameraInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHolographicCameraInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicCameraInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IHolographicCameraInterop> for ::windows::core::IInspectable {
    fn from(value: IHolographicCameraInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicCameraInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IHolographicCameraInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraInterop> for ::windows::core::IInspectable {
    fn from(value: &IHolographicCameraInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IHolographicCameraInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicCameraInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicCameraInterop {}
impl ::core::fmt::Debug for IHolographicCameraInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicCameraInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHolographicCameraInterop {
    type Vtable = IHolographicCameraInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cc1f9c5_6d02_41fa_9500_e1809eb48eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12BackBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12BackBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedBackBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedBackBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`*"]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParametersInterop(::windows::core::IUnknown);
impl IHolographicCameraRenderingParametersInterop {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<'a, P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>,
    {
        (::windows::core::Interface::vtable(self).CommitDirect3D12Resource)(::windows::core::Interface::as_raw(self), pcolorresourcetocommit.into().abi(), pcolorresourcefence.into().abi(), colorresourcefencesignalvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<'a, P0, P1, P2, P3>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: P2, pdepthresourcefence: P3, depthresourcefencesignalvalue: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>,
    {
        (::windows::core::Interface::vtable(self).CommitDirect3D12ResourceWithDepthData)(::windows::core::Interface::as_raw(self), pcolorresourcetocommit.into().abi(), pcolorresourcefence.into().abi(), colorresourcefencesignalvalue, pdepthresourcetocommit.into().abi(), pdepthresourcefence.into().abi(), depthresourcefencesignalvalue).ok()
    }
}
impl ::core::convert::From<IHolographicCameraRenderingParametersInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicCameraRenderingParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicCameraRenderingParametersInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHolographicCameraRenderingParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraRenderingParametersInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicCameraRenderingParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IHolographicCameraRenderingParametersInterop> for ::windows::core::IInspectable {
    fn from(value: IHolographicCameraRenderingParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicCameraRenderingParametersInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IHolographicCameraRenderingParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraRenderingParametersInterop> for ::windows::core::IInspectable {
    fn from(value: &IHolographicCameraRenderingParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IHolographicCameraRenderingParametersInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicCameraRenderingParametersInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicCameraRenderingParametersInterop {}
impl ::core::fmt::Debug for IHolographicCameraRenderingParametersInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicCameraRenderingParametersInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHolographicCameraRenderingParametersInterop {
    type Vtable = IHolographicCameraRenderingParametersInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf75b68d6_d1fd_4707_aafd_fa6f4c0e3bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParametersInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12ResourceWithDepthData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: *mut ::core::ffi::c_void, pdepthresourcefence: *mut ::core::ffi::c_void, depthresourcefencesignalvalue: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12ResourceWithDepthData: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`*"]
#[repr(transparent)]
pub struct IHolographicQuadLayerInterop(::windows::core::IUnknown);
impl IHolographicQuadLayerInterop {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12ContentBufferResource<'a, P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDirect3D12ContentBufferResource)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(ptexture2ddesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<'a, P0, P1>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P1) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDirect3D12HardwareProtectedContentBufferResource)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>,
    {
        (::windows::core::Interface::vtable(self).AcquireDirect3D12BufferResource)(::windows::core::Interface::as_raw(self), presourcetoacquire.into().abi(), pcommandqueue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>,
    {
        (::windows::core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(::windows::core::Interface::as_raw(self), presourcetoacquire.into().abi(), pcommandqueue.into().abi(), duration).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, P0>(&self, presourcetounacquire: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(::windows::core::Interface::as_raw(self), presourcetounacquire.into().abi()).ok()
    }
}
impl ::core::convert::From<IHolographicQuadLayerInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicQuadLayerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicQuadLayerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHolographicQuadLayerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicQuadLayerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IHolographicQuadLayerInterop> for ::windows::core::IInspectable {
    fn from(value: IHolographicQuadLayerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicQuadLayerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IHolographicQuadLayerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerInterop> for ::windows::core::IInspectable {
    fn from(value: &IHolographicQuadLayerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IHolographicQuadLayerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicQuadLayerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicQuadLayerInterop {}
impl ::core::fmt::Debug for IHolographicQuadLayerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicQuadLayerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayerInterop {
    type Vtable = IHolographicQuadLayerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa688f0_639e_4a47_83d7_6b7f5ebf7fed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12ContentBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12ContentBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedContentBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedContentBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`*"]
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParametersInterop(::windows::core::IUnknown);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<'a, P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>,
    {
        (::windows::core::Interface::vtable(self).CommitDirect3D12Resource)(::windows::core::Interface::as_raw(self), pcolorresourcetocommit.into().abi(), pcolorresourcefence.into().abi(), colorresourcefencesignalvalue).ok()
    }
}
impl ::core::convert::From<IHolographicQuadLayerUpdateParametersInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicQuadLayerUpdateParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicQuadLayerUpdateParametersInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHolographicQuadLayerUpdateParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerUpdateParametersInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicQuadLayerUpdateParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IHolographicQuadLayerUpdateParametersInterop> for ::windows::core::IInspectable {
    fn from(value: IHolographicQuadLayerUpdateParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicQuadLayerUpdateParametersInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IHolographicQuadLayerUpdateParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerUpdateParametersInterop> for ::windows::core::IInspectable {
    fn from(value: &IHolographicQuadLayerUpdateParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IHolographicQuadLayerUpdateParametersInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicQuadLayerUpdateParametersInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicQuadLayerUpdateParametersInterop {}
impl ::core::fmt::Debug for IHolographicQuadLayerUpdateParametersInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicQuadLayerUpdateParametersInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayerUpdateParametersInterop {
    type Vtable = IHolographicQuadLayerUpdateParametersInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5f549cd_c909_444f_8809_7cc18a9c8920);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
