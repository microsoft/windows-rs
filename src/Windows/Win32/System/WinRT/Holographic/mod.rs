#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicCameraInterop(pub ::windows::runtime::IUnknown);
impl IHolographicCameraInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12BackBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>(
        &self,
        pdevice: Param0,
        ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC,
        pprotectedresourcesession: Param2,
    ) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), presourcetounacquire.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicCameraInterop {
    type Vtable = IHolographicCameraInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7cc1f9c5_6d02_41fa_9500_e1809eb48eec);
}
impl ::core::convert::From<IHolographicCameraInterop> for ::windows::runtime::IUnknown {
    fn from(value: IHolographicCameraInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicCameraInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IHolographicCameraInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHolographicCameraInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IHolographicCameraInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::runtime::RawPtr, ppcreatedtexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr, duration: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetounacquire: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicCameraRenderingParametersInterop(pub ::windows::runtime::IUnknown);
impl IHolographicCameraRenderingParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::core::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>(
        &self,
        pcolorresourcetocommit: Param0,
        pcolorresourcefence: Param1,
        colorresourcefencesignalvalue: u64,
        pdepthresourcetocommit: Param3,
        pdepthresourcefence: Param4,
        depthresourcefencesignalvalue: u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
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
unsafe impl ::windows::runtime::Interface for IHolographicCameraRenderingParametersInterop {
    type Vtable = IHolographicCameraRenderingParametersInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf75b68d6_d1fd_4707_aafd_fa6f4c0e3bf4);
}
impl ::core::convert::From<IHolographicCameraRenderingParametersInterop> for ::windows::runtime::IUnknown {
    fn from(value: IHolographicCameraRenderingParametersInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicCameraRenderingParametersInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IHolographicCameraRenderingParametersInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParametersInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolorresourcetocommit: ::windows::runtime::RawPtr, pcolorresourcefence: ::windows::runtime::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolorresourcetocommit: ::windows::runtime::RawPtr, pcolorresourcefence: ::windows::runtime::RawPtr, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::windows::runtime::RawPtr, pdepthresourcefence: ::windows::runtime::RawPtr, depthresourcefencesignalvalue: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicQuadLayerInterop(pub ::windows::runtime::IUnknown);
impl IHolographicQuadLayerInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12ContentBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>(
        &self,
        pdevice: Param0,
        ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC,
        pprotectedresourcesession: Param2,
    ) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), presourcetounacquire.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayerInterop {
    type Vtable = IHolographicQuadLayerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcfa688f0_639e_4a47_83d7_6b7f5ebf7fed);
}
impl ::core::convert::From<IHolographicQuadLayerInterop> for ::windows::runtime::IUnknown {
    fn from(value: IHolographicQuadLayerInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicQuadLayerInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IHolographicQuadLayerInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::runtime::RawPtr, ppcreatedtexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr, duration: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetounacquire: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolographicQuadLayerUpdateParametersInterop(pub ::windows::runtime::IUnknown);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_Holographic`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::core::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayerUpdateParametersInterop {
    type Vtable = IHolographicQuadLayerUpdateParametersInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe5f549cd_c909_444f_8809_7cc18a9c8920);
}
impl ::core::convert::From<IHolographicQuadLayerUpdateParametersInterop> for ::windows::runtime::IUnknown {
    fn from(value: IHolographicQuadLayerUpdateParametersInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolographicQuadLayerUpdateParametersInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IHolographicQuadLayerUpdateParametersInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParametersInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolorresourcetocommit: ::windows::runtime::RawPtr, pcolorresourcefence: ::windows::runtime::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
