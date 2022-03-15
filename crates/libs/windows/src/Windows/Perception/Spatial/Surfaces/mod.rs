#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8e9ebe7_39b7_3962_bb03_57f56e1fb0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateTime: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetBounds: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMesh(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108f57d9_df0d_3950_a0fd_f972c77c27b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMesh_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SurfaceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TriangleIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VertexPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub VertexPositionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VertexPositionScale: usize,
    pub VertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshBuffer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93cf59e0_871f_33f8_98b2_03d101458f6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshBuffer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    pub Stride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2759f89_3572_3d2d_a10d_5fee9394aa37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub TriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    TriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetTriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetTriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexNormalFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexNormalFormat: usize,
    pub IncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptionsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshOptionsStatics {
    type Vtable = ISpatialSurfaceMeshOptionsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b340abf_9781_4505_8935_013575caae5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptionsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexPositionFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexPositionFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedTriangleIndexFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedTriangleIndexFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexNormalFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexNormalFormats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserver(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10b69819_ddca_3483_ac3a_748fe8c86df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserver_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetObservedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetObservedSurfaces: usize,
    pub SetBoundingVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetBoundingVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetBoundingVolumes: usize,
    #[cfg(feature = "Foundation")]
    pub ObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ObservedSurfacesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveObservedSurfacesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserverStatics {
    type Vtable = ISpatialSurfaceObserverStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x165951ed_2108_4168_9175_87e027bc9285);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserverStatics2 {
    type Vtable = ISpatialSurfaceObserverStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f534261_c55d_4e6b_a895_a19de69a42e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceInfo(::windows::core::IUnknown);
impl SpatialSurfaceInfo {
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetBounds<'a, Param0: ::windows::core::IntoParam<'a, super::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBounds)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshAsync(&self, maxtrianglespercubicmeter: f64) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryComputeLatestMeshAsync)(::core::mem::transmute_copy(this), maxtrianglespercubicmeter, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshWithOptionsAsync<'a, Param1: ::windows::core::IntoParam<'a, SpatialSurfaceMeshOptions>>(&self, maxtrianglespercubicmeter: f64, options: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryComputeLatestMeshWithOptionsAsync)(::core::mem::transmute_copy(this), maxtrianglespercubicmeter, options.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceInfo {}
impl ::core::fmt::Debug for SpatialSurfaceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo;{f8e9ebe7-39b7-3962-bb03-57f56e1fb0a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialSurfaceInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceInfo {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
}
impl ::core::convert::From<SpatialSurfaceInfo> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceInfo> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceInfo> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceInfo> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceInfo {}
unsafe impl ::core::marker::Sync for SpatialSurfaceInfo {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMesh(::windows::core::IUnknown);
impl SpatialSurfaceMesh {
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn SurfaceInfo(&self) -> ::windows::core::Result<SpatialSurfaceInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SurfaceInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoordinateSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn TriangleIndices(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TriangleIndices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn VertexPositions(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VertexPositions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VertexPositionScale(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VertexPositionScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn VertexNormals(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VertexNormals)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMesh {}
impl ::core::fmt::Debug for SpatialSurfaceMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMesh").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMesh {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh;{108f57d9-df0d-3950-a0fd-f972c77c27b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialSurfaceMesh as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceMesh {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
}
impl ::core::convert::From<SpatialSurfaceMesh> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceMesh) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMesh> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceMesh) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceMesh> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceMesh) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMesh> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceMesh) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceMesh {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMesh {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMesh {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMeshBuffer(::windows::core::IUnknown);
impl SpatialSurfaceMeshBuffer {
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn Stride(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stride)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn ElementCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMeshBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMeshBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMeshBuffer {}
impl ::core::fmt::Debug for SpatialSurfaceMeshBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMeshBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMeshBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer;{93cf59e0-871f-33f8-98b2-03d101458f6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialSurfaceMeshBuffer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceMeshBuffer {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
}
impl ::core::convert::From<SpatialSurfaceMeshBuffer> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceMeshBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshBuffer> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceMeshBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceMeshBuffer> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceMeshBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshBuffer> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceMeshBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMeshBuffer {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshBuffer {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMeshOptions(::windows::core::IUnknown);
impl SpatialSurfaceMeshOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceMeshOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexPositionFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VertexPositionFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexPositionFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVertexPositionFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn TriangleIndexFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TriangleIndexFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetTriangleIndexFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTriangleIndexFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexNormalFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VertexNormalFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexNormalFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVertexNormalFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn IncludeVertexNormals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncludeVertexNormals)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn SetIncludeVertexNormals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncludeVertexNormals)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexPositionFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedVertexPositionFormats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedTriangleIndexFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedTriangleIndexFormats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexNormalFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedVertexNormalFormats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceMeshOptionsStatics<R, F: FnOnce(&ISpatialSurfaceMeshOptionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceMeshOptions, ISpatialSurfaceMeshOptionsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMeshOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMeshOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMeshOptions {}
impl ::core::fmt::Debug for SpatialSurfaceMeshOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMeshOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMeshOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions;{d2759f89-3572-3d2d-a10d-5fee9394aa37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialSurfaceMeshOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceMeshOptions {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
}
impl ::core::convert::From<SpatialSurfaceMeshOptions> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceMeshOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshOptions> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceMeshOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceMeshOptions> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceMeshOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshOptions> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceMeshOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMeshOptions {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshOptions {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceObserver(::windows::core::IUnknown);
impl SpatialSurfaceObserver {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetObservedSurfaces(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObservedSurfaces)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn SetBoundingVolume<'a, Param0: ::windows::core::IntoParam<'a, super::SpatialBoundingVolume>>(&self, bounds: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBoundingVolume)(::core::mem::transmute_copy(this), bounds.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetBoundingVolumes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume>>>(&self, bounds: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBoundingVolumes)(::core::mem::transmute_copy(this), bounds.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ObservedSurfacesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ObservedSurfacesChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveObservedSurfacesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveObservedSurfacesChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> {
        Self::ISpatialSurfaceObserverStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ISpatialSurfaceObserverStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics<R, F: FnOnce(&ISpatialSurfaceObserverStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics2<R, F: FnOnce(&ISpatialSurfaceObserverStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialSurfaceObserver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceObserver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceObserver {}
impl ::core::fmt::Debug for SpatialSurfaceObserver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceObserver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceObserver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver;{10b69819-ddca-3483-ac3a-748fe8c86df5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialSurfaceObserver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceObserver {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
}
impl ::core::convert::From<SpatialSurfaceObserver> for ::windows::core::IUnknown {
    fn from(value: SpatialSurfaceObserver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceObserver> for ::windows::core::IUnknown {
    fn from(value: &SpatialSurfaceObserver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceObserver> for ::windows::core::IInspectable {
    fn from(value: SpatialSurfaceObserver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceObserver> for ::windows::core::IInspectable {
    fn from(value: &SpatialSurfaceObserver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialSurfaceObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceObserver {}
unsafe impl ::core::marker::Sync for SpatialSurfaceObserver {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
