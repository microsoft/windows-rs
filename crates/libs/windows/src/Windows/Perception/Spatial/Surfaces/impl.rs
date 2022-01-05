#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn TryGetBounds(&self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>>;
    fn TryComputeLatestMeshAsync(&self, maxtrianglespercubicmeter: f64) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>;
    fn TryComputeLatestMeshWithOptionsAsync(&self, maxtrianglespercubicmeter: f64, options: &::core::option::Option<SpatialSurfaceMeshOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceMeshImpl: Sized {
    fn SurfaceInfo(&self) -> ::windows::core::Result<SpatialSurfaceInfo>;
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn TriangleIndices(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer>;
    fn VertexPositions(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer>;
    fn VertexPositionScale(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn VertexNormals(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceMeshBufferImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn Stride(&self) -> ::windows::core::Result<u32>;
    fn ElementCount(&self) -> ::windows::core::Result<u32>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceMeshOptionsImpl: Sized {
    fn VertexPositionFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetVertexPositionFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn TriangleIndexFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetTriangleIndexFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn VertexNormalFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetVertexNormalFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn IncludeVertexNormals(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeVertexNormals(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceMeshOptionsStaticsImpl: Sized {
    fn SupportedVertexPositionFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>;
    fn SupportedTriangleIndexFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>;
    fn SupportedVertexNormalFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceObserverImpl: Sized {
    fn GetObservedSurfaces(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>>;
    fn SetBoundingVolume(&self, bounds: &::core::option::Option<super::SpatialBoundingVolume>) -> ::windows::core::Result<()>;
    fn SetBoundingVolumes(&self, bounds: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume>>) -> ::windows::core::Result<()>;
    fn ObservedSurfacesChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveObservedSurfacesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceObserverStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialSurfaceObserverStatics2Impl: Sized + ISpatialSurfaceObserverStaticsImpl {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
