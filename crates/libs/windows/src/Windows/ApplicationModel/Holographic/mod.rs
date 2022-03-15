#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicKeyboard(::windows::core::IUnknown);
impl HolographicKeyboard {
    #[doc = "*Required features: `\"ApplicationModel_Holographic\"`, `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverride<'a, Param0: ::windows::core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, coordinatesystem: Param0, topcenterposition: Param1, orientation: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlacementOverride)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Holographic\"`, `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverrideWithMaxSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector2>>(&self, coordinatesystem: Param0, topcenterposition: Param1, orientation: Param2, maxsize: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlacementOverrideWithMaxSize)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi(), maxsize.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Holographic\"`*"]
    pub fn ResetPlacementOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResetPlacementOverride)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Holographic\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<HolographicKeyboard> {
        Self::IHolographicKeyboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicKeyboard>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicKeyboardStatics<R, F: FnOnce(&IHolographicKeyboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HolographicKeyboard, IHolographicKeyboardStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HolographicKeyboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicKeyboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicKeyboard {}
impl ::core::fmt::Debug for HolographicKeyboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicKeyboard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicKeyboard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Holographic.HolographicKeyboard;{07dd0893-aa21-5e6f-a91b-11b2b3fd7be3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HolographicKeyboard {
    type Vtable = IHolographicKeyboard_Vtbl;
    const IID: ::windows::core::GUID = <IHolographicKeyboard as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HolographicKeyboard {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.HolographicKeyboard";
}
impl ::core::convert::From<HolographicKeyboard> for ::windows::core::IUnknown {
    fn from(value: HolographicKeyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicKeyboard> for ::windows::core::IUnknown {
    fn from(value: &HolographicKeyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicKeyboard> for ::windows::core::IInspectable {
    fn from(value: HolographicKeyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicKeyboard> for ::windows::core::IInspectable {
    fn from(value: &HolographicKeyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicKeyboard {}
unsafe impl ::core::marker::Sync for HolographicKeyboard {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboard(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicKeyboard {
    type Vtable = IHolographicKeyboard_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07dd0893_aa21_5e6f_a91b_11b2b3fd7be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboard_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverrideWithMaxSize: usize,
    pub ResetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboardStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicKeyboardStatics {
    type Vtable = IHolographicKeyboardStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb676c624_63d7_58cf_b06b_08baa032a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
