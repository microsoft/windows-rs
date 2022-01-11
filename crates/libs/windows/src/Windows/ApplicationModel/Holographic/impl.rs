#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicKeyboardImpl: Sized {
    fn SetPlacementOverride(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn SetPlacementOverrideWithMaxSize(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion, maxsize: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ResetPlacementOverride(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicKeyboard {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.IHolographicKeyboard";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicKeyboardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicKeyboardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicKeyboardVtbl {
        unsafe extern "system" fn SetPlacementOverride<Impl: IHolographicKeyboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPlacementOverride(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&topcenterposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&orientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPlacementOverrideWithMaxSize<Impl: IHolographicKeyboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPlacementOverrideWithMaxSize(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&topcenterposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&orientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
                    &*(&maxsize as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn ResetPlacementOverride<Impl: IHolographicKeyboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetPlacementOverride().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicKeyboard>, ::windows::core::GetTrustLevel, SetPlacementOverride::<Impl, IMPL_OFFSET>, SetPlacementOverrideWithMaxSize::<Impl, IMPL_OFFSET>, ResetPlacementOverride::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicKeyboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicKeyboardStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<HolographicKeyboard>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicKeyboardStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.IHolographicKeyboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicKeyboardStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicKeyboardStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicKeyboardStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IHolographicKeyboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicKeyboardStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicKeyboardStatics as ::windows::core::Interface>::IID
    }
}
