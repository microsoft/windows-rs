#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicCameraInterop(pub ::windows::core::IUnknown);
impl IHolographicCameraInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12BackBufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, Param2: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>(
        &self,
        pdevice: Param0,
        ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC,
        pprotectedresourcesession: Param2,
    ) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), presourcetounacquire.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IHolographicCameraInterop {
    type Vtable = IHolographicCameraInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cc1f9c5_6d02_41fa_9500_e1809eb48eec);
}
impl ::core::convert::From<IHolographicCameraInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicCameraInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicCameraInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicCameraInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHolographicCameraInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHolographicCameraInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicCameraRenderingParametersInterop(pub ::windows::core::IUnknown);
impl IHolographicCameraRenderingParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::core::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>, Param3: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>(
        &self,
        pcolorresourcetocommit: Param0,
        pcolorresourcefence: Param1,
        colorresourcefencesignalvalue: u64,
        pdepthresourcetocommit: Param3,
        pdepthresourcefence: Param4,
        depthresourcefencesignalvalue: u64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            pcolorresourcetocommit.into_param().abi(),
            pcolorresourcefence.into_param().abi(),
            ::core::mem::transmute(colorresourcefencesignalvalue),
            pdepthresourcetocommit.into_param().abi(),
            pdepthresourcefence.into_param().abi(),
            ::core::mem::transmute(depthresourcefencesignalvalue),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IHolographicCameraRenderingParametersInterop {
    type Vtable = IHolographicCameraRenderingParametersInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf75b68d6_d1fd_4707_aafd_fa6f4c0e3bf4);
}
impl ::core::convert::From<IHolographicCameraRenderingParametersInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicCameraRenderingParametersInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicCameraRenderingParametersInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicCameraRenderingParametersInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParametersInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::windows::core::RawPtr, pdepthresourcefence: ::windows::core::RawPtr, depthresourcefencesignalvalue: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicQuadLayerInterop(pub ::windows::core::IUnknown);
impl IHolographicQuadLayerInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12ContentBufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, Param2: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>(
        &self,
        pdevice: Param0,
        ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC,
        pprotectedresourcesession: Param2,
    ) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), presourcetounacquire.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayerInterop {
    type Vtable = IHolographicQuadLayerInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa688f0_639e_4a47_83d7_6b7f5ebf7fed);
}
impl ::core::convert::From<IHolographicQuadLayerInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicQuadLayerInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicQuadLayerInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicQuadLayerInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicQuadLayerUpdateParametersInterop(pub ::windows::core::IUnknown);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::core::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
}
unsafe impl ::windows::core::Interface for IHolographicQuadLayerUpdateParametersInterop {
    type Vtable = IHolographicQuadLayerUpdateParametersInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5f549cd_c909_444f_8809_7cc18a9c8920);
}
impl ::core::convert::From<IHolographicQuadLayerUpdateParametersInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicQuadLayerUpdateParametersInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicQuadLayerUpdateParametersInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicQuadLayerUpdateParametersInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParametersInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
