#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicKeyboard(pub ::windows::core::IInspectable);
impl HolographicKeyboard {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverride<'a, Param0: ::windows::core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, coordinatesystem: Param0, topcenterposition: Param1, orientation: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverrideWithMaxSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector2>>(
        &self,
        coordinatesystem: Param0,
        topcenterposition: Param1,
        orientation: Param2,
        maxsize: Param3,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi(), maxsize.into_param().abi()).ok() }
    }
    pub fn ResetPlacementOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<HolographicKeyboard> {
        Self::IHolographicKeyboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicKeyboard>(result__)
        })
    }
    pub fn IHolographicKeyboardStatics<R, F: FnOnce(&IHolographicKeyboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HolographicKeyboard, IHolographicKeyboardStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for HolographicKeyboard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Holographic.HolographicKeyboard;{07dd0893-aa21-5e6f-a91b-11b2b3fd7be3})");
}
unsafe impl ::windows::core::Interface for HolographicKeyboard {
    type Vtable = IHolographicKeyboard_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07dd0893_aa21_5e6f_a91b_11b2b3fd7be3);
}
impl ::windows::core::RuntimeName for HolographicKeyboard {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.HolographicKeyboard";
}
impl ::core::convert::From<HolographicKeyboard> for ::windows::core::IUnknown {
    fn from(value: HolographicKeyboard) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicKeyboard> for ::windows::core::IUnknown {
    fn from(value: &HolographicKeyboard) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicKeyboard> for ::windows::core::IInspectable {
    fn from(value: HolographicKeyboard) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicKeyboard> for ::windows::core::IInspectable {
    fn from(value: &HolographicKeyboard) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HolographicKeyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicKeyboard {}
unsafe impl ::core::marker::Sync for HolographicKeyboard {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicKeyboard(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHolographicKeyboard {
    type Vtable = IHolographicKeyboard_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07dd0893_aa21_5e6f_a91b_11b2b3fd7be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboard_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicKeyboardStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHolographicKeyboardStatics {
    type Vtable = IHolographicKeyboardStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb676c624_63d7_58cf_b06b_08baa032a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
