#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicApplicationPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHolographicApplicationPreviewStatics {
    type Vtable = IHolographicApplicationPreviewStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicApplicationPreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe038691_2a3a_45a9_a208_7bed691919f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicApplicationPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCurrentViewPresentedOnHolographicDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub IsHolographicActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatedeventargs: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    IsHolographicActivation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IHolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IHolographicKeyboardPlacementOverridePreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a8ce3a_dfde_5a14_8d5f_182c526dd9c4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverrideWithMaxSize: usize,
    #[cfg(feature = "deprecated")]
    pub ResetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResetPlacementOverride: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IHolographicKeyboardPlacementOverridePreviewStatics {
    type Vtable = IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IHolographicKeyboardPlacementOverridePreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202e6039_1ff6_5a06_aac4_a5e24fa3ec4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`*"]
pub struct HolographicApplicationPreview;
impl HolographicApplicationPreview {
    pub fn IsCurrentViewPresentedOnHolographicDisplay() -> ::windows::core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCurrentViewPresentedOnHolographicDisplay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn IsHolographicActivation<P0, E0>(activatedeventargs: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Activation::IActivatedEventArgs>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHolographicActivation)(::windows::core::Vtable::as_raw(this), activatedeventargs.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicApplicationPreviewStatics<R, F: FnOnce(&IHolographicApplicationPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HolographicApplicationPreview, IHolographicApplicationPreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for HolographicApplicationPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview";
}
#[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct HolographicKeyboardPlacementOverridePreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl HolographicKeyboardPlacementOverridePreview {
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverride(&self, coordinatesystem: &super::super::super::Perception::Spatial::SpatialCoordinateSystem, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPlacementOverride)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), topcenterposition, normal).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverrideWithMaxSize(&self, coordinatesystem: &super::super::super::Perception::Spatial::SpatialCoordinateSystem, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPlacementOverrideWithMaxSize)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), topcenterposition, normal, maxsize).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ResetPlacementOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ResetPlacementOverride)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<HolographicKeyboardPlacementOverridePreview> {
        Self::IHolographicKeyboardPlacementOverridePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IHolographicKeyboardPlacementOverridePreviewStatics<R, F: FnOnce(&IHolographicKeyboardPlacementOverridePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HolographicKeyboardPlacementOverridePreview, IHolographicKeyboardPlacementOverridePreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HolographicKeyboardPlacementOverridePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for HolographicKeyboardPlacementOverridePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for HolographicKeyboardPlacementOverridePreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for HolographicKeyboardPlacementOverridePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicKeyboardPlacementOverridePreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for HolographicKeyboardPlacementOverridePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview;{c8a8ce3a-dfde-5a14-8d5f-182c526dd9c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for HolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for HolographicKeyboardPlacementOverridePreview {
    const IID: ::windows::core::GUID = <IHolographicKeyboardPlacementOverridePreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for HolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(HolographicKeyboardPlacementOverridePreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for HolographicKeyboardPlacementOverridePreview {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for HolographicKeyboardPlacementOverridePreview {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
