#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextCompositionCompletedEventArgs(pub ::windows::core::IInspectable);
impl CoreTextCompositionCompletedEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation_Collections`*"]
    pub fn CompositionSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs;{1f34ebb6-b79f-4121-a5e7-fda9b8616e30})");
}
unsafe impl ::windows::core::Interface for CoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f34ebb6_b79f_4121_a5e7_fda9b8616e30);
}
impl ::windows::core::RuntimeName for CoreTextCompositionCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs";
}
impl ::core::convert::From<CoreTextCompositionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextCompositionCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextCompositionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextCompositionCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextCompositionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextCompositionCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextCompositionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextCompositionCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextCompositionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextCompositionCompletedEventArgs {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextCompositionSegment(pub ::windows::core::IInspectable);
impl CoreTextCompositionSegment {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn PreconversionString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionSegment;{776c6bd9-4ead-4da7-8f47-3a88b523cc34})");
}
unsafe impl ::windows::core::Interface for CoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776c6bd9_4ead_4da7_8f47_3a88b523cc34);
}
impl ::windows::core::RuntimeName for CoreTextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionSegment";
}
impl ::core::convert::From<CoreTextCompositionSegment> for ::windows::core::IUnknown {
    fn from(value: CoreTextCompositionSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextCompositionSegment> for ::windows::core::IUnknown {
    fn from(value: &CoreTextCompositionSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextCompositionSegment> for ::windows::core::IInspectable {
    fn from(value: CoreTextCompositionSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextCompositionSegment> for ::windows::core::IInspectable {
    fn from(value: &CoreTextCompositionSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextCompositionSegment {}
unsafe impl ::core::marker::Sync for CoreTextCompositionSegment {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextCompositionStartedEventArgs(pub ::windows::core::IInspectable);
impl CoreTextCompositionStartedEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs;{276b16a9-64e7-4ab0-bc4b-a02d73835bfb})");
}
unsafe impl ::windows::core::Interface for CoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276b16a9_64e7_4ab0_bc4b_a02d73835bfb);
}
impl ::windows::core::RuntimeName for CoreTextCompositionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs";
}
impl ::core::convert::From<CoreTextCompositionStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextCompositionStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextCompositionStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextCompositionStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextCompositionStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextCompositionStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextCompositionStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextCompositionStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextCompositionStartedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextCompositionStartedEventArgs {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextEditContext(pub ::windows::core::IInspectable);
impl CoreTextEditContext {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn InputScope(&self) -> ::windows::core::Result<CoreTextInputScope> {
        let this = self;
        unsafe {
            let mut result__: CoreTextInputScope = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextInputScope>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetInputScope(&self, value: CoreTextInputScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn InputPaneDisplayPolicy(&self) -> ::windows::core::Result<CoreTextInputPaneDisplayPolicy> {
        let this = self;
        unsafe {
            let mut result__: CoreTextInputPaneDisplayPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextInputPaneDisplayPolicy>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetInputPaneDisplayPolicy(&self, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn TextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveTextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn SelectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveSelectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn LayoutRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveLayoutRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn TextUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveTextUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn SelectionUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveSelectionUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn FormatUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveFormatUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn CompositionStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveCompositionStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn CompositionCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveCompositionCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn FocusRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveFocusRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn NotifyFocusEnter(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn NotifyFocusLeave(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn NotifyTextChanged<'a, Param0: ::windows::core::IntoParam<'a, CoreTextRange>, Param2: ::windows::core::IntoParam<'a, CoreTextRange>>(&self, modifiedrange: Param0, newlength: i32, newselection: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), modifiedrange.into_param().abi(), newlength, newselection.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn NotifySelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, CoreTextRange>>(&self, selection: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn NotifyLayoutChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn NotifyFocusLeaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveNotifyFocusLeaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextEditContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextEditContext;{bf6608af-4041-47c3-b263-a918eb5eaef2})");
}
unsafe impl ::windows::core::Interface for CoreTextEditContext {
    type Vtable = ICoreTextEditContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6608af_4041_47c3_b263_a918eb5eaef2);
}
impl ::windows::core::RuntimeName for CoreTextEditContext {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextEditContext";
}
impl ::core::convert::From<CoreTextEditContext> for ::windows::core::IUnknown {
    fn from(value: CoreTextEditContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextEditContext> for ::windows::core::IUnknown {
    fn from(value: &CoreTextEditContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextEditContext> for ::windows::core::IInspectable {
    fn from(value: CoreTextEditContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextEditContext> for ::windows::core::IInspectable {
    fn from(value: &CoreTextEditContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextEditContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextEditContext {}
unsafe impl ::core::marker::Sync for CoreTextEditContext {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextFormatUpdatingEventArgs(pub ::windows::core::IInspectable);
impl CoreTextFormatUpdatingEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`, `UI_ViewManagement`*"]
    pub fn TextColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`, `UI_ViewManagement`*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`, `UI_ViewManagement`*"]
    pub fn UnderlineColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn UnderlineType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::UnderlineType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::UnderlineType>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Reason(&self) -> ::windows::core::Result<CoreTextFormatUpdatingReason> {
        let this = self;
        unsafe {
            let mut result__: CoreTextFormatUpdatingReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextFormatUpdatingReason>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Result(&self) -> ::windows::core::Result<CoreTextFormatUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextFormatUpdatingResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextFormatUpdatingResult>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetResult(&self, value: CoreTextFormatUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs;{7310bd33-b4a8-43b1-b37b-0724d4aca7ab})");
}
unsafe impl ::windows::core::Interface for CoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7310bd33_b4a8_43b1_b37b_0724d4aca7ab);
}
impl ::windows::core::RuntimeName for CoreTextFormatUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs";
}
impl ::core::convert::From<CoreTextFormatUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextFormatUpdatingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextFormatUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextFormatUpdatingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextFormatUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextFormatUpdatingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextFormatUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextFormatUpdatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextFormatUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextFormatUpdatingEventArgs {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingReason(pub i32);
impl CoreTextFormatUpdatingReason {
    pub const None: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(0i32);
    pub const CompositionUnconverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(1i32);
    pub const CompositionConverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(2i32);
    pub const CompositionTargetUnconverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(3i32);
    pub const CompositionTargetConverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(4i32);
}
impl ::core::convert::From<i32> for CoreTextFormatUpdatingReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreTextFormatUpdatingReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingReason;i4)");
}
impl ::windows::core::DefaultType for CoreTextFormatUpdatingReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: CoreTextFormatUpdatingResult = CoreTextFormatUpdatingResult(0i32);
    pub const Failed: CoreTextFormatUpdatingResult = CoreTextFormatUpdatingResult(1i32);
}
impl ::core::convert::From<i32> for CoreTextFormatUpdatingResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreTextFormatUpdatingResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingResult;i4)");
}
impl ::windows::core::DefaultType for CoreTextFormatUpdatingResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: CoreTextInputPaneDisplayPolicy = CoreTextInputPaneDisplayPolicy(0i32);
    pub const Manual: CoreTextInputPaneDisplayPolicy = CoreTextInputPaneDisplayPolicy(1i32);
}
impl ::core::convert::From<i32> for CoreTextInputPaneDisplayPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreTextInputPaneDisplayPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextInputPaneDisplayPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputPaneDisplayPolicy;i4)");
}
impl ::windows::core::DefaultType for CoreTextInputPaneDisplayPolicy {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: CoreTextInputScope = CoreTextInputScope(0i32);
    pub const Url: CoreTextInputScope = CoreTextInputScope(1i32);
    pub const FilePath: CoreTextInputScope = CoreTextInputScope(2i32);
    pub const FileName: CoreTextInputScope = CoreTextInputScope(3i32);
    pub const EmailUserName: CoreTextInputScope = CoreTextInputScope(4i32);
    pub const EmailAddress: CoreTextInputScope = CoreTextInputScope(5i32);
    pub const UserName: CoreTextInputScope = CoreTextInputScope(6i32);
    pub const PersonalFullName: CoreTextInputScope = CoreTextInputScope(7i32);
    pub const PersonalNamePrefix: CoreTextInputScope = CoreTextInputScope(8i32);
    pub const PersonalGivenName: CoreTextInputScope = CoreTextInputScope(9i32);
    pub const PersonalMiddleName: CoreTextInputScope = CoreTextInputScope(10i32);
    pub const PersonalSurname: CoreTextInputScope = CoreTextInputScope(11i32);
    pub const PersonalNameSuffix: CoreTextInputScope = CoreTextInputScope(12i32);
    pub const Address: CoreTextInputScope = CoreTextInputScope(13i32);
    pub const AddressPostalCode: CoreTextInputScope = CoreTextInputScope(14i32);
    pub const AddressStreet: CoreTextInputScope = CoreTextInputScope(15i32);
    pub const AddressStateOrProvince: CoreTextInputScope = CoreTextInputScope(16i32);
    pub const AddressCity: CoreTextInputScope = CoreTextInputScope(17i32);
    pub const AddressCountryName: CoreTextInputScope = CoreTextInputScope(18i32);
    pub const AddressCountryShortName: CoreTextInputScope = CoreTextInputScope(19i32);
    pub const CurrencyAmountAndSymbol: CoreTextInputScope = CoreTextInputScope(20i32);
    pub const CurrencyAmount: CoreTextInputScope = CoreTextInputScope(21i32);
    pub const Date: CoreTextInputScope = CoreTextInputScope(22i32);
    pub const DateMonth: CoreTextInputScope = CoreTextInputScope(23i32);
    pub const DateDay: CoreTextInputScope = CoreTextInputScope(24i32);
    pub const DateYear: CoreTextInputScope = CoreTextInputScope(25i32);
    pub const DateMonthName: CoreTextInputScope = CoreTextInputScope(26i32);
    pub const DateDayName: CoreTextInputScope = CoreTextInputScope(27i32);
    pub const Number: CoreTextInputScope = CoreTextInputScope(29i32);
    pub const SingleCharacter: CoreTextInputScope = CoreTextInputScope(30i32);
    pub const Password: CoreTextInputScope = CoreTextInputScope(31i32);
    pub const TelephoneNumber: CoreTextInputScope = CoreTextInputScope(32i32);
    pub const TelephoneCountryCode: CoreTextInputScope = CoreTextInputScope(33i32);
    pub const TelephoneAreaCode: CoreTextInputScope = CoreTextInputScope(34i32);
    pub const TelephoneLocalNumber: CoreTextInputScope = CoreTextInputScope(35i32);
    pub const Time: CoreTextInputScope = CoreTextInputScope(36i32);
    pub const TimeHour: CoreTextInputScope = CoreTextInputScope(37i32);
    pub const TimeMinuteOrSecond: CoreTextInputScope = CoreTextInputScope(38i32);
    pub const NumberFullWidth: CoreTextInputScope = CoreTextInputScope(39i32);
    pub const AlphanumericHalfWidth: CoreTextInputScope = CoreTextInputScope(40i32);
    pub const AlphanumericFullWidth: CoreTextInputScope = CoreTextInputScope(41i32);
    pub const CurrencyChinese: CoreTextInputScope = CoreTextInputScope(42i32);
    pub const Bopomofo: CoreTextInputScope = CoreTextInputScope(43i32);
    pub const Hiragana: CoreTextInputScope = CoreTextInputScope(44i32);
    pub const KatakanaHalfWidth: CoreTextInputScope = CoreTextInputScope(45i32);
    pub const KatakanaFullWidth: CoreTextInputScope = CoreTextInputScope(46i32);
    pub const Hanja: CoreTextInputScope = CoreTextInputScope(47i32);
    pub const HangulHalfWidth: CoreTextInputScope = CoreTextInputScope(48i32);
    pub const HangulFullWidth: CoreTextInputScope = CoreTextInputScope(49i32);
    pub const Search: CoreTextInputScope = CoreTextInputScope(50i32);
    pub const Formula: CoreTextInputScope = CoreTextInputScope(51i32);
    pub const SearchIncremental: CoreTextInputScope = CoreTextInputScope(52i32);
    pub const ChineseHalfWidth: CoreTextInputScope = CoreTextInputScope(53i32);
    pub const ChineseFullWidth: CoreTextInputScope = CoreTextInputScope(54i32);
    pub const NativeScript: CoreTextInputScope = CoreTextInputScope(55i32);
    pub const Text: CoreTextInputScope = CoreTextInputScope(57i32);
    pub const Chat: CoreTextInputScope = CoreTextInputScope(58i32);
    pub const NameOrPhoneNumber: CoreTextInputScope = CoreTextInputScope(59i32);
    pub const EmailUserNameOrAddress: CoreTextInputScope = CoreTextInputScope(60i32);
    pub const Private: CoreTextInputScope = CoreTextInputScope(61i32);
    pub const Maps: CoreTextInputScope = CoreTextInputScope(62i32);
    pub const PasswordNumeric: CoreTextInputScope = CoreTextInputScope(63i32);
    pub const FormulaNumber: CoreTextInputScope = CoreTextInputScope(67i32);
    pub const ChatWithoutEmoji: CoreTextInputScope = CoreTextInputScope(68i32);
    pub const Digits: CoreTextInputScope = CoreTextInputScope(28i32);
    pub const PinNumeric: CoreTextInputScope = CoreTextInputScope(64i32);
    pub const PinAlphanumeric: CoreTextInputScope = CoreTextInputScope(65i32);
}
impl ::core::convert::From<i32> for CoreTextInputScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreTextInputScope {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextInputScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputScope;i4)");
}
impl ::windows::core::DefaultType for CoreTextInputScope {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextLayoutBounds(pub ::windows::core::IInspectable);
impl CoreTextLayoutBounds {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn TextBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn SetTextBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn ControlBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn SetControlBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutBounds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutBounds;{e972c974-4436-4917-80d0-a525e4ca6780})");
}
unsafe impl ::windows::core::Interface for CoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe972c974_4436_4917_80d0_a525e4ca6780);
}
impl ::windows::core::RuntimeName for CoreTextLayoutBounds {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutBounds";
}
impl ::core::convert::From<CoreTextLayoutBounds> for ::windows::core::IUnknown {
    fn from(value: CoreTextLayoutBounds) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextLayoutBounds> for ::windows::core::IUnknown {
    fn from(value: &CoreTextLayoutBounds) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextLayoutBounds> for ::windows::core::IInspectable {
    fn from(value: CoreTextLayoutBounds) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextLayoutBounds> for ::windows::core::IInspectable {
    fn from(value: &CoreTextLayoutBounds) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextLayoutBounds {}
unsafe impl ::core::marker::Sync for CoreTextLayoutBounds {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextLayoutRequest(pub ::windows::core::IInspectable);
impl CoreTextLayoutRequest {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn LayoutBounds(&self) -> ::windows::core::Result<CoreTextLayoutBounds> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutBounds>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn LayoutBoundsVisualPixels(&self) -> ::windows::core::Result<CoreTextLayoutBounds> {
        let this = &::windows::core::Interface::cast::<ICoreTextLayoutRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutBounds>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequest;{2555a8cc-51fd-4f03-98bf-ac78174d68e0})");
}
unsafe impl ::windows::core::Interface for CoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2555a8cc_51fd_4f03_98bf_ac78174d68e0);
}
impl ::windows::core::RuntimeName for CoreTextLayoutRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequest";
}
impl ::core::convert::From<CoreTextLayoutRequest> for ::windows::core::IUnknown {
    fn from(value: CoreTextLayoutRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextLayoutRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreTextLayoutRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextLayoutRequest> for ::windows::core::IInspectable {
    fn from(value: CoreTextLayoutRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextLayoutRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreTextLayoutRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextLayoutRequest {}
unsafe impl ::core::marker::Sync for CoreTextLayoutRequest {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextLayoutRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreTextLayoutRequestedEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreTextLayoutRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs;{b1dc6ae0-9a7b-4e9e-a566-4a6b5f8ad676})");
}
unsafe impl ::windows::core::Interface for CoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1dc6ae0_9a7b_4e9e_a566_4a6b5f8ad676);
}
impl ::windows::core::RuntimeName for CoreTextLayoutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs";
}
impl ::core::convert::From<CoreTextLayoutRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextLayoutRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextLayoutRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextLayoutRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextLayoutRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextLayoutRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextLayoutRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextLayoutRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextLayoutRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextLayoutRequestedEventArgs {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_Text_Core`*"]
pub struct CoreTextRange {
    pub StartCaretPosition: i32,
    pub EndCaretPosition: i32,
}
impl CoreTextRange {}
impl ::core::default::Default for CoreTextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CoreTextRange {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CoreTextRange").field("StartCaretPosition", &self.StartCaretPosition).field("EndCaretPosition", &self.EndCaretPosition).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartCaretPosition == other.StartCaretPosition && self.EndCaretPosition == other.EndCaretPosition
    }
}
impl ::core::cmp::Eq for CoreTextRange {}
unsafe impl ::windows::core::Abi for CoreTextRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Text.Core.CoreTextRange;i4;i4)");
}
impl ::windows::core::DefaultType for CoreTextRange {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextSelectionRequest(pub ::windows::core::IInspectable);
impl CoreTextSelectionRequest {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Selection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetSelection<'a, Param0: ::windows::core::IntoParam<'a, CoreTextRange>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequest;{f0a70403-208b-4301-883c-74ca7485fd8d})");
}
unsafe impl ::windows::core::Interface for CoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a70403_208b_4301_883c_74ca7485fd8d);
}
impl ::windows::core::RuntimeName for CoreTextSelectionRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequest";
}
impl ::core::convert::From<CoreTextSelectionRequest> for ::windows::core::IUnknown {
    fn from(value: CoreTextSelectionRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextSelectionRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreTextSelectionRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextSelectionRequest> for ::windows::core::IInspectable {
    fn from(value: CoreTextSelectionRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextSelectionRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreTextSelectionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextSelectionRequest {}
unsafe impl ::core::marker::Sync for CoreTextSelectionRequest {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextSelectionRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreTextSelectionRequestedEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreTextSelectionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextSelectionRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs;{13c6682b-f614-421a-8f4b-9ec8a5a37fcd})");
}
unsafe impl ::windows::core::Interface for CoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13c6682b_f614_421a_8f4b_9ec8a5a37fcd);
}
impl ::windows::core::RuntimeName for CoreTextSelectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs";
}
impl ::core::convert::From<CoreTextSelectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextSelectionRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextSelectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextSelectionRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextSelectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextSelectionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextSelectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextSelectionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextSelectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextSelectionRequestedEventArgs {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextSelectionUpdatingEventArgs(pub ::windows::core::IInspectable);
impl CoreTextSelectionUpdatingEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Selection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Result(&self) -> ::windows::core::Result<CoreTextSelectionUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextSelectionUpdatingResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextSelectionUpdatingResult>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetResult(&self, value: CoreTextSelectionUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs;{d445839f-fe7f-4bd5-8a26-0922c1b3e639})");
}
unsafe impl ::windows::core::Interface for CoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd445839f_fe7f_4bd5_8a26_0922c1b3e639);
}
impl ::windows::core::RuntimeName for CoreTextSelectionUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs";
}
impl ::core::convert::From<CoreTextSelectionUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextSelectionUpdatingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextSelectionUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextSelectionUpdatingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextSelectionUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextSelectionUpdatingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextSelectionUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextSelectionUpdatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextSelectionUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextSelectionUpdatingEventArgs {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingResult(pub i32);
impl CoreTextSelectionUpdatingResult {
    pub const Succeeded: CoreTextSelectionUpdatingResult = CoreTextSelectionUpdatingResult(0i32);
    pub const Failed: CoreTextSelectionUpdatingResult = CoreTextSelectionUpdatingResult(1i32);
}
impl ::core::convert::From<i32> for CoreTextSelectionUpdatingResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreTextSelectionUpdatingResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextSelectionUpdatingResult;i4)");
}
impl ::windows::core::DefaultType for CoreTextSelectionUpdatingResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text_Core`*"]
pub struct CoreTextServicesConstants {}
impl CoreTextServicesConstants {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn HiddenCharacter() -> ::windows::core::Result<u16> {
        Self::ICoreTextServicesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn ICoreTextServicesStatics<R, F: FnOnce(&ICoreTextServicesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreTextServicesConstants, ICoreTextServicesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CoreTextServicesConstants {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesConstants";
}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextServicesManager(pub ::windows::core::IInspectable);
impl CoreTextServicesManager {
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `UI_Text_Core`, `Globalization`*"]
    pub fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn InputLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn RemoveInputLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn CreateEditContext(&self) -> ::windows::core::Result<CoreTextEditContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextEditContext>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreTextServicesManager> {
        Self::ICoreTextServicesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextServicesManager>(result__)
        })
    }
    pub fn ICoreTextServicesManagerStatics<R, F: FnOnce(&ICoreTextServicesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreTextServicesManager, ICoreTextServicesManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextServicesManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextServicesManager;{c2507d83-6e0a-4a8a-bdf8-1948874854ba})");
}
unsafe impl ::windows::core::Interface for CoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2507d83_6e0a_4a8a_bdf8_1948874854ba);
}
impl ::windows::core::RuntimeName for CoreTextServicesManager {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesManager";
}
impl ::core::convert::From<CoreTextServicesManager> for ::windows::core::IUnknown {
    fn from(value: CoreTextServicesManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextServicesManager> for ::windows::core::IUnknown {
    fn from(value: &CoreTextServicesManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextServicesManager> for ::windows::core::IInspectable {
    fn from(value: CoreTextServicesManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextServicesManager> for ::windows::core::IInspectable {
    fn from(value: &CoreTextServicesManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextServicesManager {}
unsafe impl ::core::marker::Sync for CoreTextServicesManager {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextTextRequest(pub ::windows::core::IInspectable);
impl CoreTextTextRequest {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequest;{50d950a9-f51e-4cc1-8ca1-e6346d1a61be})");
}
unsafe impl ::windows::core::Interface for CoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d950a9_f51e_4cc1_8ca1_e6346d1a61be);
}
impl ::windows::core::RuntimeName for CoreTextTextRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequest";
}
impl ::core::convert::From<CoreTextTextRequest> for ::windows::core::IUnknown {
    fn from(value: CoreTextTextRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextTextRequest> for ::windows::core::IUnknown {
    fn from(value: &CoreTextTextRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextTextRequest> for ::windows::core::IInspectable {
    fn from(value: CoreTextTextRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextTextRequest> for ::windows::core::IInspectable {
    fn from(value: &CoreTextTextRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextTextRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextTextRequest {}
unsafe impl ::core::marker::Sync for CoreTextTextRequest {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextTextRequestedEventArgs(pub ::windows::core::IInspectable);
impl CoreTextTextRequestedEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Request(&self) -> ::windows::core::Result<CoreTextTextRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextTextRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequestedEventArgs;{f096a2d0-41c6-4c02-8b1a-d953b00cabb3})");
}
unsafe impl ::windows::core::Interface for CoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf096a2d0_41c6_4c02_8b1a_d953b00cabb3);
}
impl ::windows::core::RuntimeName for CoreTextTextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequestedEventArgs";
}
impl ::core::convert::From<CoreTextTextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextTextRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextTextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextTextRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextTextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextTextRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextTextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextTextRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextTextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextTextRequestedEventArgs {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreTextTextUpdatingEventArgs(pub ::windows::core::IInspectable);
impl CoreTextTextUpdatingEventArgs {
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn NewSelection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `UI_Text_Core`, `Globalization`*"]
    pub fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn Result(&self) -> ::windows::core::Result<CoreTextTextUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextTextUpdatingResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextTextUpdatingResult>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn SetResult(&self, value: CoreTextTextUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Text_Core`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Text_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs;{eea7918d-cc2b-4f03-8ff6-02fd217db450})");
}
unsafe impl ::windows::core::Interface for CoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea7918d_cc2b_4f03_8ff6_02fd217db450);
}
impl ::windows::core::RuntimeName for CoreTextTextUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs";
}
impl ::core::convert::From<CoreTextTextUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreTextTextUpdatingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreTextTextUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreTextTextUpdatingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreTextTextUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreTextTextUpdatingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreTextTextUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreTextTextUpdatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreTextTextUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextTextUpdatingEventArgs {}
#[doc = "*Required features: `UI_Text_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextTextUpdatingResult(pub i32);
impl CoreTextTextUpdatingResult {
    pub const Succeeded: CoreTextTextUpdatingResult = CoreTextTextUpdatingResult(0i32);
    pub const Failed: CoreTextTextUpdatingResult = CoreTextTextUpdatingResult(1i32);
}
impl ::core::convert::From<i32> for CoreTextTextUpdatingResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreTextTextUpdatingResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextTextUpdatingResult;i4)");
}
impl ::windows::core::DefaultType for CoreTextTextUpdatingResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextCompositionCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f34ebb6_b79f_4121_a5e7_fda9b8616e30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextCompositionSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776c6bd9_4ead_4da7_8f47_3a88b523cc34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextCompositionStartedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276b16a9_64e7_4ab0_bc4b_a02d73835bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextEditContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextEditContext {
    type Vtable = ICoreTextEditContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6608af_4041_47c3_b263_a918eb5eaef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextInputScope) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreTextInputScope) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextEditContext2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextEditContext2 {
    type Vtable = ICoreTextEditContext2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1867dbb_083b_49e1_b281_2b35d62bf466);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextFormatUpdatingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7310bd33_b4a8_43b1_b37b_0724d4aca7ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextFormatUpdatingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextFormatUpdatingReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextLayoutBounds(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe972c974_4436_4917_80d0_a525e4ca6780);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutBounds_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2555a8cc_51fd_4f03_98bf_ac78174d68e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequest2 {
    type Vtable = ICoreTextLayoutRequest2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x676de624_cd3d_4bcd_bf01_7f7110954511);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest2_abi(
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
pub struct ICoreTextLayoutRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1dc6ae0_9a7b_4e9e_a566_4a6b5f8ad676);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequestedEventArgs_abi(
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
pub struct ICoreTextSelectionRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a70403_208b_4301_883c_74ca7485fd8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13c6682b_f614_421a_8f4b_9ec8a5a37fcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequestedEventArgs_abi(
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
pub struct ICoreTextSelectionUpdatingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd445839f_fe7f_4bd5_8a26_0922c1b3e639);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionUpdatingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextServicesManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2507d83_6e0a_4a8a_bdf8_1948874854ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextServicesManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextServicesManagerStatics {
    type Vtable = ICoreTextServicesManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1520a388_e2cf_4d65_aeb9_b32d86fe39b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManagerStatics_abi(
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
pub struct ICoreTextServicesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextServicesStatics {
    type Vtable = ICoreTextServicesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91859a46_eccf_47a4_8ae7_098a9c6fbb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextTextRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d950a9_f51e_4cc1_8ca1_e6346d1a61be);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTextTextRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf096a2d0_41c6_4c02_8b1a_d953b00cabb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequestedEventArgs_abi(
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
pub struct ICoreTextTextUpdatingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea7918d_cc2b_4f03_8ff6_02fd217db450);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextUpdatingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreTextTextUpdatingResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreTextTextUpdatingResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
