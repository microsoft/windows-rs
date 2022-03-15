#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct DesignerAppExitedEventArgs(::windows::core::IUnknown);
impl DesignerAppExitedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for DesignerAppExitedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesignerAppExitedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesignerAppExitedEventArgs {}
impl ::core::fmt::Debug for DesignerAppExitedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppExitedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesignerAppExitedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs;{f6aac86a-0cad-410c-8f62-dc2936151c74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDesignerAppExitedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesignerAppExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs";
}
impl ::core::convert::From<DesignerAppExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DesignerAppExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DesignerAppExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesignerAppExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DesignerAppExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DesignerAppExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesignerAppExitedEventArgs {}
unsafe impl ::core::marker::Sync for DesignerAppExitedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct DesignerAppManager(::windows::core::IUnknown);
impl DesignerAppManager {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesignerAppExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesignerAppExited)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDesignerAppExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDesignerAppExited)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateNewViewAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, initialviewstate: DesignerAppViewState, initialviewsize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DesignerAppView>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateNewViewAsync)(::core::mem::transmute_copy(this), initialviewstate, initialviewsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DesignerAppView>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadObjectIntoAppAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, dllname: Param0, classid: Param1, initializationdata: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadObjectIntoAppAsync)(::core::mem::transmute_copy(this), dllname.into_param().abi(), classid.into_param().abi(), initializationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appusermodelid: Param0) -> ::windows::core::Result<DesignerAppManager> {
        Self::IDesignerAppManagerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), appusermodelid.into_param().abi(), &mut result__).from_abi::<DesignerAppManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDesignerAppManagerFactory<R, F: FnOnce(&IDesignerAppManagerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DesignerAppManager, IDesignerAppManagerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DesignerAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesignerAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesignerAppManager {}
impl ::core::fmt::Debug for DesignerAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesignerAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppManager;{a6272caa-d5c6-40cb-abd9-27ba43831bb7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesignerAppManager {
    type Vtable = IDesignerAppManager_Vtbl;
    const IID: ::windows::core::GUID = <IDesignerAppManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesignerAppManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppManager";
}
impl ::core::convert::From<DesignerAppManager> for ::windows::core::IUnknown {
    fn from(value: DesignerAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppManager> for ::windows::core::IUnknown {
    fn from(value: &DesignerAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesignerAppManager> for ::windows::core::IInspectable {
    fn from(value: DesignerAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppManager> for ::windows::core::IInspectable {
    fn from(value: &DesignerAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesignerAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct DesignerAppView(::windows::core::IUnknown);
impl DesignerAppView {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn ApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationViewId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn ViewState(&self) -> ::windows::core::Result<DesignerAppViewState> {
        let this = self;
        unsafe {
            let mut result__: DesignerAppViewState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DesignerAppViewState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ViewSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateViewAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, viewstate: DesignerAppViewState, viewsize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateViewAsync)(::core::mem::transmute_copy(this), viewstate, viewsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for DesignerAppView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesignerAppView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesignerAppView {}
impl ::core::fmt::Debug for DesignerAppView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesignerAppView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppView;{5c777cea-dd71-4a84-a56f-dacb4b14706f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesignerAppView {
    type Vtable = IDesignerAppView_Vtbl;
    const IID: ::windows::core::GUID = <IDesignerAppView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesignerAppView {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppView";
}
impl ::core::convert::From<DesignerAppView> for ::windows::core::IUnknown {
    fn from(value: DesignerAppView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppView> for ::windows::core::IUnknown {
    fn from(value: &DesignerAppView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesignerAppView> for ::windows::core::IInspectable {
    fn from(value: DesignerAppView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppView> for ::windows::core::IInspectable {
    fn from(value: &DesignerAppView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesignerAppView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DesignerAppViewState(pub i32);
impl DesignerAppViewState {
    pub const Visible: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for DesignerAppViewState {}
impl ::core::clone::Clone for DesignerAppViewState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DesignerAppViewState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DesignerAppViewState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DesignerAppViewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppViewState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesignerAppViewState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.DesignerAppViewState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct DesktopWindowXamlSource(::windows::core::IUnknown);
impl DesktopWindowXamlSource {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn Content(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContent)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn HasFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasFocus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TakeFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TakeFocusRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTakeFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTakeFocusRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GotFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGotFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn NavigateFocus<'a, Param0: ::windows::core::IntoParam<'a, XamlSourceFocusNavigationRequest>>(&self, request: Param0) -> ::windows::core::Result<XamlSourceFocusNavigationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateFocus)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn new() -> ::windows::core::Result<DesktopWindowXamlSource> {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<DesktopWindowXamlSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<DesktopWindowXamlSource> {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<DesktopWindowXamlSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDesktopWindowXamlSourceFactory<R, F: FnOnce(&IDesktopWindowXamlSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DesktopWindowXamlSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowXamlSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowXamlSource {}
impl ::core::fmt::Debug for DesktopWindowXamlSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowXamlSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSource;{d585bfe1-00ff-51be-ba1d-a1329956ea0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
    const IID: ::windows::core::GUID = <IDesktopWindowXamlSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopWindowXamlSource {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource";
}
impl ::core::convert::From<DesktopWindowXamlSource> for ::windows::core::IUnknown {
    fn from(value: DesktopWindowXamlSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSource> for ::windows::core::IUnknown {
    fn from(value: &DesktopWindowXamlSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopWindowXamlSource> for ::windows::core::IInspectable {
    fn from(value: DesktopWindowXamlSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSource> for ::windows::core::IInspectable {
    fn from(value: &DesktopWindowXamlSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesktopWindowXamlSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(::windows::core::IUnknown);
impl DesktopWindowXamlSourceGotFocusEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for DesktopWindowXamlSourceGotFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowXamlSourceGotFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowXamlSourceGotFocusEventArgs {}
impl ::core::fmt::Debug for DesktopWindowXamlSourceGotFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowXamlSourceGotFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSourceGotFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs;{39be4849-d9cc-5b70-8f05-1ad9a4aaa342})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDesktopWindowXamlSourceGotFocusEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopWindowXamlSourceGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs";
}
impl ::core::convert::From<DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceGotFocusEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows::core::IUnknown);
impl DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
impl ::core::fmt::Debug for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowXamlSourceTakeFocusRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs;{fe61e4b9-a7af-52b3-bdb9-c3305c0b8df2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDesktopWindowXamlSourceTakeFocusRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs";
}
impl ::core::convert::From<DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct ElementCompositionPreview(::windows::core::IUnknown);
impl ElementCompositionPreview {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetElementVisual)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetElementChildVisual)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::Visual>>(element: Param0, visual: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetElementChildVisual)(::core::mem::transmute_copy(this), element.into_param().abi(), visual.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`, `\"UI_Xaml_Controls\"`*"]
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub fn GetScrollViewerManipulationPropertySet<'a, Param0: ::windows::core::IntoParam<'a, super::Controls::ScrollViewer>>(scrollviewer: Param0) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScrollViewerManipulationPropertySet)(::core::mem::transmute_copy(this), scrollviewer.into_param().abi(), &mut result__).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitShowAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetImplicitShowAnimation)(::core::mem::transmute_copy(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitHideAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetImplicitHideAnimation)(::core::mem::transmute_copy(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetIsTranslationEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetIsTranslationEnabled)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetPointerPositionPropertySet<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(targetelement: Param0) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPointerPositionPropertySet)(::core::mem::transmute_copy(this), targetelement.into_param().abi(), &mut result__).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn SetAppWindowContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(appwindow: Param0, xamlcontent: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).SetAppWindowContent)(::core::mem::transmute_copy(this), appwindow.into_param().abi(), xamlcontent.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn GetAppWindowContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(appwindow: Param0) -> ::windows::core::Result<super::UIElement> {
        Self::IElementCompositionPreviewStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppWindowContent)(::core::mem::transmute_copy(this), appwindow.into_param().abi(), &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IElementCompositionPreviewStatics<R, F: FnOnce(&IElementCompositionPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IElementCompositionPreviewStatics2<R, F: FnOnce(&IElementCompositionPreviewStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IElementCompositionPreviewStatics3<R, F: FnOnce(&IElementCompositionPreviewStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ElementCompositionPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ElementCompositionPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ElementCompositionPreview {}
impl ::core::fmt::Debug for ElementCompositionPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementCompositionPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.ElementCompositionPreview;{b6f1a676-cfe6-46ac-acf6-c4687bb65e60})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
    const IID: ::windows::core::GUID = <IElementCompositionPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.ElementCompositionPreview";
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: ElementCompositionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: &ElementCompositionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: ElementCompositionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: &ElementCompositionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppExitedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6aac86a_0cad_410c_8f62_dc2936151c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppExitedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignerAppManager {
    type Vtable = IDesignerAppManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6272caa_d5c6_40cb_abd9_27ba43831bb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DesignerAppExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesignerAppExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDesignerAppExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDesignerAppExited: usize,
    #[cfg(feature = "Foundation")]
    pub CreateNewViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialviewstate: DesignerAppViewState, initialviewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateNewViewAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadObjectIntoAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dllname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, classid: ::windows::core::GUID, initializationdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadObjectIntoAppAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppManagerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignerAppManagerFactory {
    type Vtable = IDesignerAppManagerFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f9d633b_1266_4c0e_8499_0db85bbd4c43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManagerFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignerAppView {
    type Vtable = IDesignerAppView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c777cea_dd71_4a84_a56f_dacb4b14706f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ViewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DesignerAppViewState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewSize: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewstate: DesignerAppViewState, viewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateViewAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd585bfe1_00ff_51be_ba1d_a1329956ea0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub HasFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TakeFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TakeFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTakeFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTakeFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    pub NavigateFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceFactory {
    type Vtable = IDesktopWindowXamlSourceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cd61dc0_2561_56e1_8e75_6e44173805e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39be4849_d9cc_5b70_8f05_1ad9a4aaa342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe61e4b9_a7af_52b3_bdb9_c3305c0b8df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6f1a676_cfe6_46ac_acf6_c4687bb65e60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c92b38_ec99_4c55_bc85_a1c180b27646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetElementVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetElementChildVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementChildVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetElementChildVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetElementChildVisual: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub GetScrollViewerManipulationPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollviewer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Controls")))]
    GetScrollViewerManipulationPropertySet: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics2 {
    type Vtable = IElementCompositionPreviewStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24148fbb_23d6_4f37_ba0c_0733e799722d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitShowAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitShowAnimation: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitHideAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitHideAnimation: usize,
    pub SetIsTranslationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPointerPositionPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPointerPositionPropertySet: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics3 {
    type Vtable = IElementCompositionPreviewStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x843bc4c3_c105_59fe_a3d1_373c1d3e6fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub SetAppWindowContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::windows::core::RawPtr, xamlcontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    SetAppWindowContent: usize,
    #[cfg(feature = "UI_WindowManagement")]
    pub GetAppWindowContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    GetAppWindowContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsXamlManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56096c31_1aa0_5288_8818_6e74a2dcaff5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsXamlManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowsXamlManagerStatics {
    type Vtable = IWindowsXamlManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28258a12_7d82_505b_b210_712b04a58882);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub InitializeForCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbb93bb5_1496_5a80_ac00_e757359755e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XamlSourceFocusNavigationReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HintRect: usize,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationRequestFactory {
    type Vtable = IXamlSourceFocusNavigationRequestFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe746ab8f_b4ef_5390_97e5_cc0a2779c574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithHintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithHintRect: usize,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithHintRectAndCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, correlationid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithHintRectAndCorrelationId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d55a5f_9603_5d8f_9cc7_d1c4070d9801);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WasFocusMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResultFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationResultFactory {
    type Vtable = IXamlSourceFocusNavigationResultFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bbadbf_f9e1_5527_b8c5_09339ff2ca76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResultFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusmoved: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUIPresenter {
    type Vtable = IXamlUIPresenter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa714944a_1619_4fc6_b31b_89512ef022a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RootElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRootElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ThemeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetThemeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ThemeResourcesXaml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetThemeResourcesXaml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32) -> ::windows::core::HRESULT,
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IXamlUIPresenterHost(::windows::core::IUnknown);
impl IXamlUIPresenterHost {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn ResolveFileResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, path: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResolveFileResource)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IXamlUIPresenterHost> for ::windows::core::IUnknown {
    fn from(value: IXamlUIPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost> for ::windows::core::IUnknown {
    fn from(value: &IXamlUIPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlUIPresenterHost> for ::windows::core::IInspectable {
    fn from(value: IXamlUIPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost> for ::windows::core::IInspectable {
    fn from(value: &IXamlUIPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlUIPresenterHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlUIPresenterHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlUIPresenterHost {}
impl ::core::fmt::Debug for IXamlUIPresenterHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlUIPresenterHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{aafb84cd-9f6d-4f80-ac2c-0e6cb9f31659}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost {
    type Vtable = IXamlUIPresenterHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafb84cd_9f6d_4f80_ac2c_0e6cb9f31659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ResolveFileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IXamlUIPresenterHost2(::windows::core::IUnknown);
impl IXamlUIPresenterHost2 {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn GetGenericXamlFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGenericXamlFilePath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IXamlUIPresenterHost2> for ::windows::core::IUnknown {
    fn from(value: IXamlUIPresenterHost2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost2> for ::windows::core::IUnknown {
    fn from(value: &IXamlUIPresenterHost2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlUIPresenterHost2> for ::windows::core::IInspectable {
    fn from(value: IXamlUIPresenterHost2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost2> for ::windows::core::IInspectable {
    fn from(value: &IXamlUIPresenterHost2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlUIPresenterHost2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlUIPresenterHost2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlUIPresenterHost2 {}
impl ::core::fmt::Debug for IXamlUIPresenterHost2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlUIPresenterHost2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61595672-7ca4-4a21-b56a-88f4812388ca}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost2 {
    type Vtable = IXamlUIPresenterHost2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61595672_7ca4_4a21_b56a_88f4812388ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetGenericXamlFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IXamlUIPresenterHost3(::windows::core::IUnknown);
impl IXamlUIPresenterHost3 {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn ResolveDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, super::ResourceDictionary>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, dictionary: Param0, dictionarykey: Param1, suggestedvalue: Param2) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResolveDictionaryResource)(::core::mem::transmute_copy(this), dictionary.into_param().abi(), dictionarykey.into_param().abi(), suggestedvalue.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IXamlUIPresenterHost3> for ::windows::core::IUnknown {
    fn from(value: IXamlUIPresenterHost3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost3> for ::windows::core::IUnknown {
    fn from(value: &IXamlUIPresenterHost3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlUIPresenterHost3> for ::windows::core::IInspectable {
    fn from(value: IXamlUIPresenterHost3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost3> for ::windows::core::IInspectable {
    fn from(value: &IXamlUIPresenterHost3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlUIPresenterHost3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlUIPresenterHost3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlUIPresenterHost3 {}
impl ::core::fmt::Debug for IXamlUIPresenterHost3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlUIPresenterHost3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost3 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b14292bf-7320-41bb-9f26-4d6fd34db45a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost3 {
    type Vtable = IXamlUIPresenterHost3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14292bf_7320_41bb_9f26_4d6fd34db45a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ResolveDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, dictionarykey: *mut ::core::ffi::c_void, suggestedvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUIPresenterStatics {
    type Vtable = IXamlUIPresenterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71eaeac8_45e1_4192_85aa_3a422edd23cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CompleteTimelinesAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCompleteTimelinesAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyWindowSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenterStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUIPresenterStatics2 {
    type Vtable = IXamlUIPresenterStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c6b68d2_cf1c_4f53_bf09_6a745f7a9703);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub GetFlyoutPlacementTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: *mut bool, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))]
    GetFlyoutPlacementTargetInfo: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub GetFlyoutPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtargetbounds: super::super::super::Foundation::Rect, controlsize: super::super::super::Foundation::Size, mincontrolsize: super::super::super::Foundation::Size, containerrect: super::super::super::Foundation::Rect, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))]
    GetFlyoutPlacement: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct WindowsXamlManager(::windows::core::IUnknown);
impl WindowsXamlManager {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn InitializeForCurrentThread() -> ::windows::core::Result<WindowsXamlManager> {
        Self::IWindowsXamlManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InitializeForCurrentThread)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WindowsXamlManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowsXamlManagerStatics<R, F: FnOnce(&IWindowsXamlManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowsXamlManager, IWindowsXamlManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowsXamlManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsXamlManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsXamlManager {}
impl ::core::fmt::Debug for WindowsXamlManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsXamlManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsXamlManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.WindowsXamlManager;{56096c31-1aa0-5288-8818-6e74a2dcaff5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
    const IID: ::windows::core::GUID = <IWindowsXamlManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsXamlManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.WindowsXamlManager";
}
impl ::core::convert::From<WindowsXamlManager> for ::windows::core::IUnknown {
    fn from(value: WindowsXamlManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowsXamlManager> for ::windows::core::IUnknown {
    fn from(value: &WindowsXamlManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowsXamlManager> for ::windows::core::IInspectable {
    fn from(value: WindowsXamlManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowsXamlManager> for ::windows::core::IInspectable {
    fn from(value: &WindowsXamlManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WindowsXamlManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XamlSourceFocusNavigationReason(pub i32);
impl XamlSourceFocusNavigationReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Restore: Self = Self(1i32);
    pub const First: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
    pub const Left: Self = Self(7i32);
    pub const Up: Self = Self(8i32);
    pub const Right: Self = Self(9i32);
    pub const Down: Self = Self(10i32);
}
impl ::core::marker::Copy for XamlSourceFocusNavigationReason {}
impl ::core::clone::Clone for XamlSourceFocusNavigationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XamlSourceFocusNavigationReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XamlSourceFocusNavigationReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for XamlSourceFocusNavigationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct XamlSourceFocusNavigationRequest(::windows::core::IUnknown);
impl XamlSourceFocusNavigationRequest {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<XamlSourceFocusNavigationReason> {
        let this = self;
        unsafe {
            let mut result__: XamlSourceFocusNavigationReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationReason>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HintRect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn CreateInstance(reason: XamlSourceFocusNavigationReason) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceWithHintRect<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(reason: XamlSourceFocusNavigationReason, hintrect: Param1) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithHintRect)(::core::mem::transmute_copy(this), reason, hintrect.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceWithHintRectAndCorrelationId<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(reason: XamlSourceFocusNavigationReason, hintrect: Param1, correlationid: Param2) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithHintRectAndCorrelationId)(::core::mem::transmute_copy(this), reason, hintrect.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlSourceFocusNavigationRequestFactory<R, F: FnOnce(&IXamlSourceFocusNavigationRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlSourceFocusNavigationRequest, IXamlSourceFocusNavigationRequestFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlSourceFocusNavigationRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlSourceFocusNavigationRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlSourceFocusNavigationRequest {}
impl ::core::fmt::Debug for XamlSourceFocusNavigationRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest;{fbb93bb5-1496-5a80-ac00-e757359755e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
    const IID: ::windows::core::GUID = <IXamlSourceFocusNavigationRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlSourceFocusNavigationRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest";
}
impl ::core::convert::From<XamlSourceFocusNavigationRequest> for ::windows::core::IUnknown {
    fn from(value: XamlSourceFocusNavigationRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationRequest> for ::windows::core::IUnknown {
    fn from(value: &XamlSourceFocusNavigationRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlSourceFocusNavigationRequest> for ::windows::core::IInspectable {
    fn from(value: XamlSourceFocusNavigationRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationRequest> for ::windows::core::IInspectable {
    fn from(value: &XamlSourceFocusNavigationRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationRequest {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationRequest {}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct XamlSourceFocusNavigationResult(::windows::core::IUnknown);
impl XamlSourceFocusNavigationResult {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn WasFocusMoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasFocusMoved)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn CreateInstance(focusmoved: bool) -> ::windows::core::Result<XamlSourceFocusNavigationResult> {
        Self::IXamlSourceFocusNavigationResultFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), focusmoved, &mut result__).from_abi::<XamlSourceFocusNavigationResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlSourceFocusNavigationResultFactory<R, F: FnOnce(&IXamlSourceFocusNavigationResultFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlSourceFocusNavigationResult, IXamlSourceFocusNavigationResultFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlSourceFocusNavigationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlSourceFocusNavigationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlSourceFocusNavigationResult {}
impl ::core::fmt::Debug for XamlSourceFocusNavigationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult;{88d55a5f-9603-5d8f-9cc7-d1c4070d9801})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
    const IID: ::windows::core::GUID = <IXamlSourceFocusNavigationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlSourceFocusNavigationResult {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult";
}
impl ::core::convert::From<XamlSourceFocusNavigationResult> for ::windows::core::IUnknown {
    fn from(value: XamlSourceFocusNavigationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationResult> for ::windows::core::IUnknown {
    fn from(value: &XamlSourceFocusNavigationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlSourceFocusNavigationResult> for ::windows::core::IInspectable {
    fn from(value: XamlSourceFocusNavigationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationResult> for ::windows::core::IInspectable {
    fn from(value: &XamlSourceFocusNavigationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationResult {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationResult {}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct XamlUIPresenter(::windows::core::IUnknown);
impl XamlUIPresenter {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn RootElement(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RootElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetRootElement<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRootElement)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn ThemeKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThemeKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetThemeKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThemeKey)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn ThemeResourcesXaml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThemeResourcesXaml)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetThemeResourcesXaml<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThemeResourcesXaml)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetSize(&self, width: i32, height: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSize)(::core::mem::transmute_copy(this), width, height).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn Render(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Render)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn Present(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Present)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn CompleteTimelinesAutomatically() -> ::windows::core::Result<bool> {
        Self::IXamlUIPresenterStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompleteTimelinesAutomatically)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetCompleteTimelinesAutomatically(value: bool) -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetCompleteTimelinesAutomatically)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetHost<'a, Param0: ::windows::core::IntoParam<'a, IXamlUIPresenterHost>>(host: Param0) -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetHost)(::core::mem::transmute_copy(this), host.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn NotifyWindowSizeChanged() -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).NotifyWindowSizeChanged)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`, `\"UI_Xaml_Controls_Primitives\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub fn GetFlyoutPlacementTargetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::FrameworkElement>>(placementtarget: Param0, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: &mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: &mut bool) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        Self::IXamlUIPresenterStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFlyoutPlacementTargetInfo)(::core::mem::transmute_copy(this), placementtarget.into_param().abi(), preferredplacement, targetpreferredplacement, allowfallbacks, &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"Foundation\"`, `\"UI_Xaml_Controls_Primitives\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub fn GetFlyoutPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(placementtargetbounds: Param0, controlsize: Param1, mincontrolsize: Param2, containerrect: Param3, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: &mut super::Controls::Primitives::FlyoutPlacementMode) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        Self::IXamlUIPresenterStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFlyoutPlacement)(::core::mem::transmute_copy(this), placementtargetbounds.into_param().abi(), controlsize.into_param().abi(), mincontrolsize.into_param().abi(), containerrect.into_param().abi(), targetpreferredplacement, allowfallbacks, chosenplacement, &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlUIPresenterStatics<R, F: FnOnce(&IXamlUIPresenterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUIPresenter, IXamlUIPresenterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IXamlUIPresenterStatics2<R, F: FnOnce(&IXamlUIPresenterStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUIPresenter, IXamlUIPresenterStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlUIPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlUIPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlUIPresenter {}
impl ::core::fmt::Debug for XamlUIPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlUIPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlUIPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlUIPresenter;{a714944a-1619-4fc6-b31b-89512ef022a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlUIPresenter {
    type Vtable = IXamlUIPresenter_Vtbl;
    const IID: ::windows::core::GUID = <IXamlUIPresenter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlUIPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlUIPresenter";
}
impl ::core::convert::From<XamlUIPresenter> for ::windows::core::IUnknown {
    fn from(value: XamlUIPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUIPresenter> for ::windows::core::IUnknown {
    fn from(value: &XamlUIPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlUIPresenter> for ::windows::core::IInspectable {
    fn from(value: XamlUIPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUIPresenter> for ::windows::core::IInspectable {
    fn from(value: &XamlUIPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlUIPresenter {}
unsafe impl ::core::marker::Sync for XamlUIPresenter {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
