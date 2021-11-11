#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreFrameworkInputView(pub ::windows::core::IInspectable);
impl CoreFrameworkInputView {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn OcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows::core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<CoreFrameworkInputView>(result__)
        })
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreFrameworkInputView>(result__)
        })
    }
    pub fn ICoreFrameworkInputViewStatics<R, F: FnOnce(&ICoreFrameworkInputViewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreFrameworkInputView, ICoreFrameworkInputViewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreFrameworkInputView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputView;{d77c94ae-46b8-5d4a-9489-8ddec3d639a6})");
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77c94ae_46b8_5d4a_9489_8ddec3d639a6);
}
impl ::windows::core::RuntimeName for CoreFrameworkInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputView";
}
impl ::core::convert::From<CoreFrameworkInputView> for ::windows::core::IUnknown {
    fn from(value: CoreFrameworkInputView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreFrameworkInputView> for ::windows::core::IUnknown {
    fn from(value: &CoreFrameworkInputView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreFrameworkInputView> for ::windows::core::IInspectable {
    fn from(value: CoreFrameworkInputView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreFrameworkInputView> for ::windows::core::IInspectable {
    fn from(value: &CoreFrameworkInputView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputView {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputView {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(pub ::windows::core::IInspectable);
impl CoreFrameworkInputViewAnimationStartingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation_Collections`*"]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn FrameworkAnimationRecommended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreFrameworkInputViewAnimationStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs;{c0ec901c-bba4-501b-ae8b-65c9e756a719})");
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0ec901c_bba4_501b_ae8b_65c9e756a719);
}
impl ::windows::core::RuntimeName for CoreFrameworkInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs";
}
impl ::core::convert::From<CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewAnimationStartingEventArgs {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(pub ::windows::core::IInspectable);
impl CoreFrameworkInputViewOcclusionsChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation_Collections`*"]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs;{f36f4949-c82c-53d1-a75d-2b2baf0d9b0d})");
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf36f4949_c82c_53d1_a75d_2b2baf0d9b0d);
}
impl ::windows::core::RuntimeName for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs";
}
impl ::core::convert::From<CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInputView(pub ::windows::core::IInspectable);
impl CoreInputView {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn OcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation_Collections`*"]
    pub fn GetCoreInputViewOcclusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TryShowPrimaryView(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TryHidePrimaryView(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreInputView> {
        Self::ICoreInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputView>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn XYFocusTransferringFromPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemoveXYFocusTransferringFromPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn XYFocusTransferredToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemoveXYFocusTransferredToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn TryTransferXYFocusToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, origin: Param0, direction: CoreInputViewXYFocusTransferDirection) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), origin.into_param().abi(), direction, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TryShow(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TryHide(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows::core::Result<CoreInputView> {
        Self::ICoreInputViewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<CoreInputView>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn PrimaryViewShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemovePrimaryViewShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn PrimaryViewHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemovePrimaryViewHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn IsKindSupported(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn SupportedKindsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemoveSupportedKindsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ICoreInputViewStatics<R, F: FnOnce(&ICoreInputViewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInputView, ICoreInputViewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreInputViewStatics2<R, F: FnOnce(&ICoreInputViewStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInputView, ICoreInputViewStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputView;{c770cd7a-7001-4c32-bf94-25c1f554cbf1})");
}
unsafe impl ::windows::core::Interface for CoreInputView {
    type Vtable = ICoreInputView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
}
impl ::windows::core::RuntimeName for CoreInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputView";
}
impl ::core::convert::From<CoreInputView> for ::windows::core::IUnknown {
    fn from(value: CoreInputView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInputView> for ::windows::core::IUnknown {
    fn from(value: &CoreInputView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInputView> for ::windows::core::IInspectable {
    fn from(value: CoreInputView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInputView> for ::windows::core::IInspectable {
    fn from(value: &CoreInputView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInputView {}
unsafe impl ::core::marker::Sync for CoreInputView {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInputViewAnimationStartingEventArgs(pub ::windows::core::IInspectable);
impl CoreInputViewAnimationStartingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation_Collections`*"]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewAnimationStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs;{a9144af2-b55c-5ea1-b8ab-5340f3e94897})");
}
unsafe impl ::windows::core::Interface for CoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
}
impl ::windows::core::RuntimeName for CoreInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs";
}
impl ::core::convert::From<CoreInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewAnimationStartingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewAnimationStartingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewAnimationStartingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewAnimationStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewAnimationStartingEventArgs {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInputViewHidingEventArgs(pub ::windows::core::IInspectable);
impl CoreInputViewHidingEventArgs {
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewHidingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs;{eada47bd-bac5-5336-848d-41083584daad})");
}
unsafe impl ::windows::core::Interface for CoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeada47bd_bac5_5336_848d_41083584daad);
}
impl ::windows::core::RuntimeName for CoreInputViewHidingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs";
}
impl ::core::convert::From<CoreInputViewHidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewHidingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInputViewHidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewHidingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInputViewHidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewHidingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInputViewHidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewHidingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInputViewHidingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewHidingEventArgs {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreInputViewKind(pub i32);
impl CoreInputViewKind {
    pub const Default: CoreInputViewKind = CoreInputViewKind(0i32);
    pub const Keyboard: CoreInputViewKind = CoreInputViewKind(1i32);
    pub const Handwriting: CoreInputViewKind = CoreInputViewKind(2i32);
    pub const Emoji: CoreInputViewKind = CoreInputViewKind(3i32);
    pub const Symbols: CoreInputViewKind = CoreInputViewKind(4i32);
    pub const Clipboard: CoreInputViewKind = CoreInputViewKind(5i32);
    pub const Dictation: CoreInputViewKind = CoreInputViewKind(6i32);
}
impl ::core::convert::From<i32> for CoreInputViewKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewKind;i4)");
}
impl ::windows::core::DefaultType for CoreInputViewKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInputViewOcclusion(pub ::windows::core::IInspectable);
impl CoreInputViewOcclusion {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn OccludingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn OcclusionKind(&self) -> ::windows::core::Result<CoreInputViewOcclusionKind> {
        let this = self;
        unsafe {
            let mut result__: CoreInputViewOcclusionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputViewOcclusionKind>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewOcclusion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewOcclusion;{cc36ce06-3865-4177-b5f5-8b65e0b9ce84})");
}
unsafe impl ::windows::core::Interface for CoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
}
impl ::windows::core::RuntimeName for CoreInputViewOcclusion {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusion";
}
impl ::core::convert::From<CoreInputViewOcclusion> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewOcclusion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInputViewOcclusion> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewOcclusion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInputViewOcclusion> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewOcclusion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInputViewOcclusion> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewOcclusion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusion {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusion {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: CoreInputViewOcclusionKind = CoreInputViewOcclusionKind(0i32);
    pub const Floating: CoreInputViewOcclusionKind = CoreInputViewOcclusionKind(1i32);
    pub const Overlay: CoreInputViewOcclusionKind = CoreInputViewOcclusionKind(2i32);
}
impl ::core::convert::From<i32> for CoreInputViewOcclusionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewOcclusionKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewOcclusionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind;i4)");
}
impl ::windows::core::DefaultType for CoreInputViewOcclusionKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInputViewOcclusionsChangedEventArgs(pub ::windows::core::IInspectable);
impl CoreInputViewOcclusionsChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation_Collections`*"]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs;{be1027e8-b3ee-4df7-9554-89cdc66082c2})");
}
unsafe impl ::windows::core::Interface for CoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1027e8_b3ee_4df7_9554_89cdc66082c2);
}
impl ::windows::core::RuntimeName for CoreInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs";
}
impl ::core::convert::From<CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewOcclusionsChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewOcclusionsChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewOcclusionsChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewOcclusionsChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusionsChangedEventArgs {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInputViewShowingEventArgs(pub ::windows::core::IInspectable);
impl CoreInputViewShowingEventArgs {
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewShowingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs;{ca52261b-fb9e-5daf-a98c-262b8b76af50})");
}
unsafe impl ::windows::core::Interface for CoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca52261b_fb9e_5daf_a98c_262b8b76af50);
}
impl ::windows::core::RuntimeName for CoreInputViewShowingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs";
}
impl ::core::convert::From<CoreInputViewShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewShowingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInputViewShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewShowingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInputViewShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewShowingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInputViewShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewShowingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInputViewShowingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewShowingEventArgs {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInputViewTransferringXYFocusEventArgs(pub ::windows::core::IInspectable);
impl CoreInputViewTransferringXYFocusEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn Origin(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn Direction(&self) -> ::windows::core::Result<CoreInputViewXYFocusTransferDirection> {
        let this = self;
        unsafe {
            let mut result__: CoreInputViewXYFocusTransferDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputViewXYFocusTransferDirection>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetTransferHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn TransferHandled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetKeepPrimaryViewVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn KeepPrimaryViewVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewTransferringXYFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs;{04de169f-ba02-4850-8b55-d82d03ba6d7f})");
}
unsafe impl ::windows::core::Interface for CoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04de169f_ba02_4850_8b55_d82d03ba6d7f);
}
impl ::windows::core::RuntimeName for CoreInputViewTransferringXYFocusEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs";
}
impl ::core::convert::From<CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewTransferringXYFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewTransferringXYFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewTransferringXYFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewTransferringXYFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInputViewTransferringXYFocusEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewTransferringXYFocusEventArgs {}
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreInputViewXYFocusTransferDirection(pub i32);
impl CoreInputViewXYFocusTransferDirection {
    pub const Up: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(0i32);
    pub const Right: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(1i32);
    pub const Down: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(2i32);
    pub const Left: CoreInputViewXYFocusTransferDirection = CoreInputViewXYFocusTransferDirection(3i32);
}
impl ::core::convert::From<i32> for CoreInputViewXYFocusTransferDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewXYFocusTransferDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewXYFocusTransferDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection;i4)");
}
impl ::windows::core::DefaultType for CoreInputViewXYFocusTransferDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreFrameworkInputView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77c94ae_46b8_5d4a_9489_8ddec3d639a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0ec901c_bba4_501b_ae8b_65c9e756a719);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf36f4949_c82c_53d1_a75d_2b2baf0d9b0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewStatics {
    type Vtable = ICoreFrameworkInputViewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eebd9b6_eac2_5f8b_975f_772ee3e42eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputView {
    type Vtable = ICoreInputView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputView2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputView2 {
    type Vtable = ICoreInputView2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ed726c1_e09a_4ae8_aedf_dfa4857d1a01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputView3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputView3 {
    type Vtable = ICoreInputView3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc941653_3ab9_4849_8f58_46e7f0353cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputView4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputView4 {
    type Vtable = ICoreInputView4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x002863d6_d9ef_57eb_8cef_77f6ce1b7ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputView5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputView5 {
    type Vtable = ICoreInputView5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x136316e0_c6d5_5c57_811e_1ad8a99ba6ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewAnimationStartingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewAnimationStartingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewHidingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeada47bd_bac5_5336_848d_41083584daad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewHidingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusion(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusion_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreInputViewOcclusionKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusionsChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1027e8_b3ee_4df7_9554_89cdc66082c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusionsChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewShowingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca52261b_fb9e_5daf_a98c_262b8b76af50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewShowingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewStatics {
    type Vtable = ICoreInputViewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9b97cd_edbe_49cf_a54f_337de052907f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewStatics2 {
    type Vtable = ICoreInputViewStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ebc0862_d049_4e52_87b0_1e90e98c49ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInputViewTransferringXYFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04de169f_ba02_4850_8b55_d82d03ba6d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewTransferringXYFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreInputViewXYFocusTransferDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUISettingsController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUISettingsController {
    type Vtable = IUISettingsController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78a51ac4_15c0_5a1b_a75b_acbf9cb8bb9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUISettingsControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUISettingsControllerStatics {
    type Vtable = IUISettingsControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb3c68cc_c220_578c_8119_7db324ed26a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `UI_ViewManagement_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UISettingsController(pub ::windows::core::IInspectable);
impl UISettingsController {
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetAdvancedEffectsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetAutoHideScrollBars(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetMessageDuration(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_ViewManagement_Core`*"]
    pub fn SetTextScaleFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ViewManagement_Core`, `Foundation`*"]
    pub fn RequestDefaultAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>> {
        Self::IUISettingsControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<UISettingsController>>(result__)
        })
    }
    pub fn IUISettingsControllerStatics<R, F: FnOnce(&IUISettingsControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UISettingsController, IUISettingsControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UISettingsController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.UISettingsController;{78a51ac4-15c0-5a1b-a75b-acbf9cb8bb9e})");
}
unsafe impl ::windows::core::Interface for UISettingsController {
    type Vtable = IUISettingsController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78a51ac4_15c0_5a1b_a75b_acbf9cb8bb9e);
}
impl ::windows::core::RuntimeName for UISettingsController {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.UISettingsController";
}
impl ::core::convert::From<UISettingsController> for ::windows::core::IUnknown {
    fn from(value: UISettingsController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UISettingsController> for ::windows::core::IUnknown {
    fn from(value: &UISettingsController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UISettingsController> for ::windows::core::IInspectable {
    fn from(value: UISettingsController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UISettingsController> for ::windows::core::IInspectable {
    fn from(value: &UISettingsController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UISettingsController {}
unsafe impl ::core::marker::Sync for UISettingsController {}
