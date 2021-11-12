#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8e9ebe7_39b7_3962_bb03_57f56e1fb0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxtrianglespercubicmeter: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxtrianglespercubicmeter: f64, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceMesh(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108f57d9_df0d_3950_a0fd_f972c77c27b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMesh_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshBuffer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93cf59e0_871f_33f8_98b2_03d101458f6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2759f89_3572_3d2d_a10d_5fee9394aa37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptionsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshOptionsStatics {
    type Vtable = ISpatialSurfaceMeshOptionsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b340abf_9781_4505_8935_013575caae5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptionsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserver(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10b69819_ddca_3483_ac3a_748fe8c86df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserver_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserverStatics {
    type Vtable = ISpatialSurfaceObserverStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x165951ed_2108_4168_9175_87e027bc9285);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserverStatics2 {
    type Vtable = ISpatialSurfaceObserverStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f534261_c55d_4e6b_a895_a19de69a42e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialSurfaceInfo(pub ::windows::core::IInspectable);
impl SpatialSurfaceInfo {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn TryGetBounds<'a, Param0: ::windows::core::IntoParam<'a, super::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshAsync(&self, maxtrianglespercubicmeter: f64) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), maxtrianglespercubicmeter, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshWithOptionsAsync<'a, Param1: ::windows::core::IntoParam<'a, SpatialSurfaceMeshOptions>>(&self, maxtrianglespercubicmeter: f64, options: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), maxtrianglespercubicmeter, options.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo;{f8e9ebe7-39b7-3962-bb03-57f56e1fb0a1})");
}
unsafe impl ::windows::core::Interface for SpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8e9ebe7_39b7_3962_bb03_57f56e1fb0a1);
}
impl ::windows::core::RuntimeName for SpatialSurfaceInfo {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
}
impl ::core::convert::From<SpatialSurfaceInfo> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialSurfaceInfo> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialSurfaceInfo> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialSurfaceInfo> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceInfo {}
unsafe impl ::core::marker::Sync for SpatialSurfaceInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialSurfaceMesh(pub ::windows::core::IInspectable);
impl SpatialSurfaceMesh {
    pub fn SurfaceInfo(&self) -> ::windows::core::Result<SpatialSurfaceInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceInfo>(result__)
        }
    }
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        }
    }
    pub fn TriangleIndices(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    pub fn VertexPositions(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VertexPositionScale(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn VertexNormals(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMesh {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh;{108f57d9-df0d-3950-a0fd-f972c77c27b4})");
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108f57d9_df0d_3950_a0fd_f972c77c27b4);
}
impl ::windows::core::RuntimeName for SpatialSurfaceMesh {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
}
impl ::core::convert::From<SpatialSurfaceMesh> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceMesh) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialSurfaceMesh> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceMesh) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialSurfaceMesh> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceMesh) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialSurfaceMesh> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceMesh) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMesh {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMesh {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialSurfaceMeshBuffer(pub ::windows::core::IInspectable);
impl SpatialSurfaceMeshBuffer {
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    pub fn Stride(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ElementCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMeshBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer;{93cf59e0-871f-33f8-98b2-03d101458f6f})");
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93cf59e0_871f_33f8_98b2_03d101458f6f);
}
impl ::windows::core::RuntimeName for SpatialSurfaceMeshBuffer {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
}
impl ::core::convert::From<SpatialSurfaceMeshBuffer> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceMeshBuffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshBuffer> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceMeshBuffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialSurfaceMeshBuffer> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceMeshBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshBuffer> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceMeshBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMeshBuffer {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshBuffer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialSurfaceMeshOptions(pub ::windows::core::IInspectable);
impl SpatialSurfaceMeshOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceMeshOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexPositionFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexPositionFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn TriangleIndexFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetTriangleIndexFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexNormalFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexNormalFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IncludeVertexNormals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeVertexNormals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexPositionFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedTriangleIndexFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexNormalFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    pub fn ISpatialSurfaceMeshOptionsStatics<R, F: FnOnce(&ISpatialSurfaceMeshOptionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceMeshOptions, ISpatialSurfaceMeshOptionsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMeshOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions;{d2759f89-3572-3d2d-a10d-5fee9394aa37})");
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2759f89_3572_3d2d_a10d_5fee9394aa37);
}
impl ::windows::core::RuntimeName for SpatialSurfaceMeshOptions {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
}
impl ::core::convert::From<SpatialSurfaceMeshOptions> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceMeshOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshOptions> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceMeshOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialSurfaceMeshOptions> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceMeshOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshOptions> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceMeshOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMeshOptions {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialSurfaceObserver(pub ::windows::core::IInspectable);
impl SpatialSurfaceObserver {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetObservedSurfaces(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>>(result__)
        }
    }
    pub fn SetBoundingVolume<'a, Param0: ::windows::core::IntoParam<'a, super::SpatialBoundingVolume>>(&self, bounds: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), bounds.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetBoundingVolumes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume>>>(&self, bounds: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), bounds.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ObservedSurfacesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveObservedSurfacesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> {
        Self::ISpatialSurfaceObserverStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ISpatialSurfaceObserverStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ISpatialSurfaceObserverStatics<R, F: FnOnce(&ISpatialSurfaceObserverStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialSurfaceObserverStatics2<R, F: FnOnce(&ISpatialSurfaceObserverStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceObserver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver;{10b69819-ddca-3483-ac3a-748fe8c86df5})");
}
unsafe impl ::windows::core::Interface for SpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10b69819_ddca_3483_ac3a_748fe8c86df5);
}
impl ::windows::core::RuntimeName for SpatialSurfaceObserver {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
}
impl ::core::convert::From<SpatialSurfaceObserver> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceObserver) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialSurfaceObserver> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceObserver) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialSurfaceObserver> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceObserver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialSurfaceObserver> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceObserver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceObserver {}
unsafe impl ::core::marker::Sync for SpatialSurfaceObserver {}
