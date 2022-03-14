#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreviewStatics {
    type Vtable = IPalmRejectionDelayZonePreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: ::windows::core::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisual: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisualWithViewportClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: ::windows::core::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: ::windows::core::RawPtr, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisualWithViewportClip: usize,
}
#[doc = "*Required features: `\"UI_Input_Inking_Preview\"`*"]
#[repr(transparent)]
pub struct PalmRejectionDelayZonePreview(::windows::core::IUnknown);
impl PalmRejectionDelayZonePreview {
    #[doc = "*Required features: `\"UI_Input_Inking_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Input_Inking_Preview\"`, `\"Foundation\"`, `\"UI_Composition\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateForVisual<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>>(inputpanelvisual: Param0, inputpanelrect: Param1) -> ::windows::core::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForVisual)(::core::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input_Inking_Preview\"`, `\"Foundation\"`, `\"UI_Composition\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateForVisualWithViewportClip<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>, Param2: ::windows::core::IntoParam<'a, super::super::super::Composition::Visual>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>>(inputpanelvisual: Param0, inputpanelrect: Param1, viewportvisual: Param2, viewportrect: Param3) -> ::windows::core::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForVisualWithViewportClip)(::core::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), viewportvisual.into_param().abi(), viewportrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PalmRejectionDelayZonePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PalmRejectionDelayZonePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PalmRejectionDelayZonePreview {}
impl ::core::fmt::Debug for PalmRejectionDelayZonePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PalmRejectionDelayZonePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview;{62b496cb-539d-5343-a65f-41f5300ec70c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
    const IID: ::windows::core::GUID = <IPalmRejectionDelayZonePreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows::core::IUnknown {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows::core::IUnknown {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows::core::IInspectable {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows::core::IInspectable {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
