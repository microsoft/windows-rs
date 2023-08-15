#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8e9ebe7_39b7_3962_bb03_57f56e1fb0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateTime: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetBounds: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMesh(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceMesh {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x108f57d9_df0d_3950_a0fd_f972c77c27b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMesh_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SurfaceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TriangleIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub VertexPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub VertexPositionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VertexPositionScale: usize,
    pub VertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceMeshBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceMeshBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93cf59e0_871f_33f8_98b2_03d101458f6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    pub Stride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceMeshOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceMeshOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2759f89_3572_3d2d_a10d_5fee9394aa37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub TriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    TriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetTriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetTriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexNormalFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexNormalFormat: usize,
    pub IncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptionsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMeshOptionsStatics {
    type Vtable = ISpatialSurfaceMeshOptionsStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceMeshOptionsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceMeshOptionsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b340abf_9781_4505_8935_013575caae5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptionsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexPositionFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexPositionFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedTriangleIndexFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedTriangleIndexFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexNormalFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexNormalFormats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceObserver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceObserver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10b69819_ddca_3483_ac3a_748fe8c86df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetObservedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetObservedSurfaces: usize,
    pub SetBoundingVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetBoundingVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetBoundingVolumes: usize,
    #[cfg(feature = "Foundation")]
    pub ObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ObservedSurfacesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveObservedSurfacesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceObserverStatics {
    type Vtable = ISpatialSurfaceObserverStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceObserverStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceObserverStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x165951ed_2108_4168_9175_87e027bc9285);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceObserverStatics2 {
    type Vtable = ISpatialSurfaceObserverStatics2_Vtbl;
}
impl ::core::clone::Clone for ISpatialSurfaceObserverStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialSurfaceObserverStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f534261_c55d_4e6b_a895_a19de69a42e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceInfo(::windows_core::IUnknown);
impl SpatialSurfaceInfo {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetBounds<P0>(&self, coordinatesystem: P0) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>>
    where
        P0: ::windows_core::IntoParam<super::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBounds)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshAsync(&self, maxtrianglespercubicmeter: f64) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLatestMeshAsync)(::windows_core::Interface::as_raw(this), maxtrianglespercubicmeter, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshWithOptionsAsync<P0>(&self, maxtrianglespercubicmeter: f64, options: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>
    where
        P0: ::windows_core::IntoParam<SpatialSurfaceMeshOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLatestMeshWithOptionsAsync)(::windows_core::Interface::as_raw(this), maxtrianglespercubicmeter, options.into_param().abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialSurfaceInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo;{f8e9ebe7-39b7-3962-bb03-57f56e1fb0a1})");
}
impl ::core::clone::Clone for SpatialSurfaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialSurfaceInfo {
    const IID: ::windows_core::GUID = <ISpatialSurfaceInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceInfo {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
}
::windows_core::imp::interface_hierarchy!(SpatialSurfaceInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceInfo {}
unsafe impl ::core::marker::Sync for SpatialSurfaceInfo {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMesh(::windows_core::IUnknown);
impl SpatialSurfaceMesh {
    pub fn SurfaceInfo(&self) -> ::windows_core::Result<SpatialSurfaceInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SurfaceInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TriangleIndices(&self) -> ::windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriangleIndices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VertexPositions(&self) -> ::windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VertexPositions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VertexPositionScale(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VertexPositionScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VertexNormals(&self) -> ::windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VertexNormals)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialSurfaceMesh {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh;{108f57d9-df0d-3950-a0fd-f972c77c27b4})");
}
impl ::core::clone::Clone for SpatialSurfaceMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialSurfaceMesh {
    const IID: ::windows_core::GUID = <ISpatialSurfaceMesh as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceMesh {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
}
::windows_core::imp::interface_hierarchy!(SpatialSurfaceMesh, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceMesh {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMesh {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMeshBuffer(::windows_core::IUnknown);
impl SpatialSurfaceMeshBuffer {
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> ::windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Stride(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stride)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ElementCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialSurfaceMeshBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer;{93cf59e0-871f-33f8-98b2-03d101458f6f})");
}
impl ::core::clone::Clone for SpatialSurfaceMeshBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialSurfaceMeshBuffer {
    const IID: ::windows_core::GUID = <ISpatialSurfaceMeshBuffer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceMeshBuffer {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
}
::windows_core::imp::interface_hierarchy!(SpatialSurfaceMeshBuffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceMeshBuffer {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshBuffer {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMeshOptions(::windows_core::IUnknown);
impl SpatialSurfaceMeshOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialSurfaceMeshOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexPositionFormat(&self) -> ::windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VertexPositionFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexPositionFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVertexPositionFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn TriangleIndexFormat(&self) -> ::windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriangleIndexFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetTriangleIndexFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTriangleIndexFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexNormalFormat(&self) -> ::windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VertexNormalFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexNormalFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVertexNormalFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeVertexNormals(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IncludeVertexNormals)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIncludeVertexNormals(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeVertexNormals)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexPositionFormats() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVertexPositionFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedTriangleIndexFormats() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTriangleIndexFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexNormalFormats() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVertexNormalFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceMeshOptionsStatics<R, F: FnOnce(&ISpatialSurfaceMeshOptionsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialSurfaceMeshOptions, ISpatialSurfaceMeshOptionsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialSurfaceMeshOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions;{d2759f89-3572-3d2d-a10d-5fee9394aa37})");
}
impl ::core::clone::Clone for SpatialSurfaceMeshOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialSurfaceMeshOptions {
    const IID: ::windows_core::GUID = <ISpatialSurfaceMeshOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceMeshOptions {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
}
::windows_core::imp::interface_hierarchy!(SpatialSurfaceMeshOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceMeshOptions {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshOptions {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceObserver(::windows_core::IUnknown);
impl SpatialSurfaceObserver {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialSurfaceObserver, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetObservedSurfaces(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::GUID, SpatialSurfaceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetObservedSurfaces)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBoundingVolume<P0>(&self, bounds: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::SpatialBoundingVolume>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBoundingVolume)(::windows_core::Interface::as_raw(this), bounds.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetBoundingVolumes<P0>(&self, bounds: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBoundingVolumes)(::windows_core::Interface::as_raw(this), bounds.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ObservedSurfacesChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ObservedSurfacesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveObservedSurfacesChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveObservedSurfacesChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> {
        Self::ISpatialSurfaceObserverStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISpatialSurfaceObserverStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics<R, F: FnOnce(&ISpatialSurfaceObserverStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics2<R, F: FnOnce(&ISpatialSurfaceObserverStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialSurfaceObserver {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver;{10b69819-ddca-3483-ac3a-748fe8c86df5})");
}
impl ::core::clone::Clone for SpatialSurfaceObserver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialSurfaceObserver {
    const IID: ::windows_core::GUID = <ISpatialSurfaceObserver as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceObserver {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
}
::windows_core::imp::interface_hierarchy!(SpatialSurfaceObserver, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceObserver {}
unsafe impl ::core::marker::Sync for SpatialSurfaceObserver {}
