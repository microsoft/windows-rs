#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1656002251, 21405, 21315, [166, 95, 65, 245, 48, 14, 199, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPalmRejectionDelayZonePreviewStatics {
    type Vtable = IPalmRejectionDelayZonePreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3455016672, 37840, 21417, [143, 14, 154, 55, 159, 143, 117, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputpanelvisual: ::windows::runtime::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputpanelvisual: ::windows::runtime::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: ::windows::runtime::RawPtr, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))] usize,
);
#[doc = "*Required features: `UI_Input_Inking_Preview`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PalmRejectionDelayZonePreview(::windows::runtime::IInspectable);
impl PalmRejectionDelayZonePreview {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`, `UI_Composition`*"]
    pub fn CreateForVisual<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Rect>>(inputpanelvisual: Param0, inputpanelrect: Param1) -> ::windows::runtime::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`, `UI_Composition`*"]
    pub fn CreateForVisualWithViewportClip<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Rect>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Visual>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Rect>>(
        inputpanelvisual: Param0,
        inputpanelrect: Param1,
        viewportvisual: Param2,
        viewportrect: Param3,
    ) -> ::windows::runtime::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), viewportvisual.into_param().abi(), viewportrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview;{62b496cb-539d-5343-a65f-41f5300ec70c})");
}
unsafe impl ::windows::runtime::Interface for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1656002251, 21405, 21315, [166, 95, 65, 245, 48, 14, 199, 12]);
}
impl ::windows::runtime::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
impl ::std::convert::From<PalmRejectionDelayZonePreview> for ::windows::runtime::IUnknown {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PalmRejectionDelayZonePreview> for ::windows::runtime::IUnknown {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PalmRejectionDelayZonePreview> for ::windows::runtime::IInspectable {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PalmRejectionDelayZonePreview> for ::windows::runtime::IInspectable {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PalmRejectionDelayZonePreview) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PalmRejectionDelayZonePreview) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::IClosable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::IClosable> for &PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PalmRejectionDelayZonePreview {}
unsafe impl ::std::marker::Sync for PalmRejectionDelayZonePreview {}
