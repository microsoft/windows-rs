#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Holographic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HolographicKeyboard(::windows::runtime::IInspectable);
impl HolographicKeyboard {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `ApplicationModel_Holographic`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn SetPlacementOverride<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, coordinatesystem: Param0, topcenterposition: Param1, orientation: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `ApplicationModel_Holographic`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn SetPlacementOverrideWithMaxSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector2>>(
        &self,
        coordinatesystem: Param0,
        topcenterposition: Param1,
        orientation: Param2,
        maxsize: Param3,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi(), maxsize.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Holographic`*"]
    pub fn ResetPlacementOverride(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Holographic`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<HolographicKeyboard> {
        Self::IHolographicKeyboardStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HolographicKeyboard>(result__)
        })
    }
    pub fn IHolographicKeyboardStatics<R, F: FnOnce(&IHolographicKeyboardStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HolographicKeyboard, IHolographicKeyboardStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicKeyboard {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Holographic.HolographicKeyboard;{07dd0893-aa21-5e6f-a91b-11b2b3fd7be3})");
}
unsafe impl ::windows::runtime::Interface for HolographicKeyboard {
    type Vtable = IHolographicKeyboard_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(131926163, 43553, 24175, [169, 27, 17, 178, 179, 253, 123, 227]);
}
impl ::windows::runtime::RuntimeName for HolographicKeyboard {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.HolographicKeyboard";
}
impl ::std::convert::From<HolographicKeyboard> for ::windows::runtime::IUnknown {
    fn from(value: HolographicKeyboard) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HolographicKeyboard> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicKeyboard) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicKeyboard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HolographicKeyboard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HolographicKeyboard> for ::windows::runtime::IInspectable {
    fn from(value: HolographicKeyboard) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HolographicKeyboard> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicKeyboard) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicKeyboard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicKeyboard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HolographicKeyboard {}
unsafe impl ::std::marker::Sync for HolographicKeyboard {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicKeyboard(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicKeyboard {
    type Vtable = IHolographicKeyboard_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(131926163, 43553, 24175, [169, 27, 17, 178, 179, 253, 123, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboard_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicKeyboardStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicKeyboardStatics {
    type Vtable = IHolographicKeyboardStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3061237284, 25559, 22735, [176, 107, 8, 186, 160, 50, 162, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
