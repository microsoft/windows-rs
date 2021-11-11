#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreviewStatics {
    type Vtable = IPalmRejectionDelayZonePreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputpanelvisual: ::windows::core::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputpanelvisual: ::windows::core::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: ::windows::core::RawPtr, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))] usize,
);
#[doc = "*Required features: `UI_Input_Inking_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PalmRejectionDelayZonePreview(pub ::windows::core::IInspectable);
impl PalmRejectionDelayZonePreview {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`, `UI_Composition`*"]
    pub fn CreateForVisual<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>>(inputpanelvisual: Param0, inputpanelrect: Param1) -> ::windows::core::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`, `UI_Composition`*"]
    pub fn CreateForVisualWithViewportClip<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>, Param2: ::windows::core::IntoParam<'a, super::super::super::Composition::Visual>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>>(
        inputpanelvisual: Param0,
        inputpanelrect: Param1,
        viewportvisual: Param2,
        viewportrect: Param3,
    ) -> ::windows::core::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), viewportvisual.into_param().abi(), viewportrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview;{62b496cb-539d-5343-a65f-41f5300ec70c})");
}
unsafe impl ::windows::core::Interface for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
impl ::windows::core::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows::core::IUnknown {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows::core::IUnknown {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows::core::IInspectable {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows::core::IInspectable {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PalmRejectionDelayZonePreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PalmRejectionDelayZonePreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IClosable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IClosable> for &PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PalmRejectionDelayZonePreview {}
unsafe impl ::core::marker::Sync for PalmRejectionDelayZonePreview {}
