#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboard(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicKeyboard {
    type Vtable = IHolographicKeyboard_Vtbl;
}
impl ::core::clone::Clone for IHolographicKeyboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHolographicKeyboard {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07dd0893_aa21_5e6f_a91b_11b2b3fd7be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboard_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverrideWithMaxSize: usize,
    pub ResetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboardStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicKeyboardStatics {
    type Vtable = IHolographicKeyboardStatics_Vtbl;
}
impl ::core::clone::Clone for IHolographicKeyboardStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHolographicKeyboardStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb676c624_63d7_58cf_b06b_08baa032a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicKeyboard(::windows::core::IUnknown);
impl HolographicKeyboard {
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverride(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlacementOverride)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), topcenterposition, orientation).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverrideWithMaxSize(&self, coordinatesystem: &super::super::Perception::Spatial::SpatialCoordinateSystem, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlacementOverrideWithMaxSize)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), topcenterposition, orientation, maxsize).ok() }
    }
    pub fn ResetPlacementOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResetPlacementOverride)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<HolographicKeyboard> {
        Self::IHolographicKeyboardStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HolographicKeyboard>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicKeyboardStatics<R, F: FnOnce(&IHolographicKeyboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HolographicKeyboard, IHolographicKeyboardStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for HolographicKeyboard {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Holographic.HolographicKeyboard;{07dd0893-aa21-5e6f-a91b-11b2b3fd7be3})");
}
impl ::core::clone::Clone for HolographicKeyboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HolographicKeyboard {
    type Vtable = IHolographicKeyboard_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HolographicKeyboard {
    const IID: ::windows::core::GUID = <IHolographicKeyboard as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HolographicKeyboard {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.HolographicKeyboard";
}
::windows::imp::interface_hierarchy!(HolographicKeyboard, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HolographicKeyboard {}
unsafe impl ::core::marker::Sync for HolographicKeyboard {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
