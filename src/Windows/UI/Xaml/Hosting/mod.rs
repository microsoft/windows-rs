#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DesignerAppExitedEventArgs(pub ::windows::core::IInspectable);
impl DesignerAppExitedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DesignerAppExitedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs;{f6aac86a-0cad-410c-8f62-dc2936151c74})");
}
unsafe impl ::windows::core::Interface for DesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6aac86a_0cad_410c_8f62_dc2936151c74);
}
impl ::windows::core::RuntimeName for DesignerAppExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs";
}
impl ::core::convert::From<DesignerAppExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DesignerAppExitedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DesignerAppExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DesignerAppExitedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DesignerAppExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DesignerAppExitedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DesignerAppExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DesignerAppExitedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DesignerAppExitedEventArgs {}
unsafe impl ::core::marker::Sync for DesignerAppExitedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DesignerAppManager(pub ::windows::core::IInspectable);
impl DesignerAppManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn DesignerAppExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn RemoveDesignerAppExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn CreateNewViewAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, initialviewstate: DesignerAppViewState, initialviewsize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DesignerAppView>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), initialviewstate, initialviewsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DesignerAppView>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn LoadObjectIntoAppAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, dllname: Param0, classid: Param1, initializationdata: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dllname.into_param().abi(), classid.into_param().abi(), initializationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appusermodelid: Param0) -> ::windows::core::Result<DesignerAppManager> {
        Self::IDesignerAppManagerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appusermodelid.into_param().abi(), &mut result__).from_abi::<DesignerAppManager>(result__)
        })
    }
    pub fn IDesignerAppManagerFactory<R, F: FnOnce(&IDesignerAppManagerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DesignerAppManager, IDesignerAppManagerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DesignerAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppManager;{a6272caa-d5c6-40cb-abd9-27ba43831bb7})");
}
unsafe impl ::windows::core::Interface for DesignerAppManager {
    type Vtable = IDesignerAppManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6272caa_d5c6_40cb_abd9_27ba43831bb7);
}
impl ::windows::core::RuntimeName for DesignerAppManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppManager";
}
impl ::core::convert::From<DesignerAppManager> for ::windows::core::IUnknown {
    fn from(value: DesignerAppManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DesignerAppManager> for ::windows::core::IUnknown {
    fn from(value: &DesignerAppManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DesignerAppManager> for ::windows::core::IInspectable {
    fn from(value: DesignerAppManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DesignerAppManager> for ::windows::core::IInspectable {
    fn from(value: &DesignerAppManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DesignerAppManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DesignerAppManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DesignerAppManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesignerAppManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesignerAppManager {}
unsafe impl ::core::marker::Sync for DesignerAppManager {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DesignerAppView(pub ::windows::core::IInspectable);
impl DesignerAppView {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn ApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn ViewState(&self) -> ::windows::core::Result<DesignerAppViewState> {
        let this = self;
        unsafe {
            let mut result__: DesignerAppViewState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DesignerAppViewState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn ViewSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn UpdateViewAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, viewstate: DesignerAppViewState, viewsize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), viewstate, viewsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DesignerAppView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppView;{5c777cea-dd71-4a84-a56f-dacb4b14706f})");
}
unsafe impl ::windows::core::Interface for DesignerAppView {
    type Vtable = IDesignerAppView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c777cea_dd71_4a84_a56f_dacb4b14706f);
}
impl ::windows::core::RuntimeName for DesignerAppView {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppView";
}
impl ::core::convert::From<DesignerAppView> for ::windows::core::IUnknown {
    fn from(value: DesignerAppView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DesignerAppView> for ::windows::core::IUnknown {
    fn from(value: &DesignerAppView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DesignerAppView> for ::windows::core::IInspectable {
    fn from(value: DesignerAppView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DesignerAppView> for ::windows::core::IInspectable {
    fn from(value: &DesignerAppView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DesignerAppView> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DesignerAppView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DesignerAppView> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesignerAppView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesignerAppView {}
unsafe impl ::core::marker::Sync for DesignerAppView {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DesignerAppViewState(pub i32);
impl DesignerAppViewState {
    pub const Visible: DesignerAppViewState = DesignerAppViewState(0i32);
    pub const Hidden: DesignerAppViewState = DesignerAppViewState(1i32);
}
impl ::core::convert::From<i32> for DesignerAppViewState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DesignerAppViewState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DesignerAppViewState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.DesignerAppViewState;i4)");
}
impl ::windows::core::DefaultType for DesignerAppViewState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DesktopWindowXamlSource(pub ::windows::core::IInspectable);
impl DesktopWindowXamlSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn Content(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn HasFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn TakeFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn RemoveTakeFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn NavigateFocus<'a, Param0: ::windows::core::IntoParam<'a, XamlSourceFocusNavigationRequest>>(&self, request: Param0) -> ::windows::core::Result<XamlSourceFocusNavigationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationResult>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn new() -> ::windows::core::Result<DesktopWindowXamlSource> {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<DesktopWindowXamlSource>(result__)
        })
    }
    pub fn IDesktopWindowXamlSourceFactory<R, F: FnOnce(&IDesktopWindowXamlSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSource;{d585bfe1-00ff-51be-ba1d-a1329956ea0a})");
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd585bfe1_00ff_51be_ba1d_a1329956ea0a);
}
impl ::windows::core::RuntimeName for DesktopWindowXamlSource {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource";
}
impl ::core::convert::From<DesktopWindowXamlSource> for ::windows::core::IUnknown {
    fn from(value: DesktopWindowXamlSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DesktopWindowXamlSource> for ::windows::core::IUnknown {
    fn from(value: &DesktopWindowXamlSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DesktopWindowXamlSource> for ::windows::core::IInspectable {
    fn from(value: DesktopWindowXamlSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DesktopWindowXamlSource> for ::windows::core::IInspectable {
    fn from(value: &DesktopWindowXamlSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DesktopWindowXamlSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopWindowXamlSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DesktopWindowXamlSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopWindowXamlSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSource {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSource {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(pub ::windows::core::IInspectable);
impl DesktopWindowXamlSourceGotFocusEventArgs {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSourceGotFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs;{39be4849-d9cc-5b70-8f05-1ad9a4aaa342})");
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39be4849_d9cc_5b70_8f05_1ad9a4aaa342);
}
impl ::windows::core::RuntimeName for DesktopWindowXamlSourceGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs";
}
impl ::core::convert::From<DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceGotFocusEventArgs {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(pub ::windows::core::IInspectable);
impl DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs;{fe61e4b9-a7af-52b3-bdb9-c3305c0b8df2})");
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe61e4b9_a7af_52b3_bdb9_c3305c0b8df2);
}
impl ::windows::core::RuntimeName for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs";
}
impl ::core::convert::From<DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ElementCompositionPreview(pub ::windows::core::IInspectable);
impl ElementCompositionPreview {
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn GetElementVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn GetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn SetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::Visual>>(element: Param0, visual: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), visual.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`, `UI_Xaml_Controls`*"]
    pub fn GetScrollViewerManipulationPropertySet<'a, Param0: ::windows::core::IntoParam<'a, super::Controls::ScrollViewer>>(scrollviewer: Param0) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), scrollviewer.into_param().abi(), &mut result__).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn SetImplicitShowAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn SetImplicitHideAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetIsTranslationEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn GetPointerPositionPropertySet<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(targetelement: Param0) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), targetelement.into_param().abi(), &mut result__).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(feature = "UI_WindowManagement")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_WindowManagement`*"]
    pub fn SetAppWindowContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(appwindow: Param0, xamlcontent: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appwindow.into_param().abi(), xamlcontent.into_param().abi()).ok() })
    }
    #[cfg(feature = "UI_WindowManagement")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_WindowManagement`*"]
    pub fn GetAppWindowContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(appwindow: Param0) -> ::windows::core::Result<super::UIElement> {
        Self::IElementCompositionPreviewStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appwindow.into_param().abi(), &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    pub fn IElementCompositionPreviewStatics<R, F: FnOnce(&IElementCompositionPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IElementCompositionPreviewStatics2<R, F: FnOnce(&IElementCompositionPreviewStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IElementCompositionPreviewStatics3<R, F: FnOnce(&IElementCompositionPreviewStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.ElementCompositionPreview;{b6f1a676-cfe6-46ac-acf6-c4687bb65e60})");
}
unsafe impl ::windows::core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6f1a676_cfe6_46ac_acf6_c4687bb65e60);
}
impl ::windows::core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.ElementCompositionPreview";
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: ElementCompositionPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: &ElementCompositionPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: ElementCompositionPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: &ElementCompositionPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct HostingContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDesignerAppExitedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6aac86a_0cad_410c_8f62_dc2936151c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppExitedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDesignerAppManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesignerAppManager {
    type Vtable = IDesignerAppManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6272caa_d5c6_40cb_abd9_27ba43831bb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialviewstate: DesignerAppViewState, initialviewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dllname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, classid: ::windows::core::GUID, initializationdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDesignerAppManagerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesignerAppManagerFactory {
    type Vtable = IDesignerAppManagerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f9d633b_1266_4c0e_8499_0db85bbd4c43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManagerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDesignerAppView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesignerAppView {
    type Vtable = IDesignerAppView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c777cea_dd71_4a84_a56f_dacb4b14706f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DesignerAppViewState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewstate: DesignerAppViewState, viewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd585bfe1_00ff_51be_ba1d_a1329956ea0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceFactory {
    type Vtable = IDesktopWindowXamlSourceFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cd61dc0_2561_56e1_8e75_6e44173805e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39be4849_d9cc_5b70_8f05_1ad9a4aaa342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs_abi(
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
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe61e4b9_a7af_52b3_bdb9_c3305c0b8df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_abi(
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
pub struct IElementCompositionPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6f1a676_cfe6_46ac_acf6_c4687bb65e60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c92b38_ec99_4c55_bc85_a1c180b27646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scrollviewer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Controls")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics2 {
    type Vtable = IElementCompositionPreviewStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24148fbb_23d6_4f37_ba0c_0733e799722d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics3 {
    type Vtable = IElementCompositionPreviewStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x843bc4c3_c105_59fe_a3d1_373c1d3e6fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: ::windows::core::RawPtr, xamlcontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowsXamlManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowsXamlManager {
    type Vtable = IWindowsXamlManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56096c31_1aa0_5288_8818_6e74a2dcaff5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowsXamlManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowsXamlManagerStatics {
    type Vtable = IWindowsXamlManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28258a12_7d82_505b_b210_712b04a58882);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManagerStatics_abi(
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
pub struct IXamlSourceFocusNavigationRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbb93bb5_1496_5a80_ac00_e757359755e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut XamlSourceFocusNavigationReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationRequestFactory {
    type Vtable = IXamlSourceFocusNavigationRequestFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe746ab8f_b4ef_5390_97e5_cc0a2779c574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: XamlSourceFocusNavigationReason, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, correlationid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d55a5f_9603_5d8f_9cc7_d1c4070d9801);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResult_abi(
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
pub struct IXamlSourceFocusNavigationResultFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationResultFactory {
    type Vtable = IXamlSourceFocusNavigationResultFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bbadbf_f9e1_5527_b8c5_09339ff2ca76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResultFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusmoved: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUIPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUIPresenter {
    type Vtable = IXamlUIPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa714944a_1619_4fc6_b31b_89512ef022a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: i32, height: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
pub struct IXamlUIPresenterHost(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost {
    type Vtable = IXamlUIPresenterHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafb84cd_9f6d_4f80_ac2c_0e6cb9f31659);
}
impl IXamlUIPresenterHost {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn ResolveFileResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, path: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{aafb84cd-9f6d-4f80-ac2c-0e6cb9f31659}");
}
impl ::core::convert::From<IXamlUIPresenterHost> for ::windows::core::IUnknown {
    fn from(value: IXamlUIPresenterHost) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost> for ::windows::core::IUnknown {
    fn from(value: &IXamlUIPresenterHost) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlUIPresenterHost> for ::windows::core::IInspectable {
    fn from(value: IXamlUIPresenterHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost> for ::windows::core::IInspectable {
    fn from(value: &IXamlUIPresenterHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
pub struct IXamlUIPresenterHost2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost2 {
    type Vtable = IXamlUIPresenterHost2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61595672_7ca4_4a21_b56a_88f4812388ca);
}
impl IXamlUIPresenterHost2 {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn GetGenericXamlFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61595672-7ca4-4a21-b56a-88f4812388ca}");
}
impl ::core::convert::From<IXamlUIPresenterHost2> for ::windows::core::IUnknown {
    fn from(value: IXamlUIPresenterHost2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost2> for ::windows::core::IUnknown {
    fn from(value: &IXamlUIPresenterHost2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlUIPresenterHost2> for ::windows::core::IInspectable {
    fn from(value: IXamlUIPresenterHost2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost2> for ::windows::core::IInspectable {
    fn from(value: &IXamlUIPresenterHost2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
pub struct IXamlUIPresenterHost3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost3 {
    type Vtable = IXamlUIPresenterHost3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14292bf_7320_41bb_9f26_4d6fd34db45a);
}
impl IXamlUIPresenterHost3 {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn ResolveDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, super::ResourceDictionary>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, dictionary: Param0, dictionarykey: Param1, suggestedvalue: Param2) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dictionary.into_param().abi(), dictionarykey.into_param().abi(), suggestedvalue.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost3 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b14292bf-7320-41bb-9f26-4d6fd34db45a}");
}
impl ::core::convert::From<IXamlUIPresenterHost3> for ::windows::core::IUnknown {
    fn from(value: IXamlUIPresenterHost3) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost3> for ::windows::core::IUnknown {
    fn from(value: &IXamlUIPresenterHost3) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlUIPresenterHost3> for ::windows::core::IInspectable {
    fn from(value: IXamlUIPresenterHost3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost3> for ::windows::core::IInspectable {
    fn from(value: &IXamlUIPresenterHost3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dictionary: ::windows::core::RawPtr, dictionarykey: ::windows::core::RawPtr, suggestedvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUIPresenterStatics {
    type Vtable = IXamlUIPresenterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71eaeac8_45e1_4192_85aa_3a422edd23cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUIPresenterStatics2 {
    type Vtable = IXamlUIPresenterStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c6b68d2_cf1c_4f53_bf09_6a745f7a9703);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, placementtarget: ::windows::core::RawPtr, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: *mut bool, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        placementtargetbounds: super::super::super::Foundation::Rect,
        controlsize: super::super::super::Foundation::Size,
        mincontrolsize: super::super::super::Foundation::Size,
        containerrect: super::super::super::Foundation::Rect,
        targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode,
        allowfallbacks: bool,
        chosenplacement: *mut super::Controls::Primitives::FlyoutPlacementMode,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))] usize,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WindowsXamlManager(pub ::windows::core::IInspectable);
impl WindowsXamlManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn InitializeForCurrentThread() -> ::windows::core::Result<WindowsXamlManager> {
        Self::IWindowsXamlManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WindowsXamlManager>(result__)
        })
    }
    pub fn IWindowsXamlManagerStatics<R, F: FnOnce(&IWindowsXamlManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowsXamlManager, IWindowsXamlManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsXamlManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.WindowsXamlManager;{56096c31-1aa0-5288-8818-6e74a2dcaff5})");
}
unsafe impl ::windows::core::Interface for WindowsXamlManager {
    type Vtable = IWindowsXamlManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56096c31_1aa0_5288_8818_6e74a2dcaff5);
}
impl ::windows::core::RuntimeName for WindowsXamlManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.WindowsXamlManager";
}
impl ::core::convert::From<WindowsXamlManager> for ::windows::core::IUnknown {
    fn from(value: WindowsXamlManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WindowsXamlManager> for ::windows::core::IUnknown {
    fn from(value: &WindowsXamlManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WindowsXamlManager> for ::windows::core::IInspectable {
    fn from(value: WindowsXamlManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WindowsXamlManager> for ::windows::core::IInspectable {
    fn from(value: &WindowsXamlManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<WindowsXamlManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: WindowsXamlManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&WindowsXamlManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &WindowsXamlManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WindowsXamlManager {}
unsafe impl ::core::marker::Sync for WindowsXamlManager {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XamlSourceFocusNavigationReason(pub i32);
impl XamlSourceFocusNavigationReason {
    pub const Programmatic: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(0i32);
    pub const Restore: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(1i32);
    pub const First: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(3i32);
    pub const Last: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(4i32);
    pub const Left: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(7i32);
    pub const Up: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(8i32);
    pub const Right: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(9i32);
    pub const Down: XamlSourceFocusNavigationReason = XamlSourceFocusNavigationReason(10i32);
}
impl ::core::convert::From<i32> for XamlSourceFocusNavigationReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XamlSourceFocusNavigationReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationReason;i4)");
}
impl ::windows::core::DefaultType for XamlSourceFocusNavigationReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlSourceFocusNavigationRequest(pub ::windows::core::IInspectable);
impl XamlSourceFocusNavigationRequest {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn Reason(&self) -> ::windows::core::Result<XamlSourceFocusNavigationReason> {
        let this = self;
        unsafe {
            let mut result__: XamlSourceFocusNavigationReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationReason>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn CreateInstance(reason: XamlSourceFocusNavigationReason) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn CreateInstanceWithHintRect<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(reason: XamlSourceFocusNavigationReason, hintrect: Param1) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, hintrect.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`*"]
    pub fn CreateInstanceWithHintRectAndCorrelationId<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(reason: XamlSourceFocusNavigationReason, hintrect: Param1, correlationid: Param2) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), reason, hintrect.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    pub fn IXamlSourceFocusNavigationRequestFactory<R, F: FnOnce(&IXamlSourceFocusNavigationRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlSourceFocusNavigationRequest, IXamlSourceFocusNavigationRequestFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest;{fbb93bb5-1496-5a80-ac00-e757359755e6})");
}
unsafe impl ::windows::core::Interface for XamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbb93bb5_1496_5a80_ac00_e757359755e6);
}
impl ::windows::core::RuntimeName for XamlSourceFocusNavigationRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest";
}
impl ::core::convert::From<XamlSourceFocusNavigationRequest> for ::windows::core::IUnknown {
    fn from(value: XamlSourceFocusNavigationRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationRequest> for ::windows::core::IUnknown {
    fn from(value: &XamlSourceFocusNavigationRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlSourceFocusNavigationRequest> for ::windows::core::IInspectable {
    fn from(value: XamlSourceFocusNavigationRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationRequest> for ::windows::core::IInspectable {
    fn from(value: &XamlSourceFocusNavigationRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationRequest {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationRequest {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlSourceFocusNavigationResult(pub ::windows::core::IInspectable);
impl XamlSourceFocusNavigationResult {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn WasFocusMoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn CreateInstance(focusmoved: bool) -> ::windows::core::Result<XamlSourceFocusNavigationResult> {
        Self::IXamlSourceFocusNavigationResultFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focusmoved, &mut result__).from_abi::<XamlSourceFocusNavigationResult>(result__)
        })
    }
    pub fn IXamlSourceFocusNavigationResultFactory<R, F: FnOnce(&IXamlSourceFocusNavigationResultFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlSourceFocusNavigationResult, IXamlSourceFocusNavigationResultFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult;{88d55a5f-9603-5d8f-9cc7-d1c4070d9801})");
}
unsafe impl ::windows::core::Interface for XamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d55a5f_9603_5d8f_9cc7_d1c4070d9801);
}
impl ::windows::core::RuntimeName for XamlSourceFocusNavigationResult {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult";
}
impl ::core::convert::From<XamlSourceFocusNavigationResult> for ::windows::core::IUnknown {
    fn from(value: XamlSourceFocusNavigationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationResult> for ::windows::core::IUnknown {
    fn from(value: &XamlSourceFocusNavigationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlSourceFocusNavigationResult> for ::windows::core::IInspectable {
    fn from(value: XamlSourceFocusNavigationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationResult> for ::windows::core::IInspectable {
    fn from(value: &XamlSourceFocusNavigationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationResult {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationResult {}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlUIPresenter(pub ::windows::core::IInspectable);
impl XamlUIPresenter {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn RootElement(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetRootElement<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn ThemeKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetThemeKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn ThemeResourcesXaml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetThemeResourcesXaml<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetSize(&self, width: i32, height: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), width, height).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn Render(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn Present(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn CompleteTimelinesAutomatically() -> ::windows::core::Result<bool> {
        Self::IXamlUIPresenterStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetCompleteTimelinesAutomatically(value: bool) -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetHost<'a, Param0: ::windows::core::IntoParam<'a, IXamlUIPresenterHost>>(host: Param0) -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), host.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn NotifyWindowSizeChanged() -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`, `UI_Xaml_Controls_Primitives`*"]
    pub fn GetFlyoutPlacementTargetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::FrameworkElement>>(placementtarget: Param0, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: &mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: &mut bool) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        Self::IXamlUIPresenterStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), placementtarget.into_param().abi(), preferredplacement, targetpreferredplacement, allowfallbacks, &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `Foundation`, `UI_Xaml_Controls_Primitives`*"]
    pub fn GetFlyoutPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(
        placementtargetbounds: Param0,
        controlsize: Param1,
        mincontrolsize: Param2,
        containerrect: Param3,
        targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode,
        allowfallbacks: bool,
        chosenplacement: &mut super::Controls::Primitives::FlyoutPlacementMode,
    ) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        Self::IXamlUIPresenterStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), placementtargetbounds.into_param().abi(), controlsize.into_param().abi(), mincontrolsize.into_param().abi(), containerrect.into_param().abi(), targetpreferredplacement, allowfallbacks, chosenplacement, &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        })
    }
    pub fn IXamlUIPresenterStatics<R, F: FnOnce(&IXamlUIPresenterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUIPresenter, IXamlUIPresenterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlUIPresenterStatics2<R, F: FnOnce(&IXamlUIPresenterStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUIPresenter, IXamlUIPresenterStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlUIPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlUIPresenter;{a714944a-1619-4fc6-b31b-89512ef022a2})");
}
unsafe impl ::windows::core::Interface for XamlUIPresenter {
    type Vtable = IXamlUIPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa714944a_1619_4fc6_b31b_89512ef022a2);
}
impl ::windows::core::RuntimeName for XamlUIPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlUIPresenter";
}
impl ::core::convert::From<XamlUIPresenter> for ::windows::core::IUnknown {
    fn from(value: XamlUIPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlUIPresenter> for ::windows::core::IUnknown {
    fn from(value: &XamlUIPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlUIPresenter> for ::windows::core::IInspectable {
    fn from(value: XamlUIPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlUIPresenter> for ::windows::core::IInspectable {
    fn from(value: &XamlUIPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlUIPresenter {}
unsafe impl ::core::marker::Sync for XamlUIPresenter {}
