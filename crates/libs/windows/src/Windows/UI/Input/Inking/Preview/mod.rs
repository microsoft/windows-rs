#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
}
impl ::core::clone::Clone for IPalmRejectionDelayZonePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPalmRejectionDelayZonePreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreviewStatics {
    type Vtable = IPalmRejectionDelayZonePreviewStatics_Vtbl;
}
impl ::core::clone::Clone for IPalmRejectionDelayZonePreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPalmRejectionDelayZonePreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: *mut ::core::ffi::c_void, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisual: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisualWithViewportClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: *mut ::core::ffi::c_void, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: *mut ::core::ffi::c_void, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisualWithViewportClip: usize,
}
#[doc = "*Required features: `\"UI_Input_Inking_Preview\"`*"]
#[repr(transparent)]
pub struct PalmRejectionDelayZonePreview(::windows::core::IUnknown);
impl PalmRejectionDelayZonePreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Composition\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateForVisual<P0>(inputpanelvisual: P0, inputpanelrect: super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Composition::Visual>,
    {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PalmRejectionDelayZonePreview>();
            (::windows::core::Interface::vtable(this).CreateForVisual)(::windows::core::Interface::as_raw(this), inputpanelvisual.try_into_param()?.abi(), inputpanelrect, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Composition\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateForVisualWithViewportClip<P0, P1>(inputpanelvisual: P0, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: P1, viewportrect: super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Composition::Visual>,
        P1: ::windows::core::TryIntoParam<super::super::super::Composition::Visual>,
    {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PalmRejectionDelayZonePreview>();
            (::windows::core::Interface::vtable(this).CreateForVisualWithViewportClip)(::windows::core::Interface::as_raw(this), inputpanelvisual.try_into_param()?.abi(), inputpanelrect, viewportvisual.try_into_param()?.abi(), viewportrect, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview;{62b496cb-539d-5343-a65f-41f5300ec70c})");
}
impl ::core::clone::Clone for PalmRejectionDelayZonePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PalmRejectionDelayZonePreview {
    const IID: ::windows::core::GUID = <IPalmRejectionDelayZonePreview as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
::windows::imp::interface_hierarchy!(PalmRejectionDelayZonePreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::super::Foundation::IClosable> for PalmRejectionDelayZonePreview {}
unsafe impl ::core::marker::Send for PalmRejectionDelayZonePreview {}
unsafe impl ::core::marker::Sync for PalmRejectionDelayZonePreview {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
