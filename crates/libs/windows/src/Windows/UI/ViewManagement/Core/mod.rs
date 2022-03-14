#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreFrameworkInputView(::windows::core::IUnknown);
impl CoreFrameworkInputView {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryViewAnimationStarting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OcclusionsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOcclusionsChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows::core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUIContext)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<CoreFrameworkInputView>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreFrameworkInputView>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputView_Vtbl;
    const IID: ::windows::core::GUID = <ICoreFrameworkInputView as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreFrameworkInputView {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreFrameworkInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputView {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputView {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
impl CoreFrameworkInputViewAnimationStartingEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Occlusions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn FrameworkAnimationRecommended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameworkAnimationRecommended)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AnimationDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreFrameworkInputViewAnimationStartingEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreFrameworkInputViewAnimationStartingEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewAnimationStartingEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
impl CoreFrameworkInputViewOcclusionsChangedEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Occlusions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreFrameworkInputViewOcclusionsChangedEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreFrameworkInputViewOcclusionsChangedEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputView(::windows::core::IUnknown);
impl CoreInputView {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OcclusionsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOcclusionsChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCoreInputViewOcclusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCoreInputViewOcclusions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TryShowPrimaryView(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryShowPrimaryView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TryHidePrimaryView(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryHidePrimaryView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn XYFocusTransferringFromPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusTransferringFromPrimaryView)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveXYFocusTransferringFromPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveXYFocusTransferringFromPrimaryView)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn XYFocusTransferredToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusTransferredToPrimaryView)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveXYFocusTransferredToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveXYFocusTransferredToPrimaryView)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryTransferXYFocusToPrimaryView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, origin: Param0, direction: CoreInputViewXYFocusTransferDirection) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryTransferXYFocusToPrimaryView)(::core::mem::transmute_copy(this), origin.into_param().abi(), direction, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TryShow(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryShow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryShowWithKind)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TryHide(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryHide)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryViewShowing)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrimaryViewShowing)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryViewHiding)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrimaryViewHiding)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn IsKindSupported(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsKindSupported)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportedKindsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedKindsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSupportedKindsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSupportedKindsChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryViewAnimationStarting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreInputView> {
        Self::ICoreInputViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputView>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows::core::Result<CoreInputView> {
        Self::ICoreInputViewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUIContext)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<CoreInputView>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreInputView {
    type Vtable = ICoreInputView_Vtbl;
    const IID: ::windows::core::GUID = <ICoreInputView as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputView {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputView {}
unsafe impl ::core::marker::Sync for CoreInputView {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
impl CoreInputViewAnimationStartingEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Occlusions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AnimationDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreInputViewAnimationStartingEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewAnimationStartingEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewAnimationStartingEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewHidingEventArgs(::windows::core::IUnknown);
impl CoreInputViewHidingEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreInputViewHidingEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewHidingEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewHidingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewHidingEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for CoreInputViewKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreInputViewKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewOcclusion(::windows::core::IUnknown);
impl CoreInputViewOcclusion {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OccludingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OccludingRect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn OcclusionKind(&self) -> ::windows::core::Result<CoreInputViewOcclusionKind> {
        let this = self;
        unsafe {
            let mut result__: CoreInputViewOcclusionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OcclusionKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputViewOcclusionKind>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusion_Vtbl;
    const IID: ::windows::core::GUID = <ICoreInputViewOcclusion as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewOcclusion {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusion {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusion {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for CoreInputViewOcclusionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewOcclusionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreInputViewOcclusionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewOcclusionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
impl CoreInputViewOcclusionsChangedEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Occlusions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreInputViewOcclusionsChangedEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewOcclusionsChangedEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusionsChangedEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewShowingEventArgs(::windows::core::IUnknown);
impl CoreInputViewShowingEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreInputViewShowingEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewShowingEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewShowingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewShowingEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewTransferringXYFocusEventArgs(::windows::core::IUnknown);
impl CoreInputViewTransferringXYFocusEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Origin(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Origin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn Direction(&self) -> ::windows::core::Result<CoreInputViewXYFocusTransferDirection> {
        let this = self;
        unsafe {
            let mut result__: CoreInputViewXYFocusTransferDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreInputViewXYFocusTransferDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetTransferHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTransferHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn TransferHandled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransferHandled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetKeepPrimaryViewVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeepPrimaryViewVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn KeepPrimaryViewVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeepPrimaryViewVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreInputViewTransferringXYFocusEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInputViewTransferringXYFocusEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewTransferringXYFocusEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewTransferringXYFocusEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for CoreInputViewXYFocusTransferDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreInputViewXYFocusTransferDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreInputViewXYFocusTransferDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewXYFocusTransferDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInputViewXYFocusTransferDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77c94ae_46b8_5d4a_9489_8ddec3d639a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub OcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OcclusionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOcclusionsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0ec901c_bba4_501b_ae8b_65c9e756a719);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub FrameworkAnimationRecommended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnimationDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf36f4949_c82c_53d1_a75d_2b2baf0d9b0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewStatics {
    type Vtable = ICoreFrameworkInputViewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eebd9b6_eac2_5f8b_975f_772ee3e42eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView {
    type Vtable = ICoreInputView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub OcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OcclusionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOcclusionsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCoreInputViewOcclusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCoreInputViewOcclusions: usize,
    pub TryShowPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryHidePrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView2 {
    type Vtable = ICoreInputView2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ed726c1_e09a_4ae8_aedf_dfa4857d1a01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub XYFocusTransferringFromPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XYFocusTransferringFromPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXYFocusTransferringFromPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXYFocusTransferringFromPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub XYFocusTransferredToPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XYFocusTransferredToPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXYFocusTransferredToPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXYFocusTransferredToPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub TryTransferXYFocusToPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTransferXYFocusToPrimaryView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView3 {
    type Vtable = ICoreInputView3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc941653_3ab9_4849_8f58_46e7f0353cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryShowWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryHide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView4 {
    type Vtable = ICoreInputView4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x002863d6_d9ef_57eb_8cef_77f6ce1b7ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewShowing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewShowing: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewHiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewHiding: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewHiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewHiding: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputView5 {
    type Vtable = ICoreInputView5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x136316e0_c6d5_5c57_811e_1ad8a99ba6ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsKindSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SupportedKindsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportedKindsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSupportedKindsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSupportedKindsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewAnimationStarting: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewAnimationStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewAnimationStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnimationDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewHidingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeada47bd_bac5_5336_848d_41083584daad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewHidingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewOcclusion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusion_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub OccludingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludingRect: usize,
    pub OcclusionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewOcclusionKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewOcclusionsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1027e8_b3ee_4df7_9554_89cdc66082c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewShowingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca52261b_fb9e_5daf_a98c_262b8b76af50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewShowingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewStatics {
    type Vtable = ICoreInputViewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9b97cd_edbe_49cf_a54f_337de052907f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewStatics2 {
    type Vtable = ICoreInputViewStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ebc0862_d049_4e52_87b0_1e90e98c49ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewTransferringXYFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04de169f_ba02_4850_8b55_d82d03ba6d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewTransferringXYFocusEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Origin: usize,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewXYFocusTransferDirection) -> ::windows::core::HRESULT,
    pub SetTransferHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TransferHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetKeepPrimaryViewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub KeepPrimaryViewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettingsController {
    type Vtable = IUISettingsController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78a51ac4_15c0_5a1b_a75b_acbf9cb8bb9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetAdvancedEffectsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetAnimationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetAutoHideScrollBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetMessageDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub SetTextScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettingsControllerStatics {
    type Vtable = IUISettingsControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb3c68cc_c220_578c_8119_7db324ed26a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDefaultAsync: usize,
}
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct UISettingsController(::windows::core::IUnknown);
impl UISettingsController {
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetAdvancedEffectsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAdvancedEffectsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAnimationsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetAutoHideScrollBars(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoHideScrollBars)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetMessageDuration(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMessageDuration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
    pub fn SetTextScaleFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextScaleFactor)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDefaultAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>> {
        Self::IUISettingsControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<UISettingsController>>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UISettingsController {
    type Vtable = IUISettingsController_Vtbl;
    const IID: ::windows::core::GUID = <IUISettingsController as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UISettingsController {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UISettingsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UISettingsController {}
unsafe impl ::core::marker::Sync for UISettingsController {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
