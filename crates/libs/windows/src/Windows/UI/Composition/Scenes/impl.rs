#[cfg(feature = "implement_exclusive")]
pub trait ISceneBoundingBoxImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Extents(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Max(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Min(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneBoundingBox {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneBoundingBox";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneBoundingBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneBoundingBoxImpl, const OFFSET: isize>() -> ISceneBoundingBoxVtbl {
        unsafe extern "system" fn Center<Impl: ISceneBoundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extents<Impl: ISceneBoundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extents() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: ISceneBoundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: ISceneBoundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: ISceneBoundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneBoundingBox>, ::windows::core::GetTrustLevel, Center::<Impl, OFFSET>, Extents::<Impl, OFFSET>, Max::<Impl, OFFSET>, Min::<Impl, OFFSET>, Size::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneComponentImpl: Sized {
    fn ComponentType(&self) -> ::windows::core::Result<SceneComponentType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneComponent";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneComponentImpl, const OFFSET: isize>() -> ISceneComponentVtbl {
        unsafe extern "system" fn ComponentType<Impl: ISceneComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SceneComponentType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComponentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneComponent>, ::windows::core::GetTrustLevel, ComponentType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneComponentCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneComponentCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneComponentCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneComponentCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneComponentCollectionImpl, const OFFSET: isize>() -> ISceneComponentCollectionVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneComponentCollection>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneComponentFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneComponentFactory {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneComponentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneComponentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneComponentFactoryImpl, const OFFSET: isize>() -> ISceneComponentFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneComponentFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMaterialImpl, const OFFSET: isize>() -> ISceneMaterialVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMaterial>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMaterialFactory {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMaterialFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMaterialFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMaterialFactoryImpl, const OFFSET: isize>() -> ISceneMaterialFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMaterialFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialInputImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMaterialInput";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMaterialInputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMaterialInputImpl, const OFFSET: isize>() -> ISceneMaterialInputVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMaterialInput>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialInputFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMaterialInputFactory {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMaterialInputFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMaterialInputFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMaterialInputFactoryImpl, const OFFSET: isize>() -> ISceneMaterialInputFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMaterialInputFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshImpl: Sized {
    fn Bounds(&self) -> ::windows::core::Result<SceneBoundingBox>;
    fn PrimitiveTopology(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology>;
    fn SetPrimitiveTopology(&self, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows::core::Result<()>;
    fn FillMeshAttribute(&self, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: &::core::option::Option<super::super::super::Foundation::MemoryBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMesh {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMesh";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMeshVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMeshImpl, const OFFSET: isize>() -> ISceneMeshVtbl {
        unsafe extern "system" fn Bounds<Impl: ISceneMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimitiveTopology<Impl: ISceneMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimitiveTopology() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimitiveTopology<Impl: ISceneMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrimitiveTopology(value).into()
        }
        unsafe extern "system" fn FillMeshAttribute<Impl: ISceneMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FillMeshAttribute(semantic, format, &*(&memory as *const <super::super::super::Foundation::MemoryBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::MemoryBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMesh>, ::windows::core::GetTrustLevel, Bounds::<Impl, OFFSET>, PrimitiveTopology::<Impl, OFFSET>, SetPrimitiveTopology::<Impl, OFFSET>, FillMeshAttribute::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshMaterialAttributeMapImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMeshMaterialAttributeMap {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMeshMaterialAttributeMap";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMeshMaterialAttributeMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMeshMaterialAttributeMapImpl, const OFFSET: isize>() -> ISceneMeshMaterialAttributeMapVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMeshMaterialAttributeMap>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshRendererComponentImpl: Sized {
    fn Material(&self) -> ::windows::core::Result<SceneMaterial>;
    fn SetMaterial(&self, value: &::core::option::Option<SceneMaterial>) -> ::windows::core::Result<()>;
    fn Mesh(&self) -> ::windows::core::Result<SceneMesh>;
    fn SetMesh(&self, value: &::core::option::Option<SceneMesh>) -> ::windows::core::Result<()>;
    fn UVMappings(&self) -> ::windows::core::Result<SceneMeshMaterialAttributeMap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMeshRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMeshRendererComponent";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMeshRendererComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMeshRendererComponentImpl, const OFFSET: isize>() -> ISceneMeshRendererComponentVtbl {
        unsafe extern "system" fn Material<Impl: ISceneMeshRendererComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Material() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaterial<Impl: ISceneMeshRendererComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaterial(&*(&value as *const <SceneMaterial as ::windows::core::Abi>::Abi as *const <SceneMaterial as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Mesh<Impl: ISceneMeshRendererComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mesh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMesh<Impl: ISceneMeshRendererComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMesh(&*(&value as *const <SceneMesh as ::windows::core::Abi>::Abi as *const <SceneMesh as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UVMappings<Impl: ISceneMeshRendererComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UVMappings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMeshRendererComponent>, ::windows::core::GetTrustLevel, Material::<Impl, OFFSET>, SetMaterial::<Impl, OFFSET>, Mesh::<Impl, OFFSET>, SetMesh::<Impl, OFFSET>, UVMappings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshRendererComponentStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneMeshRendererComponent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMeshRendererComponentStatics {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMeshRendererComponentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMeshRendererComponentStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMeshRendererComponentStaticsImpl, const OFFSET: isize>() -> ISceneMeshRendererComponentStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ISceneMeshRendererComponentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMeshRendererComponentStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneMesh>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMeshStatics {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMeshStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMeshStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMeshStaticsImpl, const OFFSET: isize>() -> ISceneMeshStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ISceneMeshStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMeshStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMetallicRoughnessMaterialImpl: Sized {
    fn BaseColorInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetBaseColorInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn BaseColorFactor(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector4>;
    fn SetBaseColorFactor(&self, value: &super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn MetallicFactor(&self) -> ::windows::core::Result<f32>;
    fn SetMetallicFactor(&self, value: f32) -> ::windows::core::Result<()>;
    fn MetallicRoughnessInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetMetallicRoughnessInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn RoughnessFactor(&self) -> ::windows::core::Result<f32>;
    fn SetRoughnessFactor(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMetallicRoughnessMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMetallicRoughnessMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>() -> ISceneMetallicRoughnessMaterialVtbl {
        unsafe extern "system" fn BaseColorInput<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseColorInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseColorInput<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseColorInput(&*(&value as *const <SceneMaterialInput as ::windows::core::Abi>::Abi as *const <SceneMaterialInput as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseColorFactor<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseColorFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseColorFactor<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseColorFactor(&*(&value as *const <super::super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MetallicFactor<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MetallicFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetallicFactor<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMetallicFactor(value).into()
        }
        unsafe extern "system" fn MetallicRoughnessInput<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MetallicRoughnessInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetallicRoughnessInput<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMetallicRoughnessInput(&*(&value as *const <SceneMaterialInput as ::windows::core::Abi>::Abi as *const <SceneMaterialInput as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RoughnessFactor<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoughnessFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoughnessFactor<Impl: ISceneMetallicRoughnessMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoughnessFactor(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISceneMetallicRoughnessMaterial>,
            ::windows::core::GetTrustLevel,
            BaseColorInput::<Impl, OFFSET>,
            SetBaseColorInput::<Impl, OFFSET>,
            BaseColorFactor::<Impl, OFFSET>,
            SetBaseColorFactor::<Impl, OFFSET>,
            MetallicFactor::<Impl, OFFSET>,
            SetMetallicFactor::<Impl, OFFSET>,
            MetallicRoughnessInput::<Impl, OFFSET>,
            SetMetallicRoughnessInput::<Impl, OFFSET>,
            RoughnessFactor::<Impl, OFFSET>,
            SetRoughnessFactor::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMetallicRoughnessMaterialStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneMetallicRoughnessMaterial>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneMetallicRoughnessMaterialStatics {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterialStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneMetallicRoughnessMaterialStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneMetallicRoughnessMaterialStaticsImpl, const OFFSET: isize>() -> ISceneMetallicRoughnessMaterialStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ISceneMetallicRoughnessMaterialStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneMetallicRoughnessMaterialStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneModelTransformImpl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
    fn SetOrientation(&self, value: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAxis(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Translation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetTranslation(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneModelTransform {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneModelTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneModelTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneModelTransformImpl, const OFFSET: isize>() -> ISceneModelTransformVtbl {
        unsafe extern "system" fn Orientation<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(&*(&value as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn RotationAxis<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAxis() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAxis<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAxis(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Scale<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Translation<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Translation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslation<Impl: ISceneModelTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslation(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISceneModelTransform>,
            ::windows::core::GetTrustLevel,
            Orientation::<Impl, OFFSET>,
            SetOrientation::<Impl, OFFSET>,
            RotationAngle::<Impl, OFFSET>,
            SetRotationAngle::<Impl, OFFSET>,
            RotationAngleInDegrees::<Impl, OFFSET>,
            SetRotationAngleInDegrees::<Impl, OFFSET>,
            RotationAxis::<Impl, OFFSET>,
            SetRotationAxis::<Impl, OFFSET>,
            Scale::<Impl, OFFSET>,
            SetScale::<Impl, OFFSET>,
            Translation::<Impl, OFFSET>,
            SetTranslation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneNodeImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<SceneNodeCollection>;
    fn Components(&self) -> ::windows::core::Result<SceneComponentCollection>;
    fn Parent(&self) -> ::windows::core::Result<SceneNode>;
    fn Transform(&self) -> ::windows::core::Result<SceneModelTransform>;
    fn FindFirstComponentOfType(&self, value: SceneComponentType) -> ::windows::core::Result<SceneComponent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneNode {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneNode";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneNodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneNodeImpl, const OFFSET: isize>() -> ISceneNodeVtbl {
        unsafe extern "system" fn Children<Impl: ISceneNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Components<Impl: ISceneNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: ISceneNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Impl: ISceneNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstComponentOfType<Impl: ISceneNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SceneComponentType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstComponentOfType(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneNode>, ::windows::core::GetTrustLevel, Children::<Impl, OFFSET>, Components::<Impl, OFFSET>, Parent::<Impl, OFFSET>, Transform::<Impl, OFFSET>, FindFirstComponentOfType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneNodeCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneNodeCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneNodeCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneNodeCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneNodeCollectionImpl, const OFFSET: isize>() -> ISceneNodeCollectionVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneNodeCollection>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneNodeStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneNode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneNodeStatics {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneNodeStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneNodeStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneNodeStaticsImpl, const OFFSET: isize>() -> ISceneNodeStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ISceneNodeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneNodeStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneObjectImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneObject {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneObject";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneObjectImpl, const OFFSET: isize>() -> ISceneObjectVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneObject>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneObjectFactory {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneObjectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneObjectFactoryImpl, const OFFSET: isize>() -> ISceneObjectFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneObjectFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScenePbrMaterialImpl: Sized {
    fn AlphaCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetAlphaCutoff(&self, value: f32) -> ::windows::core::Result<()>;
    fn AlphaMode(&self) -> ::windows::core::Result<SceneAlphaMode>;
    fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::core::Result<()>;
    fn EmissiveInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetEmissiveInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn EmissiveFactor(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetEmissiveFactor(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn IsDoubleSided(&self) -> ::windows::core::Result<bool>;
    fn SetIsDoubleSided(&self, value: bool) -> ::windows::core::Result<()>;
    fn NormalInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetNormalInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn NormalScale(&self) -> ::windows::core::Result<f32>;
    fn SetNormalScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn OcclusionInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetOcclusionInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn OcclusionStrength(&self) -> ::windows::core::Result<f32>;
    fn SetOcclusionStrength(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScenePbrMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.IScenePbrMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IScenePbrMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScenePbrMaterialImpl, const OFFSET: isize>() -> IScenePbrMaterialVtbl {
        unsafe extern "system" fn AlphaCutoff<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlphaCutoff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaCutoff<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaCutoff(value).into()
        }
        unsafe extern "system" fn AlphaMode<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SceneAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SceneAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaMode(value).into()
        }
        unsafe extern "system" fn EmissiveInput<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmissiveInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmissiveInput<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmissiveInput(&*(&value as *const <SceneMaterialInput as ::windows::core::Abi>::Abi as *const <SceneMaterialInput as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EmissiveFactor<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmissiveFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmissiveFactor<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmissiveFactor(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsDoubleSided<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleSided() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDoubleSided<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDoubleSided(value).into()
        }
        unsafe extern "system" fn NormalInput<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalInput<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalInput(&*(&value as *const <SceneMaterialInput as ::windows::core::Abi>::Abi as *const <SceneMaterialInput as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalScale<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalScale<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalScale(value).into()
        }
        unsafe extern "system" fn OcclusionInput<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OcclusionInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOcclusionInput<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOcclusionInput(&*(&value as *const <SceneMaterialInput as ::windows::core::Abi>::Abi as *const <SceneMaterialInput as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OcclusionStrength<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OcclusionStrength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOcclusionStrength<Impl: IScenePbrMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOcclusionStrength(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IScenePbrMaterial>,
            ::windows::core::GetTrustLevel,
            AlphaCutoff::<Impl, OFFSET>,
            SetAlphaCutoff::<Impl, OFFSET>,
            AlphaMode::<Impl, OFFSET>,
            SetAlphaMode::<Impl, OFFSET>,
            EmissiveInput::<Impl, OFFSET>,
            SetEmissiveInput::<Impl, OFFSET>,
            EmissiveFactor::<Impl, OFFSET>,
            SetEmissiveFactor::<Impl, OFFSET>,
            IsDoubleSided::<Impl, OFFSET>,
            SetIsDoubleSided::<Impl, OFFSET>,
            NormalInput::<Impl, OFFSET>,
            SetNormalInput::<Impl, OFFSET>,
            NormalScale::<Impl, OFFSET>,
            SetNormalScale::<Impl, OFFSET>,
            OcclusionInput::<Impl, OFFSET>,
            SetOcclusionInput::<Impl, OFFSET>,
            OcclusionStrength::<Impl, OFFSET>,
            SetOcclusionStrength::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScenePbrMaterialFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScenePbrMaterialFactory {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.IScenePbrMaterialFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScenePbrMaterialFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScenePbrMaterialFactoryImpl, const OFFSET: isize>() -> IScenePbrMaterialFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScenePbrMaterialFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneRendererComponentImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneRendererComponent";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneRendererComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneRendererComponentImpl, const OFFSET: isize>() -> ISceneRendererComponentVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneRendererComponent>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneRendererComponentFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneRendererComponentFactory {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneRendererComponentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneRendererComponentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneRendererComponentFactoryImpl, const OFFSET: isize>() -> ISceneRendererComponentFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneRendererComponentFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneSurfaceMaterialInputImpl: Sized {
    fn BitmapInterpolationMode(&self) -> ::windows::core::Result<super::CompositionBitmapInterpolationMode>;
    fn SetBitmapInterpolationMode(&self, value: super::CompositionBitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn Surface(&self) -> ::windows::core::Result<super::ICompositionSurface>;
    fn SetSurface(&self, value: &::core::option::Option<super::ICompositionSurface>) -> ::windows::core::Result<()>;
    fn WrappingUMode(&self) -> ::windows::core::Result<SceneWrappingMode>;
    fn SetWrappingUMode(&self, value: SceneWrappingMode) -> ::windows::core::Result<()>;
    fn WrappingVMode(&self) -> ::windows::core::Result<SceneWrappingMode>;
    fn SetWrappingVMode(&self, value: SceneWrappingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneSurfaceMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInput";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneSurfaceMaterialInputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>() -> ISceneSurfaceMaterialInputVtbl {
        unsafe extern "system" fn BitmapInterpolationMode<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::CompositionBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::CompositionBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapInterpolationMode(value).into()
        }
        unsafe extern "system" fn Surface<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Surface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSurface<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurface(&*(&value as *const <super::ICompositionSurface as ::windows::core::Abi>::Abi as *const <super::ICompositionSurface as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WrappingUMode<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SceneWrappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WrappingUMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWrappingUMode<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SceneWrappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWrappingUMode(value).into()
        }
        unsafe extern "system" fn WrappingVMode<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SceneWrappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WrappingVMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWrappingVMode<Impl: ISceneSurfaceMaterialInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SceneWrappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWrappingVMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISceneSurfaceMaterialInput>,
            ::windows::core::GetTrustLevel,
            BitmapInterpolationMode::<Impl, OFFSET>,
            SetBitmapInterpolationMode::<Impl, OFFSET>,
            Surface::<Impl, OFFSET>,
            SetSurface::<Impl, OFFSET>,
            WrappingUMode::<Impl, OFFSET>,
            SetWrappingUMode::<Impl, OFFSET>,
            WrappingVMode::<Impl, OFFSET>,
            SetWrappingVMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneSurfaceMaterialInputStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneSurfaceMaterialInput>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneSurfaceMaterialInputStatics {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInputStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneSurfaceMaterialInputStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneSurfaceMaterialInputStaticsImpl, const OFFSET: isize>() -> ISceneSurfaceMaterialInputStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ISceneSurfaceMaterialInputStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneSurfaceMaterialInputStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneVisualImpl: Sized {
    fn Root(&self) -> ::windows::core::Result<SceneNode>;
    fn SetRoot(&self, value: &::core::option::Option<SceneNode>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneVisual {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneVisual";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneVisualImpl, const OFFSET: isize>() -> ISceneVisualVtbl {
        unsafe extern "system" fn Root<Impl: ISceneVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Root() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoot<Impl: ISceneVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoot(&*(&value as *const <SceneNode as ::windows::core::Abi>::Abi as *const <SceneNode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneVisual>, ::windows::core::GetTrustLevel, Root::<Impl, OFFSET>, SetRoot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneVisualStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneVisual>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneVisualStatics {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ISceneVisualStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneVisualStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneVisualStaticsImpl, const OFFSET: isize>() -> ISceneVisualStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ISceneVisualStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISceneVisualStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
