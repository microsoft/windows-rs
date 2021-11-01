#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub mod ShareTarget;
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
pub struct Clipboard {}
impl Clipboard {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn GetContent() -> ::windows::runtime::Result<DataPackageView> {
        Self::IClipboardStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageView>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, DataPackage>>(content: Param0) -> ::windows::runtime::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), content.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Flush() -> ::windows::runtime::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Clear() -> ::windows::runtime::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ContentChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveContentChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetHistoryItemsAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ClipboardHistoryItemsResult>> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClipboardHistoryItemsResult>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ClearHistory() -> ::windows::runtime::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn DeleteItemFromHistory<'a, Param0: ::windows::runtime::IntoParam<'a, ClipboardHistoryItem>>(item: Param0) -> ::windows::runtime::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetHistoryItemAsContent<'a, Param0: ::windows::runtime::IntoParam<'a, ClipboardHistoryItem>>(item: Param0) -> ::windows::runtime::Result<SetHistoryItemAsContentStatus> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: SetHistoryItemAsContentStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<SetHistoryItemAsContentStatus>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn IsHistoryEnabled() -> ::windows::runtime::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn IsRoamingEnabled() -> ::windows::runtime::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetContentWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, DataPackage>, Param1: ::windows::runtime::IntoParam<'a, ClipboardContentOptions>>(content: Param0, options: Param1) -> ::windows::runtime::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), content.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn HistoryChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ClipboardHistoryChangedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveHistoryChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IClipboardStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RoamingEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveRoamingEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IClipboardStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn HistoryEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveHistoryEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IClipboardStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IClipboardStatics<R, F: FnOnce(&IClipboardStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Clipboard, IClipboardStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IClipboardStatics2<R, F: FnOnce(&IClipboardStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Clipboard, IClipboardStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for Clipboard {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.Clipboard";
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClipboardContentOptions(::windows::runtime::IInspectable);
impl ClipboardContentOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ClipboardContentOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn IsRoamable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetIsRoamable(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn IsAllowedInHistory(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetIsAllowedInHistory(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn RoamingFormats(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn HistoryFormats(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClipboardContentOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardContentOptions;{e888a98c-ad4b-5447-a056-ab3556276d2b})");
}
unsafe impl ::windows::runtime::Interface for ClipboardContentOptions {
    type Vtable = IClipboardContentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3901270412, 44363, 21575, [160, 86, 171, 53, 86, 39, 109, 43]);
}
impl ::windows::runtime::RuntimeName for ClipboardContentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardContentOptions";
}
impl ::std::convert::From<ClipboardContentOptions> for ::windows::runtime::IUnknown {
    fn from(value: ClipboardContentOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClipboardContentOptions> for ::windows::runtime::IUnknown {
    fn from(value: &ClipboardContentOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClipboardContentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClipboardContentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClipboardContentOptions> for ::windows::runtime::IInspectable {
    fn from(value: ClipboardContentOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClipboardContentOptions> for ::windows::runtime::IInspectable {
    fn from(value: &ClipboardContentOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClipboardContentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClipboardContentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClipboardContentOptions {}
unsafe impl ::std::marker::Sync for ClipboardContentOptions {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClipboardHistoryChangedEventArgs(::windows::runtime::IInspectable);
impl ClipboardHistoryChangedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ClipboardHistoryChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs;{c0be453f-8ea2-53ce-9aba-8d2212573452})");
}
unsafe impl ::windows::runtime::Interface for ClipboardHistoryChangedEventArgs {
    type Vtable = IClipboardHistoryChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3233695039, 36514, 21454, [154, 186, 141, 34, 18, 87, 52, 82]);
}
impl ::windows::runtime::RuntimeName for ClipboardHistoryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs";
}
impl ::std::convert::From<ClipboardHistoryChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ClipboardHistoryChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClipboardHistoryChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ClipboardHistoryChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClipboardHistoryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClipboardHistoryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClipboardHistoryChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ClipboardHistoryChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClipboardHistoryChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ClipboardHistoryChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClipboardHistoryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClipboardHistoryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClipboardHistoryChangedEventArgs {}
unsafe impl ::std::marker::Sync for ClipboardHistoryChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClipboardHistoryItem(::windows::runtime::IInspectable);
impl ClipboardHistoryItem {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageView>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClipboardHistoryItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem;{0173bd8a-afff-5c50-ab92-3d19f481ec58})");
}
unsafe impl ::windows::runtime::Interface for ClipboardHistoryItem {
    type Vtable = IClipboardHistoryItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(24362378, 45055, 23632, [171, 146, 61, 25, 244, 129, 236, 88]);
}
impl ::windows::runtime::RuntimeName for ClipboardHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem";
}
impl ::std::convert::From<ClipboardHistoryItem> for ::windows::runtime::IUnknown {
    fn from(value: ClipboardHistoryItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClipboardHistoryItem> for ::windows::runtime::IUnknown {
    fn from(value: &ClipboardHistoryItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClipboardHistoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClipboardHistoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClipboardHistoryItem> for ::windows::runtime::IInspectable {
    fn from(value: ClipboardHistoryItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClipboardHistoryItem> for ::windows::runtime::IInspectable {
    fn from(value: &ClipboardHistoryItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClipboardHistoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClipboardHistoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClipboardHistoryItem {}
unsafe impl ::std::marker::Sync for ClipboardHistoryItem {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClipboardHistoryItemsResult(::windows::runtime::IInspectable);
impl ClipboardHistoryItemsResult {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ClipboardHistoryItemsResultStatus> {
        let this = self;
        unsafe {
            let mut result__: ClipboardHistoryItemsResultStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClipboardHistoryItemsResultStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ClipboardHistoryItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ClipboardHistoryItem>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClipboardHistoryItemsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult;{e6dfdee6-0ee2-52e3-852b-f295db65939a})");
}
unsafe impl ::windows::runtime::Interface for ClipboardHistoryItemsResult {
    type Vtable = IClipboardHistoryItemsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3873431270, 3810, 21219, [133, 43, 242, 149, 219, 101, 147, 154]);
}
impl ::windows::runtime::RuntimeName for ClipboardHistoryItemsResult {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult";
}
impl ::std::convert::From<ClipboardHistoryItemsResult> for ::windows::runtime::IUnknown {
    fn from(value: ClipboardHistoryItemsResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClipboardHistoryItemsResult> for ::windows::runtime::IUnknown {
    fn from(value: &ClipboardHistoryItemsResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClipboardHistoryItemsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClipboardHistoryItemsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClipboardHistoryItemsResult> for ::windows::runtime::IInspectable {
    fn from(value: ClipboardHistoryItemsResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClipboardHistoryItemsResult> for ::windows::runtime::IInspectable {
    fn from(value: &ClipboardHistoryItemsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClipboardHistoryItemsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClipboardHistoryItemsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClipboardHistoryItemsResult {}
unsafe impl ::std::marker::Sync for ClipboardHistoryItemsResult {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClipboardHistoryItemsResultStatus(pub i32);
impl ClipboardHistoryItemsResultStatus {
    pub const Success: ClipboardHistoryItemsResultStatus = ClipboardHistoryItemsResultStatus(0i32);
    pub const AccessDenied: ClipboardHistoryItemsResultStatus = ClipboardHistoryItemsResultStatus(1i32);
    pub const ClipboardHistoryDisabled: ClipboardHistoryItemsResultStatus = ClipboardHistoryItemsResultStatus(2i32);
}
impl ::std::convert::From<i32> for ClipboardHistoryItemsResultStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ClipboardHistoryItemsResultStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ClipboardHistoryItemsResultStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResultStatus;i4)");
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataPackage(::windows::runtime::IInspectable);
impl DataPackage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataPackage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageView>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<DataPackagePropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackagePropertySet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn RequestedOperation(&self) -> ::windows::runtime::Result<DataPackageOperation> {
        let this = self;
        unsafe {
            let mut result__: DataPackageOperation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetRequestedOperation(&self, value: DataPackageOperation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn OperationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DataPackage, OperationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveOperationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn Destroyed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DataPackage, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveDestroyed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, formatid: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), formatid.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetDataProvider<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, DataProviderHandler>>(&self, formatid: Param0, delayrenderer: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), formatid.into_param().abi(), delayrenderer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetHtmlFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn ResourceMap(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetRtf<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn SetBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::RandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`, `Storage`*"]
    pub fn SetStorageItemsReadOnly<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`, `Storage`*"]
    pub fn SetStorageItems<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>>(&self, value: Param0, readonly: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi(), readonly).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SetApplicationLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SetWebLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ShareCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IDataPackage3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveShareCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackage3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ShareCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DataPackage, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IDataPackage4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveShareCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackage4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataPackage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackage;{61ebf5c7-efea-4346-9554-981d7e198ffe})");
}
unsafe impl ::windows::runtime::Interface for DataPackage {
    type Vtable = IDataPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1642853831, 61418, 17222, [149, 84, 152, 29, 126, 25, 143, 254]);
}
impl ::windows::runtime::RuntimeName for DataPackage {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackage";
}
impl ::std::convert::From<DataPackage> for ::windows::runtime::IUnknown {
    fn from(value: DataPackage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataPackage> for ::windows::runtime::IUnknown {
    fn from(value: &DataPackage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataPackage> for ::windows::runtime::IInspectable {
    fn from(value: DataPackage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataPackage> for ::windows::runtime::IInspectable {
    fn from(value: &DataPackage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataPackage {}
unsafe impl ::std::marker::Sync for DataPackage {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataPackageOperation(pub u32);
impl DataPackageOperation {
    pub const None: DataPackageOperation = DataPackageOperation(0u32);
    pub const Copy: DataPackageOperation = DataPackageOperation(1u32);
    pub const Move: DataPackageOperation = DataPackageOperation(2u32);
    pub const Link: DataPackageOperation = DataPackageOperation(4u32);
}
impl ::std::convert::From<u32> for DataPackageOperation {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DataPackageOperation {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DataPackageOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DataPackageOperation;u4)");
}
impl ::std::ops::BitOr for DataPackageOperation {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DataPackageOperation {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DataPackageOperation {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DataPackageOperation {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DataPackageOperation {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataPackagePropertySet(::windows::runtime::IInspectable);
impl DataPackagePropertySet {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn FileTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ApplicationName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetApplicationName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ApplicationListingUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SetApplicationListingUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ContentSourceWebLink(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SetContentSourceWebLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ContentSourceApplicationLink(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SetContentSourceApplicationLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn Square30x30Logo(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn SetSquare30x30Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `UI`*"]
    pub fn LogoBackgroundColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `UI`*"]
    pub fn SetLogoBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn EnterpriseId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetEnterpriseId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ContentSourceUserActivityJson(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet4>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetContentSourceUserActivityJson<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySet4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataPackagePropertySet {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackagePropertySet;{cd1c93eb-4c4c-443a-a8d3-f5c241e91689})");
}
unsafe impl ::windows::runtime::Interface for DataPackagePropertySet {
    type Vtable = IDataPackagePropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3441202155, 19532, 17466, [168, 211, 245, 194, 65, 233, 22, 137]);
}
impl ::windows::runtime::RuntimeName for DataPackagePropertySet {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackagePropertySet";
}
impl ::std::convert::From<DataPackagePropertySet> for ::windows::runtime::IUnknown {
    fn from(value: DataPackagePropertySet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataPackagePropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &DataPackagePropertySet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataPackagePropertySet> for ::windows::runtime::IInspectable {
    fn from(value: DataPackagePropertySet) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataPackagePropertySet> for ::windows::runtime::IInspectable {
    fn from(value: &DataPackagePropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<DataPackagePropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataPackagePropertySet) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&DataPackagePropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataPackagePropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<DataPackagePropertySet> for super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataPackagePropertySet) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&DataPackagePropertySet> for super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataPackagePropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &DataPackagePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DataPackagePropertySet {}
unsafe impl ::std::marker::Sync for DataPackagePropertySet {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for DataPackagePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &DataPackagePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataPackagePropertySetView(::windows::runtime::IInspectable);
impl DataPackagePropertySetView {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn FileTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ApplicationName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ApplicationListingUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ContentSourceWebLink(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ContentSourceApplicationLink(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn Square30x30Logo(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `UI`*"]
    pub fn LogoBackgroundColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn EnterpriseId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::std::option::Option<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>, second: &mut ::std::option::Option<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ContentSourceUserActivityJson(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView4>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn IsFromRoamingClipboard(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDataPackagePropertySetView5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataPackagePropertySetView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView;{b94cec01-0c1a-4c57-be55-75d01289735d})");
}
unsafe impl ::windows::runtime::Interface for DataPackagePropertySetView {
    type Vtable = IDataPackagePropertySetView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3108826113, 3098, 19543, [190, 85, 117, 208, 18, 137, 115, 93]);
}
impl ::windows::runtime::RuntimeName for DataPackagePropertySetView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView";
}
impl ::std::convert::From<DataPackagePropertySetView> for ::windows::runtime::IUnknown {
    fn from(value: DataPackagePropertySetView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataPackagePropertySetView> for ::windows::runtime::IUnknown {
    fn from(value: &DataPackagePropertySetView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataPackagePropertySetView> for ::windows::runtime::IInspectable {
    fn from(value: DataPackagePropertySetView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataPackagePropertySetView> for ::windows::runtime::IInspectable {
    fn from(value: &DataPackagePropertySetView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<DataPackagePropertySetView> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataPackagePropertySetView) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&DataPackagePropertySetView> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataPackagePropertySetView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<DataPackagePropertySetView> for super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataPackagePropertySetView) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&DataPackagePropertySetView> for super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataPackagePropertySetView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &DataPackagePropertySetView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DataPackagePropertySetView {}
unsafe impl ::std::marker::Sync for DataPackagePropertySetView {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for DataPackagePropertySetView {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &DataPackagePropertySetView {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataPackageView(::windows::runtime::IInspectable);
impl DataPackageView {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<DataPackagePropertySetView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackagePropertySetView>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn RequestedOperation(&self) -> ::windows::runtime::Result<DataPackageOperation> {
        let this = self;
        unsafe {
            let mut result__: DataPackageOperation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ReportOperationCompleted(&self, value: DataPackageOperation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn AvailableFormats(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Contains<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, formatid: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), formatid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, formatid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), formatid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetTextAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetCustomTextAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, formatid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), formatid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetUriAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetHtmlFormatAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn GetResourceMapAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetRtfAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`, `Storage_Streams`*"]
    pub fn GetBitmapAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`, `Foundation_Collections`, `Storage`*"]
    pub fn GetStorageItemsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetApplicationLinkAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = &::windows::runtime::Interface::cast::<IDataPackageView2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetWebLinkAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = &::windows::runtime::Interface::cast::<IDataPackageView2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`, `Security_EnterpriseData`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>> {
        let this = &::windows::runtime::Interface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`, `Security_EnterpriseData`*"]
    pub fn RequestAccessWithEnterpriseIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, enterpriseid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>> {
        let this = &::windows::runtime::Interface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), enterpriseid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>(result__)
        }
    }
    #[cfg(feature = "Security_EnterpriseData")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Security_EnterpriseData`*"]
    pub fn UnlockAndAssumeEnterpriseIdentity(&self) -> ::windows::runtime::Result<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult> {
        let this = &::windows::runtime::Interface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__: super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetAcceptedFormatId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, formatid: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataPackageView4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), formatid.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataPackageView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackageView;{7b840471-5900-4d85-a90b-10cb85fe3552})");
}
unsafe impl ::windows::runtime::Interface for DataPackageView {
    type Vtable = IDataPackageView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2072249457, 22784, 19845, [169, 11, 16, 203, 133, 254, 53, 82]);
}
impl ::windows::runtime::RuntimeName for DataPackageView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackageView";
}
impl ::std::convert::From<DataPackageView> for ::windows::runtime::IUnknown {
    fn from(value: DataPackageView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataPackageView> for ::windows::runtime::IUnknown {
    fn from(value: &DataPackageView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataPackageView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataPackageView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataPackageView> for ::windows::runtime::IInspectable {
    fn from(value: DataPackageView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataPackageView> for ::windows::runtime::IInspectable {
    fn from(value: &DataPackageView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataPackageView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataPackageView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataPackageView {}
unsafe impl ::std::marker::Sync for DataPackageView {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataProviderDeferral(::windows::runtime::IInspectable);
impl DataProviderDeferral {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataProviderDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataProviderDeferral;{c2cf2373-2d26-43d9-b69d-dcb86d03f6da})");
}
unsafe impl ::windows::runtime::Interface for DataProviderDeferral {
    type Vtable = IDataProviderDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3268354931, 11558, 17369, [182, 157, 220, 184, 109, 3, 246, 218]);
}
impl ::windows::runtime::RuntimeName for DataProviderDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataProviderDeferral";
}
impl ::std::convert::From<DataProviderDeferral> for ::windows::runtime::IUnknown {
    fn from(value: DataProviderDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataProviderDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &DataProviderDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataProviderDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataProviderDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataProviderDeferral> for ::windows::runtime::IInspectable {
    fn from(value: DataProviderDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataProviderDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &DataProviderDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataProviderDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataProviderDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataProviderDeferral {}
unsafe impl ::std::marker::Sync for DataProviderDeferral {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataProviderHandler(::windows::runtime::IUnknown);
impl DataProviderHandler {
    pub fn new<F: FnMut(&::std::option::Option<DataProviderRequest>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DataProviderHandler_box::<F> {
            vtable: &DataProviderHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, DataProviderRequest>>(&self, request: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), request.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataProviderHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({e7ecd720-f2f4-4a2d-920e-170a2f482a27})");
}
unsafe impl ::windows::runtime::Interface for DataProviderHandler {
    type Vtable = DataProviderHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3891058464, 62196, 18989, [146, 14, 23, 10, 47, 72, 42, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct DataProviderHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DataProviderHandler_box<F: FnMut(&::std::option::Option<DataProviderRequest>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DataProviderHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<DataProviderRequest>) -> ::windows::runtime::Result<()> + 'static> DataProviderHandler_box<F> {
    const VTABLE: DataProviderHandler_abi = DataProviderHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DataProviderHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&request as *const <DataProviderRequest as ::windows::runtime::Abi>::Abi as *const <DataProviderRequest as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataProviderRequest(::windows::runtime::IInspectable);
impl DataProviderRequest {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn FormatId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<DataProviderDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataProviderDeferral>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataProviderRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataProviderRequest;{ebbc7157-d3c8-47da-acde-f82388d5f716})");
}
unsafe impl ::windows::runtime::Interface for DataProviderRequest {
    type Vtable = IDataProviderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3954995543, 54216, 18394, [172, 222, 248, 35, 136, 213, 247, 22]);
}
impl ::windows::runtime::RuntimeName for DataProviderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataProviderRequest";
}
impl ::std::convert::From<DataProviderRequest> for ::windows::runtime::IUnknown {
    fn from(value: DataProviderRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataProviderRequest> for ::windows::runtime::IUnknown {
    fn from(value: &DataProviderRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataProviderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataProviderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataProviderRequest> for ::windows::runtime::IInspectable {
    fn from(value: DataProviderRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataProviderRequest> for ::windows::runtime::IInspectable {
    fn from(value: &DataProviderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataProviderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataProviderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataProviderRequest {}
unsafe impl ::std::marker::Sync for DataProviderRequest {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataRequest(::windows::runtime::IInspectable);
impl DataRequest {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<DataPackage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackage>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, DataPackage>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn FailWithDisplayText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<DataRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataRequestDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataRequest;{4341ae3b-fc12-4e53-8c02-ac714c415a27})");
}
unsafe impl ::windows::runtime::Interface for DataRequest {
    type Vtable = IDataRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1128377915, 64530, 20051, [140, 2, 172, 113, 76, 65, 90, 39]);
}
impl ::windows::runtime::RuntimeName for DataRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataRequest";
}
impl ::std::convert::From<DataRequest> for ::windows::runtime::IUnknown {
    fn from(value: DataRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataRequest> for ::windows::runtime::IUnknown {
    fn from(value: &DataRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataRequest> for ::windows::runtime::IInspectable {
    fn from(value: DataRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataRequest> for ::windows::runtime::IInspectable {
    fn from(value: &DataRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataRequest {}
unsafe impl ::std::marker::Sync for DataRequest {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataRequestDeferral(::windows::runtime::IInspectable);
impl DataRequestDeferral {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataRequestDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataRequestDeferral;{6dc4b89f-0386-4263-87c1-ed7dce30890e})");
}
unsafe impl ::windows::runtime::Interface for DataRequestDeferral {
    type Vtable = IDataRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1841608863, 902, 16995, [135, 193, 237, 125, 206, 48, 137, 14]);
}
impl ::windows::runtime::RuntimeName for DataRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataRequestDeferral";
}
impl ::std::convert::From<DataRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: DataRequestDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &DataRequestDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: DataRequestDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &DataRequestDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataRequestDeferral {}
unsafe impl ::std::marker::Sync for DataRequestDeferral {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataRequestedEventArgs(::windows::runtime::IInspectable);
impl DataRequestedEventArgs {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<DataRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs;{cb8ba807-6ac5-43c9-8ac5-9ba232163182})");
}
unsafe impl ::windows::runtime::Interface for DataRequestedEventArgs {
    type Vtable = IDataRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3414927367, 27333, 17353, [138, 197, 155, 162, 50, 22, 49, 130]);
}
impl ::windows::runtime::RuntimeName for DataRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs";
}
impl ::std::convert::From<DataRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DataRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DataRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DataRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DataRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataRequestedEventArgs {}
unsafe impl ::std::marker::Sync for DataRequestedEventArgs {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataTransferManager(::windows::runtime::IInspectable);
impl DataTransferManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn DataRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DataTransferManager, DataRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveDataRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn TargetApplicationChosen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DataTransferManager, TargetApplicationChosenEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveTargetApplicationChosen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ShowShareUI() -> ::windows::runtime::Result<()> {
        Self::IDataTransferManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<DataTransferManager> {
        Self::IDataTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataTransferManager>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IDataTransferManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ShareProvidersRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DataTransferManager, ShareProvidersRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IDataTransferManager2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn RemoveShareProvidersRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDataTransferManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ShowShareUIWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ShareUIOptions>>(options: Param0) -> ::windows::runtime::Result<()> {
        Self::IDataTransferManagerStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), options.into_param().abi()).ok() })
    }
    pub fn IDataTransferManagerStatics<R, F: FnOnce(&IDataTransferManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataTransferManager, IDataTransferManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDataTransferManagerStatics2<R, F: FnOnce(&IDataTransferManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataTransferManager, IDataTransferManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDataTransferManagerStatics3<R, F: FnOnce(&IDataTransferManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataTransferManager, IDataTransferManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataTransferManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataTransferManager;{a5caee9b-8708-49d1-8d36-67d25a8da00c})");
}
unsafe impl ::windows::runtime::Interface for DataTransferManager {
    type Vtable = IDataTransferManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2781539995, 34568, 18897, [141, 54, 103, 210, 90, 141, 160, 12]);
}
impl ::windows::runtime::RuntimeName for DataTransferManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataTransferManager";
}
impl ::std::convert::From<DataTransferManager> for ::windows::runtime::IUnknown {
    fn from(value: DataTransferManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataTransferManager> for ::windows::runtime::IUnknown {
    fn from(value: &DataTransferManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataTransferManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataTransferManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataTransferManager> for ::windows::runtime::IInspectable {
    fn from(value: DataTransferManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataTransferManager> for ::windows::runtime::IInspectable {
    fn from(value: &DataTransferManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataTransferManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataTransferManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
pub struct HtmlFormatHelper {}
impl HtmlFormatHelper {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn GetStaticFragment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(htmlformat: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IHtmlFormatHelperStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), htmlformat.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn CreateHtmlFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(htmlfragment: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IHtmlFormatHelperStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), htmlfragment.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IHtmlFormatHelperStatics<R, F: FnOnce(&IHtmlFormatHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HtmlFormatHelper, IHtmlFormatHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for HtmlFormatHelper {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.HtmlFormatHelper";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IClipboardContentOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClipboardContentOptions {
    type Vtable = IClipboardContentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3901270412, 44363, 21575, [160, 86, 171, 53, 86, 39, 109, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardContentOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IClipboardHistoryChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClipboardHistoryChangedEventArgs {
    type Vtable = IClipboardHistoryChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3233695039, 36514, 21454, [154, 186, 141, 34, 18, 87, 52, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardHistoryChangedEventArgs_abi(
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
pub struct IClipboardHistoryItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClipboardHistoryItem {
    type Vtable = IClipboardHistoryItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(24362378, 45055, 23632, [171, 146, 61, 25, 244, 129, 236, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardHistoryItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IClipboardHistoryItemsResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClipboardHistoryItemsResult {
    type Vtable = IClipboardHistoryItemsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3873431270, 3810, 21219, [133, 43, 242, 149, 219, 101, 147, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardHistoryItemsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClipboardHistoryItemsResultStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IClipboardStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClipboardStatics {
    type Vtable = IClipboardStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3324502673, 13538, 18787, [142, 237, 147, 203, 176, 234, 61, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IClipboardStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClipboardStatics2 {
    type Vtable = IClipboardStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534494570, 53919, 21835, [179, 3, 240, 69, 35, 69, 254, 2]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr, result__: *mut SetHistoryItemAsContentStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackage {
    type Vtable = IDataPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1642853831, 61418, 17222, [149, 84, 152, 29, 126, 25, 143, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DataPackageOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DataPackageOperation) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, delayrenderer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, readonly: bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackage2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackage2 {
    type Vtable = IDataPackage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(68952041, 9225, 17889, [165, 56, 76, 83, 238, 238, 4, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackage3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackage3 {
    type Vtable = IDataPackage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2297634653, 30843, 19762, [150, 90, 169, 131, 129, 5, 160, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackage4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackage4 {
    type Vtable = IDataPackage4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(329404104, 37762, 21359, [133, 42, 48, 69, 225, 178, 154, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySet(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySet {
    type Vtable = IDataPackagePropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3441202155, 19532, 17466, [168, 211, 245, 194, 65, 233, 22, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySet2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySet2 {
    type Vtable = IDataPackagePropertySet2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3947912522, 38912, 18090, [177, 129, 123, 111, 15, 43, 145, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySet3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySet3 {
    type Vtable = IDataPackagePropertySet3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2659712411, 20997, 16411, [135, 74, 69, 86, 83, 189, 57, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySet4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySet4 {
    type Vtable = IDataPackagePropertySet4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1670441973, 5945, 19572, [178, 47, 134, 95, 171, 94, 133, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySetView {
    type Vtable = IDataPackagePropertySetView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3108826113, 3098, 19543, [190, 85, 117, 208, 18, 137, 115, 93]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySetView2 {
    type Vtable = IDataPackagePropertySetView2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1616138395, 36542, 20459, [156, 30, 117, 230, 157, 229, 75, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySetView3 {
    type Vtable = IDataPackagePropertySetView3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3681963237, 53620, 18780, [132, 252, 26, 81, 246, 171, 69, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySetView4 {
    type Vtable = IDataPackagePropertySetView4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148504077, 53615, 16558, [149, 128, 111, 133, 98, 185, 66, 53]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackagePropertySetView5 {
    type Vtable = IDataPackagePropertySetView5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1862964293, 14176, 20667, [133, 35, 196, 32, 45, 237, 125, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackageView(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackageView {
    type Vtable = IDataPackageView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2072249457, 22784, 19845, [169, 11, 16, 203, 133, 254, 53, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DataPackageOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DataPackageOperation) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackageView2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackageView2 {
    type Vtable = IDataPackageView2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1089256085, 9296, 19485, [182, 180, 237, 69, 70, 61, 238, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackageView3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackageView3 {
    type Vtable = IDataPackageView3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3547820456, 56749, 17032, [132, 40, 209, 202, 227, 148, 18, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_EnterpriseData")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enterpriseid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_EnterpriseData")))] usize,
    #[cfg(feature = "Security_EnterpriseData")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_EnterpriseData"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPackageView4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPackageView4 {
    type Vtable = IDataPackageView4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3756617503, 57410, 17459, [160, 159, 38, 214, 255, 218, 139, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataProviderDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProviderDeferral {
    type Vtable = IDataProviderDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3268354931, 11558, 17369, [182, 157, 220, 184, 109, 3, 246, 218]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProviderDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataProviderRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProviderRequest {
    type Vtable = IDataProviderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3954995543, 54216, 18394, [172, 222, 248, 35, 136, 213, 247, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProviderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataRequest {
    type Vtable = IDataRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1128377915, 64530, 20051, [140, 2, 172, 113, 76, 65, 90, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataRequestDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataRequestDeferral {
    type Vtable = IDataRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1841608863, 902, 16995, [135, 193, 237, 125, 206, 48, 137, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRequestDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataRequestedEventArgs {
    type Vtable = IDataRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3414927367, 27333, 17353, [138, 197, 155, 162, 50, 22, 49, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataTransferManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTransferManager {
    type Vtable = IDataTransferManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2781539995, 34568, 18897, [141, 54, 103, 210, 90, 141, 160, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataTransferManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTransferManager2 {
    type Vtable = IDataTransferManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(816741745, 35752, 19458, [142, 63, 221, 178, 59, 56, 135, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTransferManagerStatics {
    type Vtable = IDataTransferManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2849636778, 57358, 19710, [170, 68, 45, 217, 50, 220, 163, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTransferManagerStatics2 {
    type Vtable = IDataTransferManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3310273260, 40855, 19811, [152, 104, 57, 94, 39, 26, 216, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTransferManagerStatics3 {
    type Vtable = IDataTransferManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(92558451, 27778, 20316, [172, 35, 98, 228, 88, 54, 31, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHtmlFormatHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHtmlFormatHelperStatics {
    type Vtable = IHtmlFormatHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3794696009, 56688, 17519, [174, 252, 97, 206, 229, 159, 101, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlFormatHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlformat: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlfragment: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOperationCompletedEventArgs {
    type Vtable = IOperationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3887018653, 1309, 20395, [177, 169, 71, 253, 119, 247, 10, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DataPackageOperation) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOperationCompletedEventArgs2 {
    type Vtable = IOperationCompletedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2240782451, 7705, 16645, [178, 247, 200, 71, 136, 8, 213, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IShareCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareCompletedEventArgs {
    type Vtable = IShareCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1165280322, 63763, 20320, [157, 247, 204, 64, 96, 171, 25, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IShareProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareProvider {
    type Vtable = IShareProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(799793190, 17470, 19674, [175, 37, 141, 129, 7, 14, 253, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IShareProviderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareProviderFactory {
    type Vtable = IShareProviderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(388634444, 59294, 20333, [176, 125, 18, 143, 70, 158, 2, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProviderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Storage_Streams", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayicon: ::windows::runtime::RawPtr, backgroundcolor: super::super::UI::Color, handler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "UI")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IShareProviderOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareProviderOperation {
    type Vtable = IShareProviderOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(432994615, 54325, 16761, [182, 175, 20, 224, 73, 43, 105, 246]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProviderOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IShareProvidersRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareProvidersRequestedEventArgs {
    type Vtable = IShareProvidersRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4169724758, 41976, 20430, [133, 228, 136, 38, 230, 59, 231, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProvidersRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IShareTargetInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareTargetInfo {
    type Vtable = IShareTargetInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945546759, 50920, 16660, [178, 148, 40, 243, 187, 111, 153, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IShareUIOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareUIOptions {
    type Vtable = IShareUIOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1929022080, 13359, 19856, [149, 81, 42, 224, 78, 55, 104, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareUIOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ShareUITheme) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ShareUITheme) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISharedStorageAccessManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedStorageAccessManagerStatics {
    type Vtable = ISharedStorageAccessManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3323144922, 13489, 18505, [189, 95, 208, 159, 238, 49, 88, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedStorageAccessManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardDataFormatsStatics {
    type Vtable = IStandardDataFormatsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2127987105, 43136, 16585, [180, 237, 11, 238, 30, 21, 245, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardDataFormatsStatics2 {
    type Vtable = IStandardDataFormatsStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1117934836, 40310, 17128, [134, 27, 71, 194, 93, 208, 207, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardDataFormatsStatics3 {
    type Vtable = IStandardDataFormatsStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(995602537, 468, 18252, [139, 95, 188, 142, 39, 243, 139, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITargetApplicationChosenEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITargetApplicationChosenEventArgs {
    type Vtable = ITargetApplicationChosenEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3396319404, 10631, 20195, [156, 84, 216, 175, 188, 184, 108, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetApplicationChosenEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OperationCompletedEventArgs(::windows::runtime::IInspectable);
impl OperationCompletedEventArgs {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Operation(&self) -> ::windows::runtime::Result<DataPackageOperation> {
        let this = self;
        unsafe {
            let mut result__: DataPackageOperation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn AcceptedFormatId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IOperationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OperationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs;{e7af329d-051d-4fab-b1a9-47fd77f70a41})");
}
unsafe impl ::windows::runtime::Interface for OperationCompletedEventArgs {
    type Vtable = IOperationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3887018653, 1309, 20395, [177, 169, 71, 253, 119, 247, 10, 65]);
}
impl ::windows::runtime::RuntimeName for OperationCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs";
}
impl ::std::convert::From<OperationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: OperationCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OperationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &OperationCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OperationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OperationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OperationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: OperationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OperationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &OperationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OperationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OperationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OperationCompletedEventArgs {}
unsafe impl ::std::marker::Sync for OperationCompletedEventArgs {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SetHistoryItemAsContentStatus(pub i32);
impl SetHistoryItemAsContentStatus {
    pub const Success: SetHistoryItemAsContentStatus = SetHistoryItemAsContentStatus(0i32);
    pub const AccessDenied: SetHistoryItemAsContentStatus = SetHistoryItemAsContentStatus(1i32);
    pub const ItemDeleted: SetHistoryItemAsContentStatus = SetHistoryItemAsContentStatus(2i32);
}
impl ::std::convert::From<i32> for SetHistoryItemAsContentStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SetHistoryItemAsContentStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SetHistoryItemAsContentStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.SetHistoryItemAsContentStatus;i4)");
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ShareCompletedEventArgs(::windows::runtime::IInspectable);
impl ShareCompletedEventArgs {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ShareTarget(&self) -> ::windows::runtime::Result<ShareTargetInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ShareTargetInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs;{4574c442-f913-4f60-9df7-cc4060ab1916})");
}
unsafe impl ::windows::runtime::Interface for ShareCompletedEventArgs {
    type Vtable = IShareCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1165280322, 63763, 20320, [157, 247, 204, 64, 96, 171, 25, 22]);
}
impl ::windows::runtime::RuntimeName for ShareCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs";
}
impl ::std::convert::From<ShareCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ShareCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ShareCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ShareCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShareCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ShareCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ShareCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ShareCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShareCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ShareCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShareCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShareCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ShareCompletedEventArgs {}
unsafe impl ::std::marker::Sync for ShareCompletedEventArgs {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ShareProvider(::windows::runtime::IInspectable);
impl ShareProvider {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`*"]
    pub fn DisplayIcon(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `UI`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "UI"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage_Streams`, `UI`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::RandomAccessStreamReference>, Param2: ::windows::runtime::IntoParam<'a, super::super::UI::Color>, Param3: ::windows::runtime::IntoParam<'a, ShareProviderHandler>>(title: Param0, displayicon: Param1, backgroundcolor: Param2, handler: Param3) -> ::windows::runtime::Result<ShareProvider> {
        Self::IShareProviderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), title.into_param().abi(), displayicon.into_param().abi(), backgroundcolor.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<ShareProvider>(result__)
        })
    }
    pub fn IShareProviderFactory<R, F: FnOnce(&IShareProviderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ShareProvider, IShareProviderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareProvider;{2fabe026-443e-4cda-af25-8d81070efd80})");
}
unsafe impl ::windows::runtime::Interface for ShareProvider {
    type Vtable = IShareProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(799793190, 17470, 19674, [175, 37, 141, 129, 7, 14, 253, 128]);
}
impl ::windows::runtime::RuntimeName for ShareProvider {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareProvider";
}
impl ::std::convert::From<ShareProvider> for ::windows::runtime::IUnknown {
    fn from(value: ShareProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ShareProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ShareProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShareProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ShareProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ShareProvider> for ::windows::runtime::IInspectable {
    fn from(value: ShareProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShareProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ShareProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShareProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShareProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ShareProvider {}
unsafe impl ::std::marker::Sync for ShareProvider {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ShareProviderHandler(::windows::runtime::IUnknown);
impl ShareProviderHandler {
    pub fn new<F: FnMut(&::std::option::Option<ShareProviderOperation>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ShareProviderHandler_box::<F> {
            vtable: &ShareProviderHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ShareProviderOperation>>(&self, operation: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), operation.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareProviderHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({e7f9d9ba-e1ba-4e4d-bd65-d43845d3212f})");
}
unsafe impl ::windows::runtime::Interface for ShareProviderHandler {
    type Vtable = ShareProviderHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3891911098, 57786, 20045, [189, 101, 212, 56, 69, 211, 33, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ShareProviderHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ShareProviderHandler_box<F: FnMut(&::std::option::Option<ShareProviderOperation>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ShareProviderHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<ShareProviderOperation>) -> ::windows::runtime::Result<()> + 'static> ShareProviderHandler_box<F> {
    const VTABLE: ShareProviderHandler_abi = ShareProviderHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ShareProviderHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, operation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&operation as *const <ShareProviderOperation as ::windows::runtime::Abi>::Abi as *const <ShareProviderOperation as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ShareProviderOperation(::windows::runtime::IInspectable);
impl ShareProviderOperation {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageView>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Provider(&self) -> ::windows::runtime::Result<ShareProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ShareProvider>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ReportCompleted(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareProviderOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareProviderOperation;{19cef937-d435-4179-b6af-14e0492b69f6})");
}
unsafe impl ::windows::runtime::Interface for ShareProviderOperation {
    type Vtable = IShareProviderOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(432994615, 54325, 16761, [182, 175, 20, 224, 73, 43, 105, 246]);
}
impl ::windows::runtime::RuntimeName for ShareProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareProviderOperation";
}
impl ::std::convert::From<ShareProviderOperation> for ::windows::runtime::IUnknown {
    fn from(value: ShareProviderOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ShareProviderOperation> for ::windows::runtime::IUnknown {
    fn from(value: &ShareProviderOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShareProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ShareProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ShareProviderOperation> for ::windows::runtime::IInspectable {
    fn from(value: ShareProviderOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShareProviderOperation> for ::windows::runtime::IInspectable {
    fn from(value: &ShareProviderOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShareProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShareProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ShareProviderOperation {}
unsafe impl ::std::marker::Sync for ShareProviderOperation {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ShareProvidersRequestedEventArgs(::windows::runtime::IInspectable);
impl ShareProvidersRequestedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation_Collections`*"]
    pub fn Providers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ShareProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ShareProvider>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPackageView>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareProvidersRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs;{f888f356-a3f8-4fce-85e4-8826e63be799})");
}
unsafe impl ::windows::runtime::Interface for ShareProvidersRequestedEventArgs {
    type Vtable = IShareProvidersRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4169724758, 41976, 20430, [133, 228, 136, 38, 230, 59, 231, 153]);
}
impl ::windows::runtime::RuntimeName for ShareProvidersRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs";
}
impl ::std::convert::From<ShareProvidersRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ShareProvidersRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ShareProvidersRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ShareProvidersRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShareProvidersRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ShareProvidersRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ShareProvidersRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ShareProvidersRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShareProvidersRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ShareProvidersRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShareProvidersRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShareProvidersRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ShareProvidersRequestedEventArgs {}
unsafe impl ::std::marker::Sync for ShareProvidersRequestedEventArgs {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ShareTargetInfo(::windows::runtime::IInspectable);
impl ShareTargetInfo {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn AppUserModelId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ShareProvider(&self) -> ::windows::runtime::Result<ShareProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ShareProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareTargetInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareTargetInfo;{385be607-c6e8-4114-b294-28f3bb6f9904})");
}
unsafe impl ::windows::runtime::Interface for ShareTargetInfo {
    type Vtable = IShareTargetInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945546759, 50920, 16660, [178, 148, 40, 243, 187, 111, 153, 4]);
}
impl ::windows::runtime::RuntimeName for ShareTargetInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTargetInfo";
}
impl ::std::convert::From<ShareTargetInfo> for ::windows::runtime::IUnknown {
    fn from(value: ShareTargetInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ShareTargetInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ShareTargetInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShareTargetInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ShareTargetInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ShareTargetInfo> for ::windows::runtime::IInspectable {
    fn from(value: ShareTargetInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShareTargetInfo> for ::windows::runtime::IInspectable {
    fn from(value: &ShareTargetInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShareTargetInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShareTargetInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ShareTargetInfo {}
unsafe impl ::std::marker::Sync for ShareTargetInfo {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ShareUIOptions(::windows::runtime::IInspectable);
impl ShareUIOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ShareUIOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Theme(&self) -> ::windows::runtime::Result<ShareUITheme> {
        let this = self;
        unsafe {
            let mut result__: ShareUITheme = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ShareUITheme>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn SetTheme(&self, value: ShareUITheme) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SelectionRect(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Rect>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn SetSelectionRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::Rect>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareUIOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareUIOptions;{72fa8a80-342f-4d90-9551-2ae04e37680c})");
}
unsafe impl ::windows::runtime::Interface for ShareUIOptions {
    type Vtable = IShareUIOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1929022080, 13359, 19856, [149, 81, 42, 224, 78, 55, 104, 12]);
}
impl ::windows::runtime::RuntimeName for ShareUIOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareUIOptions";
}
impl ::std::convert::From<ShareUIOptions> for ::windows::runtime::IUnknown {
    fn from(value: ShareUIOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ShareUIOptions> for ::windows::runtime::IUnknown {
    fn from(value: &ShareUIOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShareUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ShareUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ShareUIOptions> for ::windows::runtime::IInspectable {
    fn from(value: ShareUIOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShareUIOptions> for ::windows::runtime::IInspectable {
    fn from(value: &ShareUIOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShareUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShareUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ShareUIOptions {}
unsafe impl ::std::marker::Sync for ShareUIOptions {}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ShareUITheme(pub i32);
impl ShareUITheme {
    pub const Default: ShareUITheme = ShareUITheme(0i32);
    pub const Light: ShareUITheme = ShareUITheme(1i32);
    pub const Dark: ShareUITheme = ShareUITheme(2i32);
}
impl ::std::convert::From<i32> for ShareUITheme {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ShareUITheme {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ShareUITheme {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.ShareUITheme;i4)");
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
pub struct SharedStorageAccessManager {}
impl SharedStorageAccessManager {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Storage`*"]
    pub fn AddFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISharedStorageAccessManagerStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`, `Foundation`, `Storage`*"]
    pub fn RedeemTokenForFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(token: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>> {
        Self::ISharedStorageAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn RemoveFile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ISharedStorageAccessManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn ISharedStorageAccessManagerStatics<R, F: FnOnce(&ISharedStorageAccessManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SharedStorageAccessManager, ISharedStorageAccessManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SharedStorageAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager";
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
pub struct StandardDataFormats {}
impl StandardDataFormats {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Text() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Uri() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Html() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Rtf() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn Bitmap() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn StorageItems() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn WebLink() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ApplicationLink() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn UserActivityJsonArray() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardDataFormatsStatics3(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IStandardDataFormatsStatics<R, F: FnOnce(&IStandardDataFormatsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardDataFormats, IStandardDataFormatsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardDataFormatsStatics2<R, F: FnOnce(&IStandardDataFormatsStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardDataFormats, IStandardDataFormatsStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardDataFormatsStatics3<R, F: FnOnce(&IStandardDataFormatsStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardDataFormats, IStandardDataFormatsStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for StandardDataFormats {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.StandardDataFormats";
}
#[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TargetApplicationChosenEventArgs(::windows::runtime::IInspectable);
impl TargetApplicationChosenEventArgs {
    #[doc = "*Required features: `ApplicationModel_DataTransfer`*"]
    pub fn ApplicationName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TargetApplicationChosenEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs;{ca6fb8ac-2987-4ee3-9c54-d8afbcb86c1d})");
}
unsafe impl ::windows::runtime::Interface for TargetApplicationChosenEventArgs {
    type Vtable = ITargetApplicationChosenEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3396319404, 10631, 20195, [156, 84, 216, 175, 188, 184, 108, 29]);
}
impl ::windows::runtime::RuntimeName for TargetApplicationChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs";
}
impl ::std::convert::From<TargetApplicationChosenEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: TargetApplicationChosenEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TargetApplicationChosenEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &TargetApplicationChosenEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TargetApplicationChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TargetApplicationChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TargetApplicationChosenEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: TargetApplicationChosenEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TargetApplicationChosenEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &TargetApplicationChosenEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TargetApplicationChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TargetApplicationChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TargetApplicationChosenEventArgs {}
unsafe impl ::std::marker::Sync for TargetApplicationChosenEventArgs {}
