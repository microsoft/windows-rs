#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreFrameworkInputView(::windows::core::IUnknown);
impl CoreFrameworkInputView {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows::core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<CoreFrameworkInputView>(result__)
        })
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreFrameworkInputView>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreFrameworkInputViewStatics<R, F: FnOnce(&ICoreFrameworkInputViewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreFrameworkInputView, ICoreFrameworkInputViewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreFrameworkInputView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreFrameworkInputView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreFrameworkInputView {}
impl ::core::fmt::Debug for CoreFrameworkInputView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreFrameworkInputView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreFrameworkInputView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputView;{d77c94ae-46b8-5d4a-9489-8ddec3d639a6})");
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77c94ae_46b8_5d4a_9489_8ddec3d639a6);
}
impl ::windows::core::RuntimeName for CoreFrameworkInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputView";
}
impl ::core::convert::From<CoreFrameworkInputView> for ::windows::core::IUnknown {
    fn from(value: CoreFrameworkInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputView> for ::windows::core::IUnknown {
    fn from(value: &CoreFrameworkInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreFrameworkInputView> for ::windows::core::IInspectable {
    fn from(value: CoreFrameworkInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputView> for ::windows::core::IInspectable {
    fn from(value: &CoreFrameworkInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputView {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputView {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
impl CoreFrameworkInputViewAnimationStartingEventArgs {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn FrameworkAnimationRecommended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreFrameworkInputViewAnimationStartingEventArgs {}
impl ::core::fmt::Debug for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreFrameworkInputViewAnimationStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreFrameworkInputViewAnimationStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs;{c0ec901c-bba4-501b-ae8b-65c9e756a719})");
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0ec901c_bba4_501b_ae8b_65c9e756a719);
}
impl ::windows::core::RuntimeName for CoreFrameworkInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs";
}
impl ::core::convert::From<CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewAnimationStartingEventArgs {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
impl CoreFrameworkInputViewOcclusionsChangedEventArgs {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
impl ::core::fmt::Debug for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreFrameworkInputViewOcclusionsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs;{f36f4949-c82c-53d1-a75d-2b2baf0d9b0d})");
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf36f4949_c82c_53d1_a75d_2b2baf0d9b0d);
}
impl ::windows::core::RuntimeName for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs";
}
impl ::core::convert::From<CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputView(::windows::core::IUnknown);
impl CoreInputView {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCoreInputViewOcclusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TryShowPrimaryView(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TryHidePrimaryView(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn XYFocusTransferringFromPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveXYFocusTransferringFromPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn XYFocusTransferredToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveXYFocusTransferredToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryTransferXYFocusToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, origin: Param0, direction: CoreInputViewXYFocusTransferDirection) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), origin.into_param().abi(), direction, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TryShow(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TryHide(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn IsKindSupported(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportedKindsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSupportedKindsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreInputView> {
        Self::ICoreInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputView>(result__)
        })
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows::core::Result<CoreInputView> {
        Self::ICoreInputViewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<CoreInputView>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreInputViewStatics<R, F: FnOnce(&ICoreInputViewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInputView, ICoreInputViewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreInputViewStatics2<R, F: FnOnce(&ICoreInputViewStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInputView, ICoreInputViewStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreInputView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputView {}
impl ::core::fmt::Debug for CoreInputView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputView;{c770cd7a-7001-4c32-bf94-25c1f554cbf1})");
}
unsafe impl ::windows::core::Interface for CoreInputView {
    type Vtable = ICoreInputViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
}
impl ::windows::core::RuntimeName for CoreInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputView";
}
impl ::core::convert::From<CoreInputView> for ::windows::core::IUnknown {
    fn from(value: CoreInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputView> for ::windows::core::IUnknown {
    fn from(value: &CoreInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputView> for ::windows::core::IInspectable {
    fn from(value: CoreInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputView> for ::windows::core::IInspectable {
    fn from(value: &CoreInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputView {}
unsafe impl ::core::marker::Sync for CoreInputView {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
impl CoreInputViewAnimationStartingEventArgs {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewAnimationStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewAnimationStartingEventArgs {}
impl ::core::fmt::Debug for CoreInputViewAnimationStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewAnimationStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewAnimationStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs;{a9144af2-b55c-5ea1-b8ab-5340f3e94897})");
}
unsafe impl ::windows::core::Interface for CoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
}
impl ::windows::core::RuntimeName for CoreInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs";
}
impl ::core::convert::From<CoreInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewAnimationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewAnimationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewAnimationStartingEventArgs {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewHidingEventArgs(::windows::core::IUnknown);
impl CoreInputViewHidingEventArgs {
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewHidingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewHidingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewHidingEventArgs {}
impl ::core::fmt::Debug for CoreInputViewHidingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewHidingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewHidingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs;{eada47bd-bac5-5336-848d-41083584daad})");
}
unsafe impl ::windows::core::Interface for CoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeada47bd_bac5_5336_848d_41083584daad);
}
impl ::windows::core::RuntimeName for CoreInputViewHidingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs";
}
impl ::core::convert::From<CoreInputViewHidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewHidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewHidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewHidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewHidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewHidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewHidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewHidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewHidingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewHidingEventArgs {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewKind(pub i32);
impl CoreInputViewKind {
    pub const Default: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Handwriting: Self = Self(2i32);
    pub const Emoji: Self = Self(3i32);
    pub const Symbols: Self = Self(4i32);
    pub const Clipboard: Self = Self(5i32);
    pub const Dictation: Self = Self(6i32);
}
impl ::core::marker::Copy for CoreInputViewKind {}
impl ::core::clone::Clone for CoreInputViewKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CoreInputViewKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewKind {}
impl ::core::fmt::Debug for CoreInputViewKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewKind;i4)");
}
impl ::windows::core::DefaultType for CoreInputViewKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewOcclusion(::windows::core::IUnknown);
impl CoreInputViewOcclusion {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OccludingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn OcclusionKind(&self) -> ::windows::core::Result<CoreInputViewOcclusionKind> {
        let this = self;
        unsafe {
            let mut result__: CoreInputViewOcclusionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputViewOcclusionKind>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewOcclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewOcclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewOcclusion {}
impl ::core::fmt::Debug for CoreInputViewOcclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewOcclusion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewOcclusion;{cc36ce06-3865-4177-b5f5-8b65e0b9ce84})");
}
unsafe impl ::windows::core::Interface for CoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
}
impl ::windows::core::RuntimeName for CoreInputViewOcclusion {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusion";
}
impl ::core::convert::From<CoreInputViewOcclusion> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusion> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewOcclusion> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusion> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusion {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusion {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: Self = Self(0i32);
    pub const Floating: Self = Self(1i32);
    pub const Overlay: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreInputViewOcclusionKind {}
impl ::core::clone::Clone for CoreInputViewOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewOcclusionKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CoreInputViewOcclusionKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewOcclusionKind {}
impl ::core::fmt::Debug for CoreInputViewOcclusionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewOcclusionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind;i4)");
}
impl ::windows::core::DefaultType for CoreInputViewOcclusionKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
impl CoreInputViewOcclusionsChangedEventArgs {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for CoreInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewOcclusionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewOcclusionsChangedEventArgs {}
impl ::core::fmt::Debug for CoreInputViewOcclusionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusionsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs;{be1027e8-b3ee-4df7-9554-89cdc66082c2})");
}
unsafe impl ::windows::core::Interface for CoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1027e8_b3ee_4df7_9554_89cdc66082c2);
}
impl ::windows::core::RuntimeName for CoreInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs";
}
impl ::core::convert::From<CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusionsChangedEventArgs {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewShowingEventArgs(::windows::core::IUnknown);
impl CoreInputViewShowingEventArgs {
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewShowingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewShowingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewShowingEventArgs {}
impl ::core::fmt::Debug for CoreInputViewShowingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewShowingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewShowingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs;{ca52261b-fb9e-5daf-a98c-262b8b76af50})");
}
unsafe impl ::windows::core::Interface for CoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca52261b_fb9e_5daf_a98c_262b8b76af50);
}
impl ::windows::core::RuntimeName for CoreInputViewShowingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs";
}
impl ::core::convert::From<CoreInputViewShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewShowingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewShowingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewShowingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewShowingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewShowingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewShowingEventArgs {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewTransferringXYFocusEventArgs(::windows::core::IUnknown);
impl CoreInputViewTransferringXYFocusEventArgs {
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Origin(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn Direction(&self) -> ::windows::core::Result<CoreInputViewXYFocusTransferDirection> {
        let this = self;
        unsafe {
            let mut result__: CoreInputViewXYFocusTransferDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputViewXYFocusTransferDirection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetTransferHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn TransferHandled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetKeepPrimaryViewVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn KeepPrimaryViewVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewTransferringXYFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewTransferringXYFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewTransferringXYFocusEventArgs {}
impl ::core::fmt::Debug for CoreInputViewTransferringXYFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewTransferringXYFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewTransferringXYFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs;{04de169f-ba02-4850-8b55-d82d03ba6d7f})");
}
unsafe impl ::windows::core::Interface for CoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04de169f_ba02_4850_8b55_d82d03ba6d7f);
}
impl ::windows::core::RuntimeName for CoreInputViewTransferringXYFocusEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs";
}
impl ::core::convert::From<CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreInputViewTransferringXYFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreInputViewTransferringXYFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreInputViewTransferringXYFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewTransferringXYFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreInputViewTransferringXYFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewTransferringXYFocusEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewTransferringXYFocusEventArgs {}
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct CoreInputViewXYFocusTransferDirection(pub i32);
impl CoreInputViewXYFocusTransferDirection {
    pub const Up: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Down: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreInputViewXYFocusTransferDirection {}
impl ::core::clone::Clone for CoreInputViewXYFocusTransferDirection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewXYFocusTransferDirection {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CoreInputViewXYFocusTransferDirection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewXYFocusTransferDirection {}
impl ::core::fmt::Debug for CoreInputViewXYFocusTransferDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewXYFocusTransferDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewXYFocusTransferDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection;i4)");
}
impl ::windows::core::DefaultType for CoreInputViewXYFocusTransferDirection {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77c94ae_46b8_5d4a_9489_8ddec3d639a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0ec901c_bba4_501b_ae8b_65c9e756a719);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf36f4949_c82c_53d1_a75d_2b2baf0d9b0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewStatics {
    type Vtable = ICoreFrameworkInputViewStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eebd9b6_eac2_5f8b_975f_772ee3e42eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView {
    type Vtable = ICoreInputViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView2 {
    type Vtable = ICoreInputView2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ed726c1_e09a_4ae8_aedf_dfa4857d1a01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView3 {
    type Vtable = ICoreInputView3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc941653_3ab9_4849_8f58_46e7f0353cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView4 {
    type Vtable = ICoreInputView4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x002863d6_d9ef_57eb_8cef_77f6ce1b7ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView5 {
    type Vtable = ICoreInputView5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x136316e0_c6d5_5c57_811e_1ad8a99ba6ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewAnimationStartingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewHidingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeada47bd_bac5_5336_848d_41083584daad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewHidingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewOcclusion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewOcclusionKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1027e8_b3ee_4df7_9554_89cdc66082c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusionsChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewShowingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca52261b_fb9e_5daf_a98c_262b8b76af50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewShowingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewStatics {
    type Vtable = ICoreInputViewStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9b97cd_edbe_49cf_a54f_337de052907f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewStatics2 {
    type Vtable = ICoreInputViewStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ebc0862_d049_4e52_87b0_1e90e98c49ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewTransferringXYFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04de169f_ba02_4850_8b55_d82d03ba6d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewTransferringXYFocusEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewXYFocusTransferDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettingsController {
    type Vtable = IUISettingsControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78a51ac4_15c0_5a1b_a75b_acbf9cb8bb9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsControllerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettingsControllerStatics {
    type Vtable = IUISettingsControllerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb3c68cc_c220_578c_8119_7db324ed26a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsControllerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'UI_ViewManagement_Core'*"]
#[repr(transparent)]
pub struct UISettingsController(::windows::core::IUnknown);
impl UISettingsController {
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetAdvancedEffectsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetAutoHideScrollBars(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetMessageDuration(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core'*"]
    pub fn SetTextScaleFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_ViewManagement_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDefaultAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>> {
        Self::IUISettingsControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<UISettingsController>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUISettingsControllerStatics<R, F: FnOnce(&IUISettingsControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UISettingsController, IUISettingsControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UISettingsController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UISettingsController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsController {}
impl ::core::fmt::Debug for UISettingsController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UISettingsController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.UISettingsController;{78a51ac4-15c0-5a1b-a75b-acbf9cb8bb9e})");
}
unsafe impl ::windows::core::Interface for UISettingsController {
    type Vtable = IUISettingsControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78a51ac4_15c0_5a1b_a75b_acbf9cb8bb9e);
}
impl ::windows::core::RuntimeName for UISettingsController {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.UISettingsController";
}
impl ::core::convert::From<UISettingsController> for ::windows::core::IUnknown {
    fn from(value: UISettingsController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsController> for ::windows::core::IUnknown {
    fn from(value: &UISettingsController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UISettingsController> for ::windows::core::IInspectable {
    fn from(value: UISettingsController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsController> for ::windows::core::IInspectable {
    fn from(value: &UISettingsController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UISettingsController {}
unsafe impl ::core::marker::Sync for UISettingsController {}
