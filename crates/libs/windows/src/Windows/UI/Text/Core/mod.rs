#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextCompositionCompletedEventArgs(::windows::core::IUnknown);
impl CoreTextCompositionCompletedEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CompositionSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompositionSegments)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextCompositionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionCompletedEventArgs {}
impl ::core::fmt::Debug for CoreTextCompositionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs;{1f34ebb6-b79f-4121-a5e7-fda9b8616e30})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextCompositionCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextCompositionCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs";
}
impl ::core::convert::From<CoreTextCompositionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextCompositionCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextCompositionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextCompositionCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextCompositionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextCompositionCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextCompositionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextCompositionCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextCompositionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextCompositionCompletedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextCompositionSegment(::windows::core::IUnknown);
impl CoreTextCompositionSegment {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn PreconversionString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreconversionString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Range)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextCompositionSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionSegment {}
impl ::core::fmt::Debug for CoreTextCompositionSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionSegment;{776c6bd9-4ead-4da7-8f47-3a88b523cc34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextCompositionSegment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionSegment";
}
impl ::core::convert::From<CoreTextCompositionSegment> for ::windows::core::IUnknown {
    fn from(value: CoreTextCompositionSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextCompositionSegment> for ::windows::core::IUnknown {
    fn from(value: &CoreTextCompositionSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextCompositionSegment> for ::windows::core::IInspectable {
    fn from(value: CoreTextCompositionSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextCompositionSegment> for ::windows::core::IInspectable {
    fn from(value: &CoreTextCompositionSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextCompositionSegment {}
unsafe impl ::core::marker::Sync for CoreTextCompositionSegment {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextCompositionStartedEventArgs(::windows::core::IUnknown);
impl CoreTextCompositionStartedEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextCompositionStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionStartedEventArgs {}
impl ::core::fmt::Debug for CoreTextCompositionStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs;{276b16a9-64e7-4ab0-bc4b-a02d73835bfb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextCompositionStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextCompositionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs";
}
impl ::core::convert::From<CoreTextCompositionStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextCompositionStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextCompositionStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextCompositionStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextCompositionStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextCompositionStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextCompositionStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextCompositionStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextCompositionStartedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextCompositionStartedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextEditContext(::windows::core::IUnknown);
impl CoreTextEditContext {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn InputScope(&self) -> ::windows::core::Result<CoreTextInputScope> {
        let this = self;
        unsafe {
            let mut result__: CoreTextInputScope = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputScope)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextInputScope>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetInputScope(&self, value: CoreTextInputScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInputScope)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsReadOnly)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn InputPaneDisplayPolicy(&self) -> ::windows::core::Result<CoreTextInputPaneDisplayPolicy> {
        let this = self;
        unsafe {
            let mut result__: CoreTextInputPaneDisplayPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputPaneDisplayPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextInputPaneDisplayPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetInputPaneDisplayPolicy(&self, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInputPaneDisplayPolicy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTextRequested)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectionRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSelectionRequested)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LayoutRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LayoutRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLayoutRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLayoutRequested)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextUpdating)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTextUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTextUpdating)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectionUpdating)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSelectionUpdating)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FormatUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormatUpdating)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFormatUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFormatUpdating)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompositionStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompositionStarted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompositionStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompositionStarted)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompositionCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompositionCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompositionCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompositionCompleted)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FocusRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusRemoved)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFocusRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFocusRemoved)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn NotifyFocusEnter(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyFocusEnter)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn NotifyFocusLeave(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyFocusLeave)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn NotifyTextChanged<'a, Param0: ::windows::core::IntoParam<'a, CoreTextRange>, Param2: ::windows::core::IntoParam<'a, CoreTextRange>>(&self, modifiedrange: Param0, newlength: i32, newselection: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyTextChanged)(::core::mem::transmute_copy(this), modifiedrange.into_param().abi(), newlength, newselection.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn NotifySelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, CoreTextRange>>(&self, selection: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifySelectionChanged)(::core::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn NotifyLayoutChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyLayoutChanged)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyFocusLeaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotifyFocusLeaveCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotifyFocusLeaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNotifyFocusLeaveCompleted)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreTextEditContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextEditContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextEditContext {}
impl ::core::fmt::Debug for CoreTextEditContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextEditContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextEditContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextEditContext;{bf6608af-4041-47c3-b263-a918eb5eaef2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextEditContext {
    type Vtable = ICoreTextEditContext_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextEditContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextEditContext {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextEditContext";
}
impl ::core::convert::From<CoreTextEditContext> for ::windows::core::IUnknown {
    fn from(value: CoreTextEditContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextEditContext> for ::windows::core::IUnknown {
    fn from(value: &CoreTextEditContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextEditContext> for ::windows::core::IInspectable {
    fn from(value: CoreTextEditContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextEditContext> for ::windows::core::IInspectable {
    fn from(value: &CoreTextEditContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextEditContext {}
unsafe impl ::core::marker::Sync for CoreTextEditContext {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingEventArgs(::windows::core::IUnknown);
impl CoreTextFormatUpdatingEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Range)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn TextColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn UnderlineColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlineColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnderlineType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::UnderlineType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlineType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::UnderlineType>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<CoreTextFormatUpdatingReason> {
        let this = self;
        unsafe {
            let mut result__: CoreTextFormatUpdatingReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextFormatUpdatingReason>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Result(&self) -> ::windows::core::Result<CoreTextFormatUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextFormatUpdatingResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextFormatUpdatingResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetResult(&self, value: CoreTextFormatUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResult)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextFormatUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextFormatUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextFormatUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextFormatUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs;{7310bd33-b4a8-43b1-b37b-0724d4aca7ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextFormatUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextFormatUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs";
}
impl ::core::convert::From<CoreTextFormatUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextFormatUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextFormatUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextFormatUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextFormatUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextFormatUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextFormatUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextFormatUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextFormatUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextFormatUpdatingEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreTextFormatUpdatingReason(pub i32);
impl CoreTextFormatUpdatingReason {
    pub const None: Self = Self(0i32);
    pub const CompositionUnconverted: Self = Self(1i32);
    pub const CompositionConverted: Self = Self(2i32);
    pub const CompositionTargetUnconverted: Self = Self(3i32);
    pub const CompositionTargetConverted: Self = Self(4i32);
}
impl ::core::marker::Copy for CoreTextFormatUpdatingReason {}
impl ::core::clone::Clone for CoreTextFormatUpdatingReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextFormatUpdatingReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextFormatUpdatingReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextFormatUpdatingReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextFormatUpdatingResult {}
impl ::core::clone::Clone for CoreTextFormatUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextFormatUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextFormatUpdatingResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextFormatUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextInputPaneDisplayPolicy {}
impl ::core::clone::Clone for CoreTextInputPaneDisplayPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextInputPaneDisplayPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextInputPaneDisplayPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextInputPaneDisplayPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextInputPaneDisplayPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextInputPaneDisplayPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputPaneDisplayPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
    pub const FileName: Self = Self(3i32);
    pub const EmailUserName: Self = Self(4i32);
    pub const EmailAddress: Self = Self(5i32);
    pub const UserName: Self = Self(6i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const PersonalNamePrefix: Self = Self(8i32);
    pub const PersonalGivenName: Self = Self(9i32);
    pub const PersonalMiddleName: Self = Self(10i32);
    pub const PersonalSurname: Self = Self(11i32);
    pub const PersonalNameSuffix: Self = Self(12i32);
    pub const Address: Self = Self(13i32);
    pub const AddressPostalCode: Self = Self(14i32);
    pub const AddressStreet: Self = Self(15i32);
    pub const AddressStateOrProvince: Self = Self(16i32);
    pub const AddressCity: Self = Self(17i32);
    pub const AddressCountryName: Self = Self(18i32);
    pub const AddressCountryShortName: Self = Self(19i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const Date: Self = Self(22i32);
    pub const DateMonth: Self = Self(23i32);
    pub const DateDay: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const DateMonthName: Self = Self(26i32);
    pub const DateDayName: Self = Self(27i32);
    pub const Number: Self = Self(29i32);
    pub const SingleCharacter: Self = Self(30i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const Time: Self = Self(36i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinuteOrSecond: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const CurrencyChinese: Self = Self(42i32);
    pub const Bopomofo: Self = Self(43i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailUserNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const PasswordNumeric: Self = Self(63i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
    pub const Digits: Self = Self(28i32);
    pub const PinNumeric: Self = Self(64i32);
    pub const PinAlphanumeric: Self = Self(65i32);
}
impl ::core::marker::Copy for CoreTextInputScope {}
impl ::core::clone::Clone for CoreTextInputScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextInputScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextInputScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextInputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextInputScope").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextInputScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextLayoutBounds(::windows::core::IUnknown);
impl CoreTextLayoutBounds {
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextBounds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTextBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextBounds)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ControlBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlBounds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetControlBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetControlBounds)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreTextLayoutBounds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutBounds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutBounds {}
impl ::core::fmt::Debug for CoreTextLayoutBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutBounds").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutBounds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutBounds;{e972c974-4436-4917-80d0-a525e4ca6780})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextLayoutBounds as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextLayoutBounds {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutBounds";
}
impl ::core::convert::From<CoreTextLayoutBounds> for ::windows::core::IUnknown {
    fn from(value: CoreTextLayoutBounds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextLayoutBounds> for ::windows::core::IUnknown {
    fn from(value: &CoreTextLayoutBounds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextLayoutBounds> for ::windows::core::IInspectable {
    fn from(value: CoreTextLayoutBounds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextLayoutBounds> for ::windows::core::IInspectable {
    fn from(value: &CoreTextLayoutBounds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextLayoutBounds {}
unsafe impl ::core::marker::Sync for CoreTextLayoutBounds {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextLayoutRequest(::windows::core::IUnknown);
impl CoreTextLayoutRequest {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Range)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn LayoutBounds(&self) -> ::windows::core::Result<CoreTextLayoutBounds> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LayoutBounds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutBounds>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn LayoutBoundsVisualPixels(&self) -> ::windows::core::Result<CoreTextLayoutBounds> {
        let this = &::windows::core::Interface::cast::<ICoreTextLayoutRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LayoutBoundsVisualPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutBounds>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextLayoutRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutRequest {}
impl ::core::fmt::Debug for CoreTextLayoutRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequest;{2555a8cc-51fd-4f03-98bf-ac78174d68e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextLayoutRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextLayoutRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequest";
}
impl ::core::convert::From<CoreTextLayoutRequest> for ::windows::core::IUnknown {
    fn from(value: CoreTextLayoutRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextLayoutRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreTextLayoutRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextLayoutRequest> for ::windows::core::IInspectable {
    fn from(value: CoreTextLayoutRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextLayoutRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreTextLayoutRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextLayoutRequest {}
unsafe impl ::core::marker::Sync for CoreTextLayoutRequest {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextLayoutRequestedEventArgs(::windows::core::IUnknown);
impl CoreTextLayoutRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreTextLayoutRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextLayoutRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextLayoutRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs;{b1dc6ae0-9a7b-4e9e-a566-4a6b5f8ad676})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextLayoutRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextLayoutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs";
}
impl ::core::convert::From<CoreTextLayoutRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextLayoutRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextLayoutRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextLayoutRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextLayoutRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextLayoutRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextLayoutRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextLayoutRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextLayoutRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextLayoutRequestedEventArgs {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
pub struct CoreTextRange {
    pub StartCaretPosition: i32,
    pub EndCaretPosition: i32,
}
impl ::core::marker::Copy for CoreTextRange {}
impl ::core::clone::Clone for CoreTextRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CoreTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreTextRange").field("StartCaretPosition", &self.StartCaretPosition).field("EndCaretPosition", &self.EndCaretPosition).finish()
    }
}
unsafe impl ::windows::core::Abi for CoreTextRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Text.Core.CoreTextRange;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CoreTextRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CoreTextRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for CoreTextRange {}
impl ::core::default::Default for CoreTextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextSelectionRequest(::windows::core::IUnknown);
impl CoreTextSelectionRequest {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Selection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Selection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetSelection<'a, Param0: ::windows::core::IntoParam<'a, CoreTextRange>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSelection)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextSelectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionRequest {}
impl ::core::fmt::Debug for CoreTextSelectionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequest;{f0a70403-208b-4301-883c-74ca7485fd8d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextSelectionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextSelectionRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequest";
}
impl ::core::convert::From<CoreTextSelectionRequest> for ::windows::core::IUnknown {
    fn from(value: CoreTextSelectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextSelectionRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreTextSelectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextSelectionRequest> for ::windows::core::IInspectable {
    fn from(value: CoreTextSelectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextSelectionRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreTextSelectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextSelectionRequest {}
unsafe impl ::core::marker::Sync for CoreTextSelectionRequest {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextSelectionRequestedEventArgs(::windows::core::IUnknown);
impl CoreTextSelectionRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreTextSelectionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextSelectionRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextSelectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextSelectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs;{13c6682b-f614-421a-8f4b-9ec8a5a37fcd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextSelectionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextSelectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs";
}
impl ::core::convert::From<CoreTextSelectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextSelectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextSelectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextSelectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextSelectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextSelectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextSelectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextSelectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextSelectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextSelectionRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingEventArgs(::windows::core::IUnknown);
impl CoreTextSelectionUpdatingEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Selection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Selection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Result(&self) -> ::windows::core::Result<CoreTextSelectionUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextSelectionUpdatingResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextSelectionUpdatingResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetResult(&self, value: CoreTextSelectionUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResult)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextSelectionUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextSelectionUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs;{d445839f-fe7f-4bd5-8a26-0922c1b3e639})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextSelectionUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextSelectionUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs";
}
impl ::core::convert::From<CoreTextSelectionUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextSelectionUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextSelectionUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextSelectionUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextSelectionUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextSelectionUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextSelectionUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextSelectionUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextSelectionUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextSelectionUpdatingEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreTextSelectionUpdatingResult(pub i32);
impl CoreTextSelectionUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextSelectionUpdatingResult {}
impl ::core::clone::Clone for CoreTextSelectionUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextSelectionUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextSelectionUpdatingResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextSelectionUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionUpdatingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextSelectionUpdatingResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
pub struct CoreTextServicesConstants {}
impl CoreTextServicesConstants {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn HiddenCharacter() -> ::windows::core::Result<u16> {
        Self::ICoreTextServicesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HiddenCharacter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreTextServicesStatics<R, F: FnOnce(&ICoreTextServicesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreTextServicesConstants, ICoreTextServicesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CoreTextServicesConstants {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesConstants";
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextServicesManager(::windows::core::IUnknown);
impl CoreTextServicesManager {
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputLanguage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputLanguageChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputLanguageChanged)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn CreateEditContext(&self) -> ::windows::core::Result<CoreTextEditContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateEditContext)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextEditContext>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreTextServicesManager> {
        Self::ICoreTextServicesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextServicesManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreTextServicesManagerStatics<R, F: FnOnce(&ICoreTextServicesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreTextServicesManager, ICoreTextServicesManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreTextServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextServicesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextServicesManager {}
impl ::core::fmt::Debug for CoreTextServicesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextServicesManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextServicesManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextServicesManager;{c2507d83-6e0a-4a8a-bdf8-1948874854ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextServicesManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextServicesManager {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesManager";
}
impl ::core::convert::From<CoreTextServicesManager> for ::windows::core::IUnknown {
    fn from(value: CoreTextServicesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextServicesManager> for ::windows::core::IUnknown {
    fn from(value: &CoreTextServicesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextServicesManager> for ::windows::core::IInspectable {
    fn from(value: CoreTextServicesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextServicesManager> for ::windows::core::IInspectable {
    fn from(value: &CoreTextServicesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextServicesManager {}
unsafe impl ::core::marker::Sync for CoreTextServicesManager {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextTextRequest(::windows::core::IUnknown);
impl CoreTextTextRequest {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Range)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextTextRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextTextRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextRequest {}
impl ::core::fmt::Debug for CoreTextTextRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequest;{50d950a9-f51e-4cc1-8ca1-e6346d1a61be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextTextRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextTextRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequest";
}
impl ::core::convert::From<CoreTextTextRequest> for ::windows::core::IUnknown {
    fn from(value: CoreTextTextRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextTextRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreTextTextRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextTextRequest> for ::windows::core::IInspectable {
    fn from(value: CoreTextTextRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextTextRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreTextTextRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextTextRequest {}
unsafe impl ::core::marker::Sync for CoreTextTextRequest {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextTextRequestedEventArgs(::windows::core::IUnknown);
impl CoreTextTextRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreTextTextRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextTextRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextTextRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextTextRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextTextRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequestedEventArgs;{f096a2d0-41c6-4c02-8b1a-d953b00cabb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextTextRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextTextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequestedEventArgs";
}
impl ::core::convert::From<CoreTextTextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextTextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextTextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextTextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextTextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextTextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextTextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextTextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextTextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextTextRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextTextUpdatingEventArgs(::windows::core::IUnknown);
impl CoreTextTextUpdatingEventArgs {
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Range)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn NewSelection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewSelection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputLanguage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn Result(&self) -> ::windows::core::Result<CoreTextTextUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextTextUpdatingResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextTextUpdatingResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn SetResult(&self, value: CoreTextTextUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResult)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextTextUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextTextUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextTextUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs;{eea7918d-cc2b-4f03-8ff6-02fd217db450})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICoreTextTextUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextTextUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs";
}
impl ::core::convert::From<CoreTextTextUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextTextUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextTextUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextTextUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreTextTextUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextTextUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreTextTextUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextTextUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreTextTextUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextTextUpdatingEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreTextTextUpdatingResult(pub i32);
impl CoreTextTextUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextTextUpdatingResult {}
impl ::core::clone::Clone for CoreTextTextUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextTextUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextTextUpdatingResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextTextUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextUpdatingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextTextUpdatingResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextCompositionCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f34ebb6_b79f_4121_a5e7_fda9b8616e30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CompositionSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CompositionSegments: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextCompositionSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776c6bd9_4ead_4da7_8f47_3a88b523cc34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionSegment_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PreconversionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextCompositionStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276b16a9_64e7_4ab0_bc4b_a02d73835bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionStartedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextEditContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextEditContext {
    type Vtable = ICoreTextEditContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6608af_4041_47c3_b263_a918eb5eaef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InputScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputScope) -> ::windows::core::HRESULT,
    pub SetInputScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextInputScope) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InputPaneDisplayPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT,
    pub SetInputPaneDisplayPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub LayoutRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLayoutRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLayoutRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TextUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub FormatUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FormatUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFormatUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFormatUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub CompositionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompositionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub CompositionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompositionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompositionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompositionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub FocusRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusRemoved: usize,
    pub NotifyFocusEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyFocusLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyTextChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows::core::HRESULT,
    pub NotifySelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: CoreTextRange) -> ::windows::core::HRESULT,
    pub NotifyLayoutChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextEditContext2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextEditContext2 {
    type Vtable = ICoreTextEditContext2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1867dbb_083b_49e1_b281_2b35d62bf466);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub NotifyFocusLeaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyFocusLeaveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotifyFocusLeaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotifyFocusLeaveCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextFormatUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7310bd33_b4a8_43b1_b37b_0724d4aca7ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextFormatUpdatingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub TextColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    TextColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    BackgroundColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub UnderlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    UnderlineColor: usize,
    #[cfg(feature = "Foundation")]
    pub UnderlineType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnderlineType: usize,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingReason) -> ::windows::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutBounds(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe972c974_4436_4917_80d0_a525e4ca6780);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutBounds_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TextBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetTextBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTextBounds: usize,
    #[cfg(feature = "Foundation")]
    pub ControlBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetControlBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetControlBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2555a8cc_51fd_4f03_98bf_ac78174d68e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub LayoutBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequest2 {
    type Vtable = ICoreTextLayoutRequest2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x676de624_cd3d_4bcd_bf01_7f7110954511);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LayoutBoundsVisualPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1dc6ae0_9a7b_4e9e_a566_4a6b5f8ad676);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextSelectionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a70403_208b_4301_883c_74ca7485fd8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextRange) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextSelectionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13c6682b_f614_421a_8f4b_9ec8a5a37fcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextSelectionUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd445839f_fe7f_4bd5_8a26_0922c1b3e639);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionUpdatingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextServicesManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2507d83_6e0a_4a8a_bdf8_1948874854ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    #[cfg(feature = "Foundation")]
    pub InputLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputLanguageChanged: usize,
    pub CreateEditContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextServicesManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextServicesManagerStatics {
    type Vtable = ICoreTextServicesManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1520a388_e2cf_4d65_aeb9_b32d86fe39b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextServicesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextServicesStatics {
    type Vtable = ICoreTextServicesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91859a46_eccf_47a4_8ae7_098a9c6fbb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HiddenCharacter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextTextRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d950a9_f51e_4cc1_8ca1_e6346d1a61be);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextTextRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf096a2d0_41c6_4c02_8b1a_d953b00cabb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextTextUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea7918d_cc2b_4f03_8ff6_02fd217db450);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextUpdatingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NewSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextTextUpdatingResult) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextTextUpdatingResult) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
