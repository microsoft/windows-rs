#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub struct HolographicApplicationPreview {}
impl HolographicApplicationPreview {
    pub fn IsCurrentViewPresentedOnHolographicDisplay() -> ::windows::core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn IsHolographicActivation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Activation::IActivatedEventArgs>>(activatedeventargs: Param0) -> ::windows::core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activatedeventargs.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHolographicApplicationPreviewStatics<R, F: FnOnce(&IHolographicApplicationPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HolographicApplicationPreview, IHolographicApplicationPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for HolographicApplicationPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview";
}
#[repr(transparent)]
pub struct HolographicKeyboardPlacementOverridePreview(::windows::core::IUnknown);
impl HolographicKeyboardPlacementOverridePreview {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverride<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, topcenterposition: Param1, normal: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), normal.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverrideWithMaxSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>>(&self, coordinatesystem: Param0, topcenterposition: Param1, normal: Param2, maxsize: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), normal.into_param().abi(), maxsize.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ResetPlacementOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<HolographicKeyboardPlacementOverridePreview> {
        Self::IHolographicKeyboardPlacementOverridePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicKeyboardPlacementOverridePreview>(result__)
        })
    }
    pub fn IHolographicKeyboardPlacementOverridePreviewStatics<R, F: FnOnce(&IHolographicKeyboardPlacementOverridePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HolographicKeyboardPlacementOverridePreview, IHolographicKeyboardPlacementOverridePreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HolographicKeyboardPlacementOverridePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicKeyboardPlacementOverridePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicKeyboardPlacementOverridePreview {}
unsafe impl ::windows::core::RuntimeType for HolographicKeyboardPlacementOverridePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview;{c8a8ce3a-dfde-5a14-8d5f-182c526dd9c4})");
}
unsafe impl ::windows::core::Interface for HolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreviewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a8ce3a_dfde_5a14_8d5f_182c526dd9c4);
}
impl ::windows::core::RuntimeName for HolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
}
impl ::core::convert::From<HolographicKeyboardPlacementOverridePreview> for ::windows::core::IUnknown {
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicKeyboardPlacementOverridePreview> for ::windows::core::IUnknown {
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicKeyboardPlacementOverridePreview> for ::windows::core::IInspectable {
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicKeyboardPlacementOverridePreview> for ::windows::core::IInspectable {
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicKeyboardPlacementOverridePreview {}
unsafe impl ::core::marker::Sync for HolographicKeyboardPlacementOverridePreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicApplicationPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicApplicationPreviewStatics {
    type Vtable = IHolographicApplicationPreviewStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe038691_2a3a_45a9_a208_7bed691919f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicApplicationPreviewStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatedeventargs: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreviewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a8ce3a_dfde_5a14_8d5f_182c526dd9c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreviewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHolographicKeyboardPlacementOverridePreviewStatics {
    type Vtable = IHolographicKeyboardPlacementOverridePreviewStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202e6039_1ff6_5a06_aac4_a5e24fa3ec4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreviewStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
