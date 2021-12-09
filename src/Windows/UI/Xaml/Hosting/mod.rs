#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct DesignerAppExitedEventArgs(::windows::core::IUnknown);
impl DesignerAppExitedEventArgs {
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
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
unsafe impl ::windows::core::RuntimeType for DesignerAppExitedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs;{f6aac86a-0cad-410c-8f62-dc2936151c74})");
}
unsafe impl ::windows::core::Interface for DesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6aac86a_0cad_410c_8f62_dc2936151c74);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DesignerAppExitedEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesignerAppExitedEventArgs {}
unsafe impl ::core::marker::Sync for DesignerAppExitedEventArgs {}
#[repr(transparent)]
pub struct DesignerAppManager(::windows::core::IUnknown);
impl DesignerAppManager {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DesignerAppExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDesignerAppExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateNewViewAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, initialviewstate: DesignerAppViewState, initialviewsize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DesignerAppView>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), initialviewstate, initialviewsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DesignerAppView>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoadObjectIntoAppAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, dllname: Param0, classid: Param1, initializationdata: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dllname.into_param().abi(), classid.into_param().abi(), initializationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
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
unsafe impl ::windows::core::RuntimeType for DesignerAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppManager;{a6272caa-d5c6-40cb-abd9-27ba43831bb7})");
}
unsafe impl ::windows::core::Interface for DesignerAppManager {
    type Vtable = IDesignerAppManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6272caa_d5c6_40cb_abd9_27ba43831bb7);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DesignerAppManager {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DesignerAppManager {
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
#[repr(transparent)]
pub struct DesignerAppView(::windows::core::IUnknown);
impl DesignerAppView {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ViewState(&self) -> ::windows::core::Result<DesignerAppViewState> {
        let this = self;
        unsafe {
            let mut result__: DesignerAppViewState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DesignerAppViewState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ViewSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateViewAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, viewstate: DesignerAppViewState, viewsize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), viewstate, viewsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
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
unsafe impl ::windows::core::RuntimeType for DesignerAppView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppView;{5c777cea-dd71-4a84-a56f-dacb4b14706f})");
}
unsafe impl ::windows::core::Interface for DesignerAppView {
    type Vtable = IDesignerAppViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c777cea_dd71_4a84_a56f_dacb4b14706f);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DesignerAppView {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DesignerAppView {
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
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for DesignerAppViewState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DesignerAppViewState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesignerAppViewState {}
unsafe impl ::windows::core::RuntimeType for DesignerAppViewState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.DesignerAppViewState;i4)");
}
impl ::windows::core::DefaultType for DesignerAppViewState {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct DesktopWindowXamlSource(::windows::core::IUnknown);
impl DesktopWindowXamlSource {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Content(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn HasFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TakeFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTakeFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn NavigateFocus<'a, Param0: ::windows::core::IntoParam<'a, XamlSourceFocusNavigationRequest>>(&self, request: Param0) -> ::windows::core::Result<XamlSourceFocusNavigationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationResult>(result__)
        }
    }
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
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSource;{d585bfe1-00ff-51be-ba1d-a1329956ea0a})");
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd585bfe1_00ff_51be_ba1d_a1329956ea0a);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DesktopWindowXamlSource {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DesktopWindowXamlSource {
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
#[repr(transparent)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(::windows::core::IUnknown);
impl DesktopWindowXamlSourceGotFocusEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
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
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSourceGotFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs;{39be4849-d9cc-5b70-8f05-1ad9a4aaa342})");
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39be4849_d9cc_5b70_8f05_1ad9a4aaa342);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DesktopWindowXamlSourceGotFocusEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceGotFocusEventArgs {}
#[repr(transparent)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows::core::IUnknown);
impl DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
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
unsafe impl ::windows::core::RuntimeType for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs;{fe61e4b9-a7af-52b3-bdb9-c3305c0b8df2})");
}
unsafe impl ::windows::core::Interface for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe61e4b9_a7af_52b3_bdb9_c3305c0b8df2);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
#[repr(transparent)]
pub struct ElementCompositionPreview(::windows::core::IUnknown);
impl ElementCompositionPreview {
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::Visual>>(element: Param0, visual: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), visual.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub fn GetScrollViewerManipulationPropertySet<'a, Param0: ::windows::core::IntoParam<'a, super::Controls::ScrollViewer>>(scrollviewer: Param0) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), scrollviewer.into_param().abi(), &mut result__).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitShowAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitHideAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    pub fn SetIsTranslationEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetPointerPositionPropertySet<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(targetelement: Param0) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), targetelement.into_param().abi(), &mut result__).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(feature = "UI_WindowManagement")]
    pub fn SetAppWindowContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(appwindow: Param0, xamlcontent: Param1) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appwindow.into_param().abi(), xamlcontent.into_param().abi()).ok() })
    }
    #[cfg(feature = "UI_WindowManagement")]
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
unsafe impl ::windows::core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.ElementCompositionPreview;{b6f1a676-cfe6-46ac-acf6-c4687bb65e60})");
}
unsafe impl ::windows::core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreviewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6f1a676_cfe6_46ac_acf6_c4687bb65e60);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ElementCompositionPreview {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ElementCompositionPreview {
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
    type Vtable = IDesignerAppExitedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6aac86a_0cad_410c_8f62_dc2936151c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppExitedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignerAppManager {
    type Vtable = IDesignerAppManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6272caa_d5c6_40cb_abd9_27ba43831bb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialviewstate: DesignerAppViewState, initialviewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dllname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, classid: ::windows::core::GUID, initializationdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppManagerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignerAppManagerFactory {
    type Vtable = IDesignerAppManagerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f9d633b_1266_4c0e_8499_0db85bbd4c43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManagerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignerAppView {
    type Vtable = IDesignerAppViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c777cea_dd71_4a84_a56f_dacb4b14706f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DesignerAppViewState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewstate: DesignerAppViewState, viewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd585bfe1_00ff_51be_ba1d_a1329956ea0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceFactory {
    type Vtable = IDesktopWindowXamlSourceFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cd61dc0_2561_56e1_8e75_6e44173805e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39be4849_d9cc_5b70_8f05_1ad9a4aaa342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgsVtbl(
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
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe61e4b9_a7af_52b3_bdb9_c3305c0b8df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl(
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
pub struct IElementCompositionPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreviewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6f1a676_cfe6_46ac_acf6_c4687bb65e60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c92b38_ec99_4c55_bc85_a1c180b27646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollviewer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Controls")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics2 {
    type Vtable = IElementCompositionPreviewStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24148fbb_23d6_4f37_ba0c_0733e799722d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics3 {
    type Vtable = IElementCompositionPreviewStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x843bc4c3_c105_59fe_a3d1_373c1d3e6fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::windows::core::RawPtr, xamlcontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsXamlManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowsXamlManager {
    type Vtable = IWindowsXamlManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56096c31_1aa0_5288_8818_6e74a2dcaff5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsXamlManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowsXamlManagerStatics {
    type Vtable = IWindowsXamlManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28258a12_7d82_505b_b210_712b04a58882);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManagerStaticsVtbl(
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
pub struct IXamlSourceFocusNavigationRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbb93bb5_1496_5a80_ac00_e757359755e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XamlSourceFocusNavigationReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationRequestFactory {
    type Vtable = IXamlSourceFocusNavigationRequestFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe746ab8f_b4ef_5390_97e5_cc0a2779c574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, correlationid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d55a5f_9603_5d8f_9cc7_d1c4070d9801);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResultVtbl(
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
pub struct IXamlSourceFocusNavigationResultFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlSourceFocusNavigationResultFactory {
    type Vtable = IXamlSourceFocusNavigationResultFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bbadbf_f9e1_5527_b8c5_09339ff2ca76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResultFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusmoved: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUIPresenter {
    type Vtable = IXamlUIPresenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa714944a_1619_4fc6_b31b_89512ef022a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXamlUIPresenterHost(::windows::core::IUnknown);
impl IXamlUIPresenterHost {
    pub fn ResolveFileResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, path: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXamlUIPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{aafb84cd-9f6d-4f80-ac2c-0e6cb9f31659}");
}
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost {
    type Vtable = IXamlUIPresenterHostVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafb84cd_9f6d_4f80_ac2c_0e6cb9f31659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHostVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXamlUIPresenterHost2(::windows::core::IUnknown);
impl IXamlUIPresenterHost2 {
    pub fn GetGenericXamlFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61595672-7ca4-4a21-b56a-88f4812388ca}");
}
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost2 {
    type Vtable = IXamlUIPresenterHost2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61595672_7ca4_4a21_b56a_88f4812388ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXamlUIPresenterHost3(::windows::core::IUnknown);
impl IXamlUIPresenterHost3 {
    pub fn ResolveDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, super::ResourceDictionary>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, dictionary: Param0, dictionarykey: Param1, suggestedvalue: Param2) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dictionary.into_param().abi(), dictionarykey.into_param().abi(), suggestedvalue.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::RuntimeType for IXamlUIPresenterHost3 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b14292bf-7320-41bb-9f26-4d6fd34db45a}");
}
unsafe impl ::windows::core::Interface for IXamlUIPresenterHost3 {
    type Vtable = IXamlUIPresenterHost3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14292bf_7320_41bb_9f26_4d6fd34db45a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, dictionarykey: *mut ::core::ffi::c_void, suggestedvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUIPresenterStatics {
    type Vtable = IXamlUIPresenterStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71eaeac8_45e1_4192_85aa_3a422edd23cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenterStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUIPresenterStatics2 {
    type Vtable = IXamlUIPresenterStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c6b68d2_cf1c_4f53_bf09_6a745f7a9703);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: *mut bool, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtargetbounds: super::super::super::Foundation::Rect, controlsize: super::super::super::Foundation::Size, mincontrolsize: super::super::super::Foundation::Size, containerrect: super::super::super::Foundation::Rect, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))] usize,
);
#[repr(transparent)]
pub struct WindowsXamlManager(::windows::core::IUnknown);
impl WindowsXamlManager {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
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
unsafe impl ::windows::core::RuntimeType for WindowsXamlManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.WindowsXamlManager;{56096c31-1aa0-5288-8818-6e74a2dcaff5})");
}
unsafe impl ::windows::core::Interface for WindowsXamlManager {
    type Vtable = IWindowsXamlManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56096c31_1aa0_5288_8818_6e74a2dcaff5);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WindowsXamlManager {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WindowsXamlManager {
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
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for XamlSourceFocusNavigationReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XamlSourceFocusNavigationReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlSourceFocusNavigationReason {}
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationReason;i4)");
}
impl ::windows::core::DefaultType for XamlSourceFocusNavigationReason {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct XamlSourceFocusNavigationRequest(::windows::core::IUnknown);
impl XamlSourceFocusNavigationRequest {
    pub fn Reason(&self) -> ::windows::core::Result<XamlSourceFocusNavigationReason> {
        let this = self;
        unsafe {
            let mut result__: XamlSourceFocusNavigationReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlSourceFocusNavigationReason>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn CreateInstance(reason: XamlSourceFocusNavigationReason) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceWithHintRect<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(reason: XamlSourceFocusNavigationReason, hintrect: Param1) -> ::windows::core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, hintrect.into_param().abi(), &mut result__).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
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
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest;{fbb93bb5-1496-5a80-ac00-e757359755e6})");
}
unsafe impl ::windows::core::Interface for XamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbb93bb5_1496_5a80_ac00_e757359755e6);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &XamlSourceFocusNavigationRequest {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationRequest {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationRequest {}
#[repr(transparent)]
pub struct XamlSourceFocusNavigationResult(::windows::core::IUnknown);
impl XamlSourceFocusNavigationResult {
    pub fn WasFocusMoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
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
unsafe impl ::windows::core::RuntimeType for XamlSourceFocusNavigationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult;{88d55a5f-9603-5d8f-9cc7-d1c4070d9801})");
}
unsafe impl ::windows::core::Interface for XamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d55a5f_9603_5d8f_9cc7_d1c4070d9801);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &XamlSourceFocusNavigationResult {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationResult {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationResult {}
#[repr(transparent)]
pub struct XamlUIPresenter(::windows::core::IUnknown);
impl XamlUIPresenter {
    pub fn RootElement(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetRootElement<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ThemeKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetThemeKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ThemeResourcesXaml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetThemeResourcesXaml<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SetSize(&self, width: i32, height: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), width, height).ok() }
    }
    pub fn Render(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Present(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn CompleteTimelinesAutomatically() -> ::windows::core::Result<bool> {
        Self::IXamlUIPresenterStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetCompleteTimelinesAutomatically(value: bool) -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn SetHost<'a, Param0: ::windows::core::IntoParam<'a, IXamlUIPresenterHost>>(host: Param0) -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), host.into_param().abi()).ok() })
    }
    pub fn NotifyWindowSizeChanged() -> ::windows::core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub fn GetFlyoutPlacementTargetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::FrameworkElement>>(placementtarget: Param0, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: &mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: &mut bool) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        Self::IXamlUIPresenterStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), placementtarget.into_param().abi(), preferredplacement, targetpreferredplacement, allowfallbacks, &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub fn GetFlyoutPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(placementtargetbounds: Param0, controlsize: Param1, mincontrolsize: Param2, containerrect: Param3, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: &mut super::Controls::Primitives::FlyoutPlacementMode) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
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
unsafe impl ::windows::core::RuntimeType for XamlUIPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlUIPresenter;{a714944a-1619-4fc6-b31b-89512ef022a2})");
}
unsafe impl ::windows::core::Interface for XamlUIPresenter {
    type Vtable = IXamlUIPresenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa714944a_1619_4fc6_b31b_89512ef022a2);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &XamlUIPresenter {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &XamlUIPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlUIPresenter {}
unsafe impl ::core::marker::Sync for XamlUIPresenter {}
