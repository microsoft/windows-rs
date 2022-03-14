#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`*"]
pub struct HolographicApplicationPreview {}
impl HolographicApplicationPreview {
    #[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`*"]
    pub fn IsCurrentViewPresentedOnHolographicDisplay() -> ::windows::core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentViewPresentedOnHolographicDisplay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn IsHolographicActivation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Activation::IActivatedEventArgs>>(activatedeventargs: Param0) -> ::windows::core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHolographicActivation)(::core::mem::transmute_copy(this), activatedeventargs.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicApplicationPreviewStatics<R, F: FnOnce(&IHolographicApplicationPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HolographicApplicationPreview, IHolographicApplicationPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
    #[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`, `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverride<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, topcenterposition: Param1, normal: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlacementOverride)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), normal.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`, `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverrideWithMaxSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>>(&self, coordinatesystem: Param0, topcenterposition: Param1, normal: Param2, maxsize: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlacementOverrideWithMaxSize)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), normal.into_param().abi(), maxsize.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ResetPlacementOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResetPlacementOverride)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Holographic\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<HolographicKeyboardPlacementOverridePreview> {
        Self::IHolographicKeyboardPlacementOverridePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicKeyboardPlacementOverridePreview>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IHolographicKeyboardPlacementOverridePreviewStatics<R, F: FnOnce(&IHolographicKeyboardPlacementOverridePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HolographicKeyboardPlacementOverridePreview, IHolographicKeyboardPlacementOverridePreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for HolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_Vtbl;
    const IID: ::windows::core::GUID = <IHolographicKeyboardPlacementOverridePreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for HolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicKeyboardPlacementOverridePreview> for ::windows::core::IUnknown {
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicKeyboardPlacementOverridePreview> for ::windows::core::IUnknown {
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicKeyboardPlacementOverridePreview> for ::windows::core::IInspectable {
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicKeyboardPlacementOverridePreview> for ::windows::core::IInspectable {
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for HolographicKeyboardPlacementOverridePreview {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for HolographicKeyboardPlacementOverridePreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicApplicationPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicApplicationPreviewStatics {
    type Vtable = IHolographicApplicationPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe038691_2a3a_45a9_a208_7bed691919f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicApplicationPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCurrentViewPresentedOnHolographicDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub IsHolographicActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatedeventargs: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    IsHolographicActivation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IHolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a8ce3a_dfde_5a14_8d5f_182c526dd9c4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IHolographicKeyboardPlacementOverridePreviewStatics {
    type Vtable = IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202e6039_1ff6_5a06_aac4_a5e24fa3ec4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
