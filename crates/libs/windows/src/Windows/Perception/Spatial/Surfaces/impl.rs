#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceInfoImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn UpdateTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn TryGetBounds(&mut self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>>;
    fn TryComputeLatestMeshAsync(&mut self, maxtrianglespercubicmeter: f64) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>;
    fn TryComputeLatestMeshWithOptionsAsync(&mut self, maxtrianglespercubicmeter: f64, options: &::core::option::Option<SpatialSurfaceMeshOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceInfo {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialSurfaceInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceInfoVtbl {
        unsafe extern "system" fn Id<Impl: ISpatialSurfaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTime<Impl: ISpatialSurfaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetBounds<Impl: ISpatialSurfaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetBounds(&*(&coordinatesystem as *const <super::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeLatestMeshAsync<Impl: ISpatialSurfaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryComputeLatestMeshAsync(maxtrianglespercubicmeter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeLatestMeshWithOptionsAsync<Impl: ISpatialSurfaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryComputeLatestMeshWithOptionsAsync(maxtrianglespercubicmeter, &*(&options as *const <SpatialSurfaceMeshOptions as ::windows::core::Abi>::Abi as *const <SpatialSurfaceMeshOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            UpdateTime: UpdateTime::<Impl, IMPL_OFFSET>,
            TryGetBounds: TryGetBounds::<Impl, IMPL_OFFSET>,
            TryComputeLatestMeshAsync: TryComputeLatestMeshAsync::<Impl, IMPL_OFFSET>,
            TryComputeLatestMeshWithOptionsAsync: TryComputeLatestMeshWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceMeshImpl: Sized {
    fn SurfaceInfo(&mut self) -> ::windows::core::Result<SpatialSurfaceInfo>;
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn TriangleIndices(&mut self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer>;
    fn VertexPositions(&mut self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer>;
    fn VertexPositionScale(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn VertexNormals(&mut self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceMesh {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMesh";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialSurfaceMeshVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceMeshImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceMeshVtbl {
        unsafe extern "system" fn SurfaceInfo<Impl: ISpatialSurfaceMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SurfaceInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialSurfaceMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TriangleIndices<Impl: ISpatialSurfaceMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriangleIndices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VertexPositions<Impl: ISpatialSurfaceMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexPositions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VertexPositionScale<Impl: ISpatialSurfaceMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexPositionScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VertexNormals<Impl: ISpatialSurfaceMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexNormals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceMesh, BASE_OFFSET>(),
            SurfaceInfo: SurfaceInfo::<Impl, IMPL_OFFSET>,
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
            TriangleIndices: TriangleIndices::<Impl, IMPL_OFFSET>,
            VertexPositions: VertexPositions::<Impl, IMPL_OFFSET>,
            VertexPositionScale: VertexPositionScale::<Impl, IMPL_OFFSET>,
            VertexNormals: VertexNormals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceMesh as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceMeshBufferImpl: Sized {
    fn Format(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn Stride(&mut self) -> ::windows::core::Result<u32>;
    fn ElementCount(&mut self) -> ::windows::core::Result<u32>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceMeshBuffer {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshBuffer";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISpatialSurfaceMeshBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceMeshBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceMeshBufferVtbl {
        unsafe extern "system" fn Format<Impl: ISpatialSurfaceMeshBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stride<Impl: ISpatialSurfaceMeshBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementCount<Impl: ISpatialSurfaceMeshBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: ISpatialSurfaceMeshBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceMeshBuffer, BASE_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            Stride: Stride::<Impl, IMPL_OFFSET>,
            ElementCount: ElementCount::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceMeshBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceMeshOptionsImpl: Sized {
    fn VertexPositionFormat(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetVertexPositionFormat(&mut self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn TriangleIndexFormat(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetTriangleIndexFormat(&mut self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn VertexNormalFormat(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetVertexNormalFormat(&mut self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn IncludeVertexNormals(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncludeVertexNormals(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceMeshOptions {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptions";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ISpatialSurfaceMeshOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceMeshOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceMeshOptionsVtbl {
        unsafe extern "system" fn VertexPositionFormat<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexPositionFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexPositionFormat<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVertexPositionFormat(value).into()
        }
        unsafe extern "system" fn TriangleIndexFormat<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriangleIndexFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriangleIndexFormat<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTriangleIndexFormat(value).into()
        }
        unsafe extern "system" fn VertexNormalFormat<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexNormalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexNormalFormat<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVertexNormalFormat(value).into()
        }
        unsafe extern "system" fn IncludeVertexNormals<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeVertexNormals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeVertexNormals<Impl: ISpatialSurfaceMeshOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeVertexNormals(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceMeshOptions, BASE_OFFSET>(),
            VertexPositionFormat: VertexPositionFormat::<Impl, IMPL_OFFSET>,
            SetVertexPositionFormat: SetVertexPositionFormat::<Impl, IMPL_OFFSET>,
            TriangleIndexFormat: TriangleIndexFormat::<Impl, IMPL_OFFSET>,
            SetTriangleIndexFormat: SetTriangleIndexFormat::<Impl, IMPL_OFFSET>,
            VertexNormalFormat: VertexNormalFormat::<Impl, IMPL_OFFSET>,
            SetVertexNormalFormat: SetVertexNormalFormat::<Impl, IMPL_OFFSET>,
            IncludeVertexNormals: IncludeVertexNormals::<Impl, IMPL_OFFSET>,
            SetIncludeVertexNormals: SetIncludeVertexNormals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceMeshOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceMeshOptionsStaticsImpl: Sized {
    fn SupportedVertexPositionFormats(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>;
    fn SupportedTriangleIndexFormats(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>;
    fn SupportedVertexNormalFormats(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceMeshOptionsStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptionsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ISpatialSurfaceMeshOptionsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceMeshOptionsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceMeshOptionsStaticsVtbl {
        unsafe extern "system" fn SupportedVertexPositionFormats<Impl: ISpatialSurfaceMeshOptionsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedVertexPositionFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTriangleIndexFormats<Impl: ISpatialSurfaceMeshOptionsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTriangleIndexFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedVertexNormalFormats<Impl: ISpatialSurfaceMeshOptionsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedVertexNormalFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceMeshOptionsStatics, BASE_OFFSET>(),
            SupportedVertexPositionFormats: SupportedVertexPositionFormats::<Impl, IMPL_OFFSET>,
            SupportedTriangleIndexFormats: SupportedTriangleIndexFormats::<Impl, IMPL_OFFSET>,
            SupportedVertexNormalFormats: SupportedVertexNormalFormats::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceMeshOptionsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceObserverImpl: Sized {
    fn GetObservedSurfaces(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>>;
    fn SetBoundingVolume(&mut self, bounds: &::core::option::Option<super::SpatialBoundingVolume>) -> ::windows::core::Result<()>;
    fn SetBoundingVolumes(&mut self, bounds: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume>>) -> ::windows::core::Result<()>;
    fn ObservedSurfacesChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveObservedSurfacesChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceObserver {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserver";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISpatialSurfaceObserverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceObserverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceObserverVtbl {
        unsafe extern "system" fn GetObservedSurfaces<Impl: ISpatialSurfaceObserverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObservedSurfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundingVolume<Impl: ISpatialSurfaceObserverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundingVolume(&*(&bounds as *const <super::SpatialBoundingVolume as ::windows::core::Abi>::Abi as *const <super::SpatialBoundingVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetBoundingVolumes<Impl: ISpatialSurfaceObserverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundingVolumes(&*(&bounds as *const <super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ObservedSurfacesChanged<Impl: ISpatialSurfaceObserverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObservedSurfacesChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveObservedSurfacesChanged<Impl: ISpatialSurfaceObserverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveObservedSurfacesChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceObserver, BASE_OFFSET>(),
            GetObservedSurfaces: GetObservedSurfaces::<Impl, IMPL_OFFSET>,
            SetBoundingVolume: SetBoundingVolume::<Impl, IMPL_OFFSET>,
            SetBoundingVolumes: SetBoundingVolumes::<Impl, IMPL_OFFSET>,
            ObservedSurfacesChanged: ObservedSurfacesChanged::<Impl, IMPL_OFFSET>,
            RemoveObservedSurfacesChanged: RemoveObservedSurfacesChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceObserver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceObserverStaticsImpl: Sized {
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceObserverStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialSurfaceObserverStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceObserverStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceObserverStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: ISpatialSurfaceObserverStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceObserverStatics, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceObserverStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialSurfaceObserverStatics2Impl: Sized + ISpatialSurfaceObserverStaticsImpl {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialSurfaceObserverStatics2 {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialSurfaceObserverStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialSurfaceObserverStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialSurfaceObserverStatics2Vtbl {
        unsafe extern "system" fn IsSupported<Impl: ISpatialSurfaceObserverStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialSurfaceObserverStatics2, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialSurfaceObserverStatics2 as ::windows::core::Interface>::IID
    }
}
