#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct AddDeleteThemeTransition(::windows::core::IUnknown);
impl AddDeleteThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AddDeleteThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AddDeleteThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddDeleteThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddDeleteThemeTransition {}
impl ::core::fmt::Debug for AddDeleteThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddDeleteThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddDeleteThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.AddDeleteThemeTransition;{adec852e-4424-4dab-99c1-3a04e36a3c48})");
}
unsafe impl ::windows::core::Interface for AddDeleteThemeTransition {
    type Vtable = IAddDeleteThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadec852e_4424_4dab_99c1_3a04e36a3c48);
}
impl ::windows::core::RuntimeName for AddDeleteThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.AddDeleteThemeTransition";
}
impl ::core::convert::From<AddDeleteThemeTransition> for ::windows::core::IUnknown {
    fn from(value: AddDeleteThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for ::windows::core::IInspectable {
    fn from(value: AddDeleteThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for Transition {
    fn from(value: AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for Transition {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for super::super::DependencyObject {
    fn from(value: AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for super::super::DependencyObject {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AddDeleteThemeTransition {}
unsafe impl ::core::marker::Sync for AddDeleteThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct BackEase(::windows::core::IUnknown);
impl BackEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Amplitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetAmplitude(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn AmplitudeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBackEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackEaseStatics<R, F: FnOnce(&IBackEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackEase, IBackEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackEase {}
impl ::core::fmt::Debug for BackEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BackEase;{e47796e7-f805-4a8f-81c9-38e6472caa94})");
}
unsafe impl ::windows::core::Interface for BackEase {
    type Vtable = IBackEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe47796e7_f805_4a8f_81c9_38e6472caa94);
}
impl ::windows::core::RuntimeName for BackEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BackEase";
}
impl ::core::convert::From<BackEase> for ::windows::core::IUnknown {
    fn from(value: BackEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackEase> for ::windows::core::IUnknown {
    fn from(value: &BackEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackEase> for ::windows::core::IInspectable {
    fn from(value: BackEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackEase> for ::windows::core::IInspectable {
    fn from(value: &BackEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackEase> for EasingFunctionBase {
    fn from(value: BackEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BackEase> for EasingFunctionBase {
    fn from(value: &BackEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<BackEase> for super::super::DependencyObject {
    fn from(value: BackEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BackEase> for super::super::DependencyObject {
    fn from(value: &BackEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BackEase {}
unsafe impl ::core::marker::Sync for BackEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct BasicConnectedAnimationConfiguration(::windows::core::IUnknown);
impl BasicConnectedAnimationConfiguration {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn new() -> ::windows::core::Result<BasicConnectedAnimationConfiguration> {
        Self::IBasicConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<BasicConnectedAnimationConfiguration>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBasicConnectedAnimationConfigurationFactory<R, F: FnOnce(&IBasicConnectedAnimationConfigurationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BasicConnectedAnimationConfiguration, IBasicConnectedAnimationConfigurationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BasicConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BasicConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BasicConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for BasicConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BasicConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BasicConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BasicConnectedAnimationConfiguration;{e675f9b5-a4d6-5353-83e6-c89e7cf8d456})");
}
unsafe impl ::windows::core::Interface for BasicConnectedAnimationConfiguration {
    type Vtable = IBasicConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe675f9b5_a4d6_5353_83e6_c89e7cf8d456);
}
impl ::windows::core::RuntimeName for BasicConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BasicConnectedAnimationConfiguration";
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for &BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for BasicConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for BasicConnectedAnimationConfiguration {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct BeginStoryboard(::windows::core::IUnknown);
impl BeginStoryboard {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BeginStoryboard, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Storyboard(&self) -> ::windows::core::Result<Storyboard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Storyboard>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetStoryboard<'a, Param0: ::windows::core::IntoParam<'a, Storyboard>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn StoryboardProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBeginStoryboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBeginStoryboardStatics<R, F: FnOnce(&IBeginStoryboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BeginStoryboard, IBeginStoryboardStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BeginStoryboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BeginStoryboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BeginStoryboard {}
impl ::core::fmt::Debug for BeginStoryboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BeginStoryboard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BeginStoryboard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BeginStoryboard;{64189fcd-49ec-4e52-a6f6-55324c921053})");
}
unsafe impl ::windows::core::Interface for BeginStoryboard {
    type Vtable = IBeginStoryboardVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64189fcd_49ec_4e52_a6f6_55324c921053);
}
impl ::windows::core::RuntimeName for BeginStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BeginStoryboard";
}
impl ::core::convert::From<BeginStoryboard> for ::windows::core::IUnknown {
    fn from(value: BeginStoryboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BeginStoryboard> for ::windows::core::IUnknown {
    fn from(value: &BeginStoryboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BeginStoryboard> for ::windows::core::IInspectable {
    fn from(value: BeginStoryboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BeginStoryboard> for ::windows::core::IInspectable {
    fn from(value: &BeginStoryboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BeginStoryboard> for super::super::TriggerAction {
    fn from(value: BeginStoryboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BeginStoryboard> for super::super::TriggerAction {
    fn from(value: &BeginStoryboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::TriggerAction> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::TriggerAction> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::TriggerAction> for &BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::TriggerAction> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::TriggerAction>::into(self))
    }
}
impl ::core::convert::From<BeginStoryboard> for super::super::DependencyObject {
    fn from(value: BeginStoryboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BeginStoryboard> for super::super::DependencyObject {
    fn from(value: &BeginStoryboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BeginStoryboard {}
unsafe impl ::core::marker::Sync for BeginStoryboard {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct BounceEase(::windows::core::IUnknown);
impl BounceEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BounceEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Bounces(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetBounces(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Bounciness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetBounciness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn BouncesProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBounceEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn BouncinessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBounceEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBounceEaseStatics<R, F: FnOnce(&IBounceEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BounceEase, IBounceEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BounceEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BounceEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BounceEase {}
impl ::core::fmt::Debug for BounceEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BounceEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BounceEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BounceEase;{2bf1464e-fc71-47ed-85a1-3ba9577718b4})");
}
unsafe impl ::windows::core::Interface for BounceEase {
    type Vtable = IBounceEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bf1464e_fc71_47ed_85a1_3ba9577718b4);
}
impl ::windows::core::RuntimeName for BounceEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BounceEase";
}
impl ::core::convert::From<BounceEase> for ::windows::core::IUnknown {
    fn from(value: BounceEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BounceEase> for ::windows::core::IUnknown {
    fn from(value: &BounceEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BounceEase> for ::windows::core::IInspectable {
    fn from(value: BounceEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BounceEase> for ::windows::core::IInspectable {
    fn from(value: &BounceEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BounceEase> for EasingFunctionBase {
    fn from(value: BounceEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BounceEase> for EasingFunctionBase {
    fn from(value: &BounceEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<BounceEase> for super::super::DependencyObject {
    fn from(value: BounceEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BounceEase> for super::super::DependencyObject {
    fn from(value: &BounceEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BounceEase {}
unsafe impl ::core::marker::Sync for BounceEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct CircleEase(::windows::core::IUnknown);
impl CircleEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CircleEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CircleEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CircleEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CircleEase {}
impl ::core::fmt::Debug for CircleEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CircleEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CircleEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CircleEase;{53a3bdb2-9177-4e6e-a043-5082d889ab1f})");
}
unsafe impl ::windows::core::Interface for CircleEase {
    type Vtable = ICircleEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53a3bdb2_9177_4e6e_a043_5082d889ab1f);
}
impl ::windows::core::RuntimeName for CircleEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CircleEase";
}
impl ::core::convert::From<CircleEase> for ::windows::core::IUnknown {
    fn from(value: CircleEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CircleEase> for ::windows::core::IUnknown {
    fn from(value: &CircleEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CircleEase> for ::windows::core::IInspectable {
    fn from(value: CircleEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CircleEase> for ::windows::core::IInspectable {
    fn from(value: &CircleEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CircleEase> for EasingFunctionBase {
    fn from(value: CircleEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CircleEase> for EasingFunctionBase {
    fn from(value: &CircleEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<CircleEase> for super::super::DependencyObject {
    fn from(value: CircleEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CircleEase> for super::super::DependencyObject {
    fn from(value: &CircleEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CircleEase {}
unsafe impl ::core::marker::Sync for CircleEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ClockState(pub i32);
impl ClockState {
    pub const Active: Self = Self(0i32);
    pub const Filling: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
}
impl ::core::marker::Copy for ClockState {}
impl ::core::clone::Clone for ClockState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ClockState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ClockState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClockState {}
impl ::core::fmt::Debug for ClockState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClockState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClockState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.ClockState;i4)");
}
impl ::windows::core::DefaultType for ClockState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ColorAnimation(::windows::core::IUnknown);
impl ColorAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ByProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorAnimationStatics<R, F: FnOnce(&IColorAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimation, IColorAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorAnimation {}
impl ::core::fmt::Debug for ColorAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorAnimation;{b8ae8a15-0f63-4694-9467-bdafac1253ea})");
}
unsafe impl ::windows::core::Interface for ColorAnimation {
    type Vtable = IColorAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8ae8a15_0f63_4694_9467_bdafac1253ea);
}
impl ::windows::core::RuntimeName for ColorAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorAnimation";
}
impl ::core::convert::From<ColorAnimation> for ::windows::core::IUnknown {
    fn from(value: ColorAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimation> for ::windows::core::IUnknown {
    fn from(value: &ColorAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimation> for ::windows::core::IInspectable {
    fn from(value: ColorAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimation> for ::windows::core::IInspectable {
    fn from(value: &ColorAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimation> for Timeline {
    fn from(value: ColorAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimation> for Timeline {
    fn from(value: &ColorAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<ColorAnimation> for super::super::DependencyObject {
    fn from(value: ColorAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimation> for super::super::DependencyObject {
    fn from(value: &ColorAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorAnimation {}
unsafe impl ::core::marker::Sync for ColorAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ColorAnimationUsingKeyFrames(::windows::core::IUnknown);
impl ColorAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<ColorKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ColorKeyFrameCollection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorAnimationUsingKeyFramesStatics<R, F: FnOnce(&IColorAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimationUsingKeyFrames, IColorAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for ColorAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorAnimationUsingKeyFrames;{f5c82640-13c3-42aa-9ae2-7e6b51c92f95})");
}
unsafe impl ::windows::core::Interface for ColorAnimationUsingKeyFrames {
    type Vtable = IColorAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c82640_13c3_42aa_9ae2_7e6b51c92f95);
}
impl ::windows::core::RuntimeName for ColorAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorAnimationUsingKeyFrames";
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for Timeline {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for Timeline {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for ColorAnimationUsingKeyFrames {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ColorKeyFrame(::windows::core::IUnknown);
impl ColorKeyFrame {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorKeyFrameStatics<R, F: FnOnce(&IColorKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorKeyFrame, IColorKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorKeyFrame {}
impl ::core::fmt::Debug for ColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrame;{b51d82d9-0910-4589-a284-b0c9205858e9})");
}
unsafe impl ::windows::core::Interface for ColorKeyFrame {
    type Vtable = IColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb51d82d9_0910_4589_a284_b0c9205858e9);
}
impl ::windows::core::RuntimeName for ColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorKeyFrame";
}
impl ::core::convert::From<ColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: ColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &ColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: ColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &ColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorKeyFrame> for super::super::DependencyObject {
    fn from(value: ColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &ColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorKeyFrame {}
unsafe impl ::core::marker::Sync for ColorKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ColorKeyFrameCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ColorKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<ColorKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<ColorKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ColorKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ColorKeyFrame>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<ColorKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<ColorKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ColorKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<ColorKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ColorKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ColorKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ColorKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ColorKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ColorKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrame;{b51d82d9-0910-4589-a284-b0c9205858e9})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ColorKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVectorVtbl<ColorKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ColorKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ColorKeyFrameCollection {
    type Item = ColorKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ColorKeyFrameCollection {
    type Item = ColorKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ColorKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: ColorKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ColorKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &ColorKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ColorKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: ColorKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ColorKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &ColorKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ColorKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: ColorKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ColorKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ColorKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame>> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame>> for &ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ColorKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: ColorKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ColorKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ColorKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> for &ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ColorKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ColorKeyFrameCollection {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct CommonNavigationTransitionInfo(::windows::core::IUnknown);
impl CommonNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommonNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsStaggeringEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsStaggerElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetIsStaggerElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsStaggerElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc(hidden)]
    pub fn ICommonNavigationTransitionInfoStatics<R, F: FnOnce(&ICommonNavigationTransitionInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommonNavigationTransitionInfo, ICommonNavigationTransitionInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CommonNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommonNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommonNavigationTransitionInfo {}
impl ::core::fmt::Debug for CommonNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CommonNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CommonNavigationTransitionInfo;{50345692-a555-4624-a361-0a91c1706473})");
}
unsafe impl ::windows::core::Interface for CommonNavigationTransitionInfo {
    type Vtable = ICommonNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50345692_a555_4624_a361_0a91c1706473);
}
impl ::windows::core::RuntimeName for CommonNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CommonNavigationTransitionInfo";
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CommonNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for CommonNavigationTransitionInfo {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ConnectedAnimation(::windows::core::IUnknown);
impl ConnectedAnimation {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TryStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(&self, destination: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsScaleAnimationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsScaleAnimationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryStartWithCoordinatedElements<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement>>>(&self, destination: Param0, coordinatedelements: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), coordinatedelements.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetAnimationComponent<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Composition::ICompositionAnimationBase>>(&self, component: ConnectedAnimationComponent, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), component, animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Configuration(&self) -> ::windows::core::Result<ConnectedAnimationConfiguration> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConnectedAnimationConfiguration>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ConnectedAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectedAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectedAnimation {}
impl ::core::fmt::Debug for ConnectedAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimation;{3518628c-f387-4c25-ac98-44e86c3cadf0})");
}
unsafe impl ::windows::core::Interface for ConnectedAnimation {
    type Vtable = IConnectedAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3518628c_f387_4c25_ac98_44e86c3cadf0);
}
impl ::windows::core::RuntimeName for ConnectedAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimation";
}
impl ::core::convert::From<ConnectedAnimation> for ::windows::core::IUnknown {
    fn from(value: ConnectedAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimation> for ::windows::core::IUnknown {
    fn from(value: &ConnectedAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectedAnimation> for ::windows::core::IInspectable {
    fn from(value: ConnectedAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimation> for ::windows::core::IInspectable {
    fn from(value: &ConnectedAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimation {}
unsafe impl ::core::marker::Sync for ConnectedAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ConnectedAnimationComponent(pub i32);
impl ConnectedAnimationComponent {
    pub const OffsetX: Self = Self(0i32);
    pub const OffsetY: Self = Self(1i32);
    pub const CrossFade: Self = Self(2i32);
    pub const Scale: Self = Self(3i32);
}
impl ::core::marker::Copy for ConnectedAnimationComponent {}
impl ::core::clone::Clone for ConnectedAnimationComponent {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ConnectedAnimationComponent {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ConnectedAnimationComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectedAnimationComponent {}
impl ::core::fmt::Debug for ConnectedAnimationComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimationComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimationComponent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.ConnectedAnimationComponent;i4)");
}
impl ::windows::core::DefaultType for ConnectedAnimationComponent {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ConnectedAnimationConfiguration(::windows::core::IUnknown);
impl ConnectedAnimationConfiguration {}
impl ::core::clone::Clone for ConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for ConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimationConfiguration;{00218aae-cd8c-5651-92a0-c1db95c03998})");
}
unsafe impl ::windows::core::Interface for ConnectedAnimationConfiguration {
    type Vtable = IConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00218aae_cd8c_5651_92a0_c1db95c03998);
}
impl ::windows::core::RuntimeName for ConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimationConfiguration";
}
impl ::core::convert::From<ConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: ConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &ConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: ConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &ConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for ConnectedAnimationConfiguration {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ConnectedAnimationService(::windows::core::IUnknown);
impl ConnectedAnimationService {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub fn DefaultEasingFunction(&self) -> ::windows::core::Result<super::super::super::Composition::CompositionEasingFunction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Composition::CompositionEasingFunction>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetDefaultEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::CompositionEasingFunction>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn PrepareToAnimate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::UIElement>>(&self, key: Param0, source: Param1) -> ::windows::core::Result<ConnectedAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), source.into_param().abi(), &mut result__).from_abi::<ConnectedAnimation>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<ConnectedAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<ConnectedAnimation>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<ConnectedAnimationService> {
        Self::IConnectedAnimationServiceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConnectedAnimationService>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IConnectedAnimationServiceStatics<R, F: FnOnce(&IConnectedAnimationServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConnectedAnimationService, IConnectedAnimationServiceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConnectedAnimationService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectedAnimationService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectedAnimationService {}
impl ::core::fmt::Debug for ConnectedAnimationService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimationService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimationService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimationService;{1c6875c9-19bb-4d47-b9aa-66c802dcb9ff})");
}
unsafe impl ::windows::core::Interface for ConnectedAnimationService {
    type Vtable = IConnectedAnimationServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c6875c9_19bb_4d47_b9aa_66c802dcb9ff);
}
impl ::windows::core::RuntimeName for ConnectedAnimationService {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimationService";
}
impl ::core::convert::From<ConnectedAnimationService> for ::windows::core::IUnknown {
    fn from(value: ConnectedAnimationService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationService> for ::windows::core::IUnknown {
    fn from(value: &ConnectedAnimationService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectedAnimationService> for ::windows::core::IInspectable {
    fn from(value: ConnectedAnimationService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationService> for ::windows::core::IInspectable {
    fn from(value: &ConnectedAnimationService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimationService {}
unsafe impl ::core::marker::Sync for ConnectedAnimationService {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ContentThemeTransition(::windows::core::IUnknown);
impl ContentThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn HorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn VerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn HorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContentThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn VerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContentThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentThemeTransitionStatics<R, F: FnOnce(&IContentThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentThemeTransition, IContentThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentThemeTransition {}
impl ::core::fmt::Debug for ContentThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ContentThemeTransition;{f66fc5c3-5915-437d-8e3b-adf8e7f0ab57})");
}
unsafe impl ::windows::core::Interface for ContentThemeTransition {
    type Vtable = IContentThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf66fc5c3_5915_437d_8e3b_adf8e7f0ab57);
}
impl ::windows::core::RuntimeName for ContentThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ContentThemeTransition";
}
impl ::core::convert::From<ContentThemeTransition> for ::windows::core::IUnknown {
    fn from(value: ContentThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &ContentThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentThemeTransition> for ::windows::core::IInspectable {
    fn from(value: ContentThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &ContentThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentThemeTransition> for Transition {
    fn from(value: ContentThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentThemeTransition> for Transition {
    fn from(value: &ContentThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<ContentThemeTransition> for super::super::DependencyObject {
    fn from(value: ContentThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentThemeTransition> for super::super::DependencyObject {
    fn from(value: &ContentThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContentThemeTransition {}
unsafe impl ::core::marker::Sync for ContentThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ContinuumNavigationTransitionInfo(::windows::core::IUnknown);
impl ContinuumNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContinuumNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitElement(&self) -> ::windows::core::Result<super::super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetExitElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsEntranceElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetIsEntranceElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsEntranceElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsExitElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetIsExitElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsExitElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitElementContainerProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls'*"]
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn GetExitElementContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Controls::ListViewBase>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls'*"]
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn SetExitElementContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Controls::ListViewBase>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc(hidden)]
    pub fn IContinuumNavigationTransitionInfoStatics<R, F: FnOnce(&IContinuumNavigationTransitionInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContinuumNavigationTransitionInfo, IContinuumNavigationTransitionInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContinuumNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContinuumNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContinuumNavigationTransitionInfo {}
impl ::core::fmt::Debug for ContinuumNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContinuumNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContinuumNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ContinuumNavigationTransitionInfo;{4be1dbad-8ba6-4004-8438-8a9017978543})");
}
unsafe impl ::windows::core::Interface for ContinuumNavigationTransitionInfo {
    type Vtable = IContinuumNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4be1dbad_8ba6_4004_8438_8a9017978543);
}
impl ::windows::core::RuntimeName for ContinuumNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ContinuumNavigationTransitionInfo";
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContinuumNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for ContinuumNavigationTransitionInfo {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct CubicEase(::windows::core::IUnknown);
impl CubicEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CubicEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CubicEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CubicEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CubicEase {}
impl ::core::fmt::Debug for CubicEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CubicEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CubicEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CubicEase;{1b94fc76-dad7-4354-b1a2-7969fbf6a70d})");
}
unsafe impl ::windows::core::Interface for CubicEase {
    type Vtable = ICubicEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b94fc76_dad7_4354_b1a2_7969fbf6a70d);
}
impl ::windows::core::RuntimeName for CubicEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CubicEase";
}
impl ::core::convert::From<CubicEase> for ::windows::core::IUnknown {
    fn from(value: CubicEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CubicEase> for ::windows::core::IUnknown {
    fn from(value: &CubicEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CubicEase> for ::windows::core::IInspectable {
    fn from(value: CubicEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CubicEase> for ::windows::core::IInspectable {
    fn from(value: &CubicEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CubicEase> for EasingFunctionBase {
    fn from(value: CubicEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CubicEase> for EasingFunctionBase {
    fn from(value: &CubicEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<CubicEase> for super::super::DependencyObject {
    fn from(value: CubicEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CubicEase> for super::super::DependencyObject {
    fn from(value: &CubicEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CubicEase {}
unsafe impl ::core::marker::Sync for CubicEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DirectConnectedAnimationConfiguration(::windows::core::IUnknown);
impl DirectConnectedAnimationConfiguration {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn new() -> ::windows::core::Result<DirectConnectedAnimationConfiguration> {
        Self::IDirectConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<DirectConnectedAnimationConfiguration>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDirectConnectedAnimationConfigurationFactory<R, F: FnOnce(&IDirectConnectedAnimationConfigurationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DirectConnectedAnimationConfiguration, IDirectConnectedAnimationConfigurationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DirectConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DirectConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DirectConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for DirectConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DirectConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DirectConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DirectConnectedAnimationConfiguration;{ee5d736f-5738-5d86-b770-151948cf365e})");
}
unsafe impl ::windows::core::Interface for DirectConnectedAnimationConfiguration {
    type Vtable = IDirectConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5d736f_5738_5d86_b770_151948cf365e);
}
impl ::windows::core::RuntimeName for DirectConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DirectConnectedAnimationConfiguration";
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for &DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for DirectConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for DirectConnectedAnimationConfiguration {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DiscreteColorKeyFrame(::windows::core::IUnknown);
impl DiscreteColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscreteColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscreteColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscreteColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscreteColorKeyFrame {}
impl ::core::fmt::Debug for DiscreteColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscreteColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DiscreteColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteColorKeyFrame;{230c08f4-e062-4cb1-8e2a-14093d73ed8c})");
}
unsafe impl ::windows::core::Interface for DiscreteColorKeyFrame {
    type Vtable = IDiscreteColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230c08f4_e062_4cb1_8e2a_14093d73ed8c);
}
impl ::windows::core::RuntimeName for DiscreteColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteColorKeyFrame";
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ColorKeyFrame {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ColorKeyFrame {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscreteColorKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteColorKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DiscreteDoubleKeyFrame(::windows::core::IUnknown);
impl DiscreteDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscreteDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscreteDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscreteDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscreteDoubleKeyFrame {}
impl ::core::fmt::Debug for DiscreteDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscreteDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DiscreteDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteDoubleKeyFrame;{f5f51f3a-ad11-49ce-8e1c-08fdf1447446})");
}
unsafe impl ::windows::core::Interface for DiscreteDoubleKeyFrame {
    type Vtable = IDiscreteDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f51f3a_ad11_49ce_8e1c_08fdf1447446);
}
impl ::windows::core::RuntimeName for DiscreteDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteDoubleKeyFrame";
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscreteDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteDoubleKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DiscreteObjectKeyFrame(::windows::core::IUnknown);
impl DiscreteObjectKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscreteObjectKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscreteObjectKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscreteObjectKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscreteObjectKeyFrame {}
impl ::core::fmt::Debug for DiscreteObjectKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscreteObjectKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DiscreteObjectKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteObjectKeyFrame;{c7dcde89-f12d-4a9c-8199-e7a9ece3a473})");
}
unsafe impl ::windows::core::Interface for DiscreteObjectKeyFrame {
    type Vtable = IDiscreteObjectKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7dcde89_f12d_4a9c_8199_e7a9ece3a473);
}
impl ::windows::core::RuntimeName for DiscreteObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteObjectKeyFrame";
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ObjectKeyFrame {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ObjectKeyFrame {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ObjectKeyFrame> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ObjectKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ObjectKeyFrame> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ObjectKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ObjectKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscreteObjectKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteObjectKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DiscretePointKeyFrame(::windows::core::IUnknown);
impl DiscretePointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscretePointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscretePointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscretePointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscretePointKeyFrame {}
impl ::core::fmt::Debug for DiscretePointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscretePointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DiscretePointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscretePointKeyFrame;{e0a9070d-4c42-4a90-983a-75f5a83a2fbe})");
}
unsafe impl ::windows::core::Interface for DiscretePointKeyFrame {
    type Vtable = IDiscretePointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0a9070d_4c42_4a90_983a_75f5a83a2fbe);
}
impl ::windows::core::RuntimeName for DiscretePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscretePointKeyFrame";
}
impl ::core::convert::From<DiscretePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscretePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscretePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for PointKeyFrame {
    fn from(value: DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for PointKeyFrame {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscretePointKeyFrame {}
unsafe impl ::core::marker::Sync for DiscretePointKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DoubleAnimation(::windows::core::IUnknown);
impl DoubleAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ByProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDoubleAnimationStatics<R, F: FnOnce(&IDoubleAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimation, IDoubleAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DoubleAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleAnimation {}
impl ::core::fmt::Debug for DoubleAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleAnimation;{7e9f3d59-0f07-4bc9-977d-03763ff8154f})");
}
unsafe impl ::windows::core::Interface for DoubleAnimation {
    type Vtable = IDoubleAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e9f3d59_0f07_4bc9_977d_03763ff8154f);
}
impl ::windows::core::RuntimeName for DoubleAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleAnimation";
}
impl ::core::convert::From<DoubleAnimation> for ::windows::core::IUnknown {
    fn from(value: DoubleAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimation> for ::windows::core::IUnknown {
    fn from(value: &DoubleAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimation> for ::windows::core::IInspectable {
    fn from(value: DoubleAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimation> for ::windows::core::IInspectable {
    fn from(value: &DoubleAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimation> for Timeline {
    fn from(value: DoubleAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimation> for Timeline {
    fn from(value: &DoubleAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DoubleAnimation> for super::super::DependencyObject {
    fn from(value: DoubleAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimation> for super::super::DependencyObject {
    fn from(value: &DoubleAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleAnimation {}
unsafe impl ::core::marker::Sync for DoubleAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DoubleAnimationUsingKeyFrames(::windows::core::IUnknown);
impl DoubleAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<DoubleKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DoubleKeyFrameCollection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDoubleAnimationUsingKeyFramesStatics<R, F: FnOnce(&IDoubleAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimationUsingKeyFrames, IDoubleAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DoubleAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for DoubleAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleAnimationUsingKeyFrames;{4fee628f-bfee-4f75-83c2-a93b39488473})");
}
unsafe impl ::windows::core::Interface for DoubleAnimationUsingKeyFrames {
    type Vtable = IDoubleAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fee628f_bfee_4f75_83c2_a93b39488473);
}
impl ::windows::core::RuntimeName for DoubleAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleAnimationUsingKeyFrames";
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for Timeline {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for Timeline {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for DoubleAnimationUsingKeyFrames {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DoubleKeyFrame(::windows::core::IUnknown);
impl DoubleKeyFrame {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Value(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetValue(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDoubleKeyFrameStatics<R, F: FnOnce(&IDoubleKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleKeyFrame, IDoubleKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleKeyFrame {}
impl ::core::fmt::Debug for DoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrame;{674456fd-e81e-4f4e-b4ad-0acfed9ecd68})");
}
unsafe impl ::windows::core::Interface for DoubleKeyFrame {
    type Vtable = IDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674456fd_e81e_4f4e_b4ad_0acfed9ecd68);
}
impl ::windows::core::RuntimeName for DoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleKeyFrame";
}
impl ::core::convert::From<DoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: DoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &DoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleKeyFrame {}
unsafe impl ::core::marker::Sync for DoubleKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct DoubleKeyFrameCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl DoubleKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<DoubleKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<DoubleKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<DoubleKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<DoubleKeyFrame>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<DoubleKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<DoubleKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<DoubleKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<DoubleKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for DoubleKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for DoubleKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for DoubleKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for DoubleKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for DoubleKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrame;{674456fd-e81e-4f4e-b4ad-0acfed9ecd68})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for DoubleKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVectorVtbl<DoubleKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for DoubleKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for DoubleKeyFrameCollection {
    type Item = DoubleKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &DoubleKeyFrameCollection {
    type Item = DoubleKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: DoubleKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &DoubleKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: DoubleKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &DoubleKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DoubleKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: DoubleKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DoubleKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DoubleKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame>> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame>> for &DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DoubleKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: DoubleKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DoubleKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DoubleKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> for &DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DoubleKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DoubleKeyFrameCollection {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DragItemThemeAnimation(::windows::core::IUnknown);
impl DragItemThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragItemThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragItemThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDragItemThemeAnimationStatics<R, F: FnOnce(&IDragItemThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragItemThemeAnimation, IDragItemThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragItemThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragItemThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragItemThemeAnimation {}
impl ::core::fmt::Debug for DragItemThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragItemThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DragItemThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DragItemThemeAnimation;{0c7d5db5-7ed6-4949-b4e6-a78c9f4f978d})");
}
unsafe impl ::windows::core::Interface for DragItemThemeAnimation {
    type Vtable = IDragItemThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c7d5db5_7ed6_4949_b4e6_a78c9f4f978d);
}
impl ::windows::core::RuntimeName for DragItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DragItemThemeAnimation";
}
impl ::core::convert::From<DragItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DragItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DragItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for Timeline {
    fn from(value: DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for Timeline {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DragItemThemeAnimation {}
unsafe impl ::core::marker::Sync for DragItemThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DragOverThemeAnimation(::windows::core::IUnknown);
impl DragOverThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragOverThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetToOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn Direction(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::AnimationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn DirectionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDragOverThemeAnimationStatics<R, F: FnOnce(&IDragOverThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragOverThemeAnimation, IDragOverThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragOverThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragOverThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragOverThemeAnimation {}
impl ::core::fmt::Debug for DragOverThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragOverThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DragOverThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DragOverThemeAnimation;{72f762f7-7e51-4a6b-b937-dc4b4c1c5458})");
}
unsafe impl ::windows::core::Interface for DragOverThemeAnimation {
    type Vtable = IDragOverThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72f762f7_7e51_4a6b_b937_dc4b4c1c5458);
}
impl ::windows::core::RuntimeName for DragOverThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DragOverThemeAnimation";
}
impl ::core::convert::From<DragOverThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DragOverThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DragOverThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for Timeline {
    fn from(value: DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for Timeline {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for super::super::DependencyObject {
    fn from(value: DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DragOverThemeAnimation {}
unsafe impl ::core::marker::Sync for DragOverThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DrillInNavigationTransitionInfo(::windows::core::IUnknown);
impl DrillInNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillInNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DrillInNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DrillInNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DrillInNavigationTransitionInfo {}
impl ::core::fmt::Debug for DrillInNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DrillInNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DrillInNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillInNavigationTransitionInfo;{3b86201a-45d3-463b-939e-c8595f439bcc})");
}
unsafe impl ::windows::core::Interface for DrillInNavigationTransitionInfo {
    type Vtable = IDrillInNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b86201a_45d3_463b_939e_c8595f439bcc);
}
impl ::windows::core::RuntimeName for DrillInNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillInNavigationTransitionInfo";
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DrillInNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for DrillInNavigationTransitionInfo {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DrillInThemeAnimation(::windows::core::IUnknown);
impl DrillInThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillInThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEntranceTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEntranceTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetExitTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetExitTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDrillInThemeAnimationStatics<R, F: FnOnce(&IDrillInThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillInThemeAnimation, IDrillInThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DrillInThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DrillInThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DrillInThemeAnimation {}
impl ::core::fmt::Debug for DrillInThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DrillInThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DrillInThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillInThemeAnimation;{b090b824-f1d2-41b8-87ba-78034126594c})");
}
unsafe impl ::windows::core::Interface for DrillInThemeAnimation {
    type Vtable = IDrillInThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb090b824_f1d2_41b8_87ba_78034126594c);
}
impl ::windows::core::RuntimeName for DrillInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillInThemeAnimation";
}
impl ::core::convert::From<DrillInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DrillInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DrillInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for Timeline {
    fn from(value: DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for Timeline {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for super::super::DependencyObject {
    fn from(value: DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DrillInThemeAnimation {}
unsafe impl ::core::marker::Sync for DrillInThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DrillOutThemeAnimation(::windows::core::IUnknown);
impl DrillOutThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillOutThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEntranceTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEntranceTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetExitTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetExitTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EntranceTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExitTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDrillOutThemeAnimationStatics<R, F: FnOnce(&IDrillOutThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillOutThemeAnimation, IDrillOutThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DrillOutThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DrillOutThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DrillOutThemeAnimation {}
impl ::core::fmt::Debug for DrillOutThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DrillOutThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DrillOutThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillOutThemeAnimation;{d890ccdf-06d3-4f7e-8e4a-4fb76e256139})");
}
unsafe impl ::windows::core::Interface for DrillOutThemeAnimation {
    type Vtable = IDrillOutThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd890ccdf_06d3_4f7e_8e4a_4fb76e256139);
}
impl ::windows::core::RuntimeName for DrillOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillOutThemeAnimation";
}
impl ::core::convert::From<DrillOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DrillOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DrillOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for Timeline {
    fn from(value: DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for Timeline {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DrillOutThemeAnimation {}
unsafe impl ::core::marker::Sync for DrillOutThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct DropTargetItemThemeAnimation(::windows::core::IUnknown);
impl DropTargetItemThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DropTargetItemThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDropTargetItemThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDropTargetItemThemeAnimationStatics<R, F: FnOnce(&IDropTargetItemThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DropTargetItemThemeAnimation, IDropTargetItemThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DropTargetItemThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DropTargetItemThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DropTargetItemThemeAnimation {}
impl ::core::fmt::Debug for DropTargetItemThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DropTargetItemThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DropTargetItemThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DropTargetItemThemeAnimation;{1881c968-1824-462b-87e8-c357212b977b})");
}
unsafe impl ::windows::core::Interface for DropTargetItemThemeAnimation {
    type Vtable = IDropTargetItemThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1881c968_1824_462b_87e8_c357212b977b);
}
impl ::windows::core::RuntimeName for DropTargetItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DropTargetItemThemeAnimation";
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for Timeline {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for Timeline {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DropTargetItemThemeAnimation {}
unsafe impl ::core::marker::Sync for DropTargetItemThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EasingColorKeyFrame(::windows::core::IUnknown);
impl EasingColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEasingColorKeyFrameStatics<R, F: FnOnce(&IEasingColorKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingColorKeyFrame, IEasingColorKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingColorKeyFrame {}
impl ::core::fmt::Debug for EasingColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasingColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingColorKeyFrame;{c733d630-f4b9-4934-9bdd-27ac5ed1cfd8})");
}
unsafe impl ::windows::core::Interface for EasingColorKeyFrame {
    type Vtable = IEasingColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc733d630_f4b9_4934_9bdd_27ac5ed1cfd8);
}
impl ::windows::core::RuntimeName for EasingColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingColorKeyFrame";
}
impl ::core::convert::From<EasingColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: EasingColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: EasingColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for ColorKeyFrame {
    fn from(value: EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ColorKeyFrame {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingColorKeyFrame {}
unsafe impl ::core::marker::Sync for EasingColorKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EasingDoubleKeyFrame(::windows::core::IUnknown);
impl EasingDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEasingDoubleKeyFrameStatics<R, F: FnOnce(&IEasingDoubleKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingDoubleKeyFrame, IEasingDoubleKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingDoubleKeyFrame {}
impl ::core::fmt::Debug for EasingDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasingDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingDoubleKeyFrame;{965adb8d-9a54-4108-b4ff-b5a5212cb338})");
}
unsafe impl ::windows::core::Interface for EasingDoubleKeyFrame {
    type Vtable = IEasingDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965adb8d_9a54_4108_b4ff_b5a5212cb338);
}
impl ::windows::core::RuntimeName for EasingDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingDoubleKeyFrame";
}
impl ::core::convert::From<EasingDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for EasingDoubleKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EasingFunctionBase(::windows::core::IUnknown);
impl EasingFunctionBase {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingMode(&self) -> ::windows::core::Result<EasingMode> {
        let this = self;
        unsafe {
            let mut result__: EasingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEasingMode(&self, value: EasingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Ease(&self, normalizedtime: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), normalizedtime, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingFunctionBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEasingFunctionBaseStatics<R, F: FnOnce(&IEasingFunctionBaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingFunctionBase, IEasingFunctionBaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingFunctionBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingFunctionBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingFunctionBase {}
impl ::core::fmt::Debug for EasingFunctionBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingFunctionBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasingFunctionBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingFunctionBase;{c108383f-2c02-4151-8ecd-68ddaa3f0d9b})");
}
unsafe impl ::windows::core::Interface for EasingFunctionBase {
    type Vtable = IEasingFunctionBaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc108383f_2c02_4151_8ecd_68ddaa3f0d9b);
}
impl ::windows::core::RuntimeName for EasingFunctionBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingFunctionBase";
}
impl ::core::convert::From<EasingFunctionBase> for ::windows::core::IUnknown {
    fn from(value: EasingFunctionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingFunctionBase> for ::windows::core::IUnknown {
    fn from(value: &EasingFunctionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingFunctionBase> for ::windows::core::IInspectable {
    fn from(value: EasingFunctionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingFunctionBase> for ::windows::core::IInspectable {
    fn from(value: &EasingFunctionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingFunctionBase> for super::super::DependencyObject {
    fn from(value: EasingFunctionBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingFunctionBase> for super::super::DependencyObject {
    fn from(value: &EasingFunctionBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingFunctionBase {}
unsafe impl ::core::marker::Sync for EasingFunctionBase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EasingMode(pub i32);
impl EasingMode {
    pub const EaseOut: Self = Self(0i32);
    pub const EaseIn: Self = Self(1i32);
    pub const EaseInOut: Self = Self(2i32);
}
impl ::core::marker::Copy for EasingMode {}
impl ::core::clone::Clone for EasingMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EasingMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EasingMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingMode {}
impl ::core::fmt::Debug for EasingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.EasingMode;i4)");
}
impl ::windows::core::DefaultType for EasingMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EasingPointKeyFrame(::windows::core::IUnknown);
impl EasingPointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingPointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingPointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEasingPointKeyFrameStatics<R, F: FnOnce(&IEasingPointKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingPointKeyFrame, IEasingPointKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingPointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingPointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingPointKeyFrame {}
impl ::core::fmt::Debug for EasingPointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingPointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasingPointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingPointKeyFrame;{b3c91380-6868-4225-a70b-3981cc0b2947})");
}
unsafe impl ::windows::core::Interface for EasingPointKeyFrame {
    type Vtable = IEasingPointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c91380_6868_4225_a70b_3981cc0b2947);
}
impl ::windows::core::RuntimeName for EasingPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingPointKeyFrame";
}
impl ::core::convert::From<EasingPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: EasingPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: EasingPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for PointKeyFrame {
    fn from(value: EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for PointKeyFrame {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingPointKeyFrame {}
unsafe impl ::core::marker::Sync for EasingPointKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EdgeUIThemeTransition(::windows::core::IUnknown);
impl EdgeUIThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EdgeUIThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::EdgeTransitionLocation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::EdgeTransitionLocation>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EdgeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEdgeUIThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEdgeUIThemeTransitionStatics<R, F: FnOnce(&IEdgeUIThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EdgeUIThemeTransition, IEdgeUIThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EdgeUIThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EdgeUIThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EdgeUIThemeTransition {}
impl ::core::fmt::Debug for EdgeUIThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeUIThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EdgeUIThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EdgeUIThemeTransition;{5c86c19b-49d7-19ec-cf19-83a73c6de75e})");
}
unsafe impl ::windows::core::Interface for EdgeUIThemeTransition {
    type Vtable = IEdgeUIThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c86c19b_49d7_19ec_cf19_83a73c6de75e);
}
impl ::windows::core::RuntimeName for EdgeUIThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EdgeUIThemeTransition";
}
impl ::core::convert::From<EdgeUIThemeTransition> for ::windows::core::IUnknown {
    fn from(value: EdgeUIThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for ::windows::core::IInspectable {
    fn from(value: EdgeUIThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for Transition {
    fn from(value: EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for Transition {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for super::super::DependencyObject {
    fn from(value: EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for super::super::DependencyObject {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EdgeUIThemeTransition {}
unsafe impl ::core::marker::Sync for EdgeUIThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ElasticEase(::windows::core::IUnknown);
impl ElasticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElasticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Oscillations(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOscillations(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Springiness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetSpringiness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OscillationsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IElasticEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SpringinessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IElasticEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IElasticEaseStatics<R, F: FnOnce(&IElasticEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElasticEase, IElasticEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ElasticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ElasticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ElasticEase {}
impl ::core::fmt::Debug for ElasticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElasticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ElasticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ElasticEase;{ef5ba58c-b0b6-4a6c-9ca8-fb4233f12459})");
}
unsafe impl ::windows::core::Interface for ElasticEase {
    type Vtable = IElasticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5ba58c_b0b6_4a6c_9ca8_fb4233f12459);
}
impl ::windows::core::RuntimeName for ElasticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ElasticEase";
}
impl ::core::convert::From<ElasticEase> for ::windows::core::IUnknown {
    fn from(value: ElasticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElasticEase> for ::windows::core::IUnknown {
    fn from(value: &ElasticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ElasticEase> for ::windows::core::IInspectable {
    fn from(value: ElasticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElasticEase> for ::windows::core::IInspectable {
    fn from(value: &ElasticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ElasticEase> for EasingFunctionBase {
    fn from(value: ElasticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ElasticEase> for EasingFunctionBase {
    fn from(value: &ElasticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<ElasticEase> for super::super::DependencyObject {
    fn from(value: ElasticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ElasticEase> for super::super::DependencyObject {
    fn from(value: &ElasticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ElasticEase {}
unsafe impl ::core::marker::Sync for ElasticEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EntranceNavigationTransitionInfo(::windows::core::IUnknown);
impl EntranceNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsTargetElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetIsTargetElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsTargetElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc(hidden)]
    pub fn IEntranceNavigationTransitionInfoStatics<R, F: FnOnce(&IEntranceNavigationTransitionInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceNavigationTransitionInfo, IEntranceNavigationTransitionInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EntranceNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EntranceNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EntranceNavigationTransitionInfo {}
impl ::core::fmt::Debug for EntranceNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EntranceNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EntranceNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EntranceNavigationTransitionInfo;{720a256b-1c8a-41ee-82ec-8a87c0cf47da})");
}
unsafe impl ::windows::core::Interface for EntranceNavigationTransitionInfo {
    type Vtable = IEntranceNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x720a256b_1c8a_41ee_82ec_8a87c0cf47da);
}
impl ::windows::core::RuntimeName for EntranceNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EntranceNavigationTransitionInfo";
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EntranceNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for EntranceNavigationTransitionInfo {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct EntranceThemeTransition(::windows::core::IUnknown);
impl EntranceThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsStaggeringEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEntranceThemeTransitionStatics<R, F: FnOnce(&IEntranceThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceThemeTransition, IEntranceThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EntranceThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EntranceThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EntranceThemeTransition {}
impl ::core::fmt::Debug for EntranceThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EntranceThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EntranceThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EntranceThemeTransition;{07698c09-a8e3-419a-a01d-7410a0ae8ec8})");
}
unsafe impl ::windows::core::Interface for EntranceThemeTransition {
    type Vtable = IEntranceThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07698c09_a8e3_419a_a01d_7410a0ae8ec8);
}
impl ::windows::core::RuntimeName for EntranceThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EntranceThemeTransition";
}
impl ::core::convert::From<EntranceThemeTransition> for ::windows::core::IUnknown {
    fn from(value: EntranceThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceThemeTransition> for ::windows::core::IInspectable {
    fn from(value: EntranceThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceThemeTransition> for Transition {
    fn from(value: EntranceThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for Transition {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<EntranceThemeTransition> for super::super::DependencyObject {
    fn from(value: EntranceThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for super::super::DependencyObject {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EntranceThemeTransition {}
unsafe impl ::core::marker::Sync for EntranceThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ExponentialEase(::windows::core::IUnknown);
impl ExponentialEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExponentialEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Exponent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetExponent(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ExponentProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IExponentialEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IExponentialEaseStatics<R, F: FnOnce(&IExponentialEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExponentialEase, IExponentialEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExponentialEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExponentialEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExponentialEase {}
impl ::core::fmt::Debug for ExponentialEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExponentialEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExponentialEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ExponentialEase;{7cb9e41d-f0bb-4bca-9da5-9ba3a11734c4})");
}
unsafe impl ::windows::core::Interface for ExponentialEase {
    type Vtable = IExponentialEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cb9e41d_f0bb_4bca_9da5_9ba3a11734c4);
}
impl ::windows::core::RuntimeName for ExponentialEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ExponentialEase";
}
impl ::core::convert::From<ExponentialEase> for ::windows::core::IUnknown {
    fn from(value: ExponentialEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExponentialEase> for ::windows::core::IUnknown {
    fn from(value: &ExponentialEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExponentialEase> for ::windows::core::IInspectable {
    fn from(value: ExponentialEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExponentialEase> for ::windows::core::IInspectable {
    fn from(value: &ExponentialEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExponentialEase> for EasingFunctionBase {
    fn from(value: ExponentialEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ExponentialEase> for EasingFunctionBase {
    fn from(value: &ExponentialEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<ExponentialEase> for super::super::DependencyObject {
    fn from(value: ExponentialEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ExponentialEase> for super::super::DependencyObject {
    fn from(value: &ExponentialEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ExponentialEase {}
unsafe impl ::core::marker::Sync for ExponentialEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct FadeInThemeAnimation(::windows::core::IUnknown);
impl FadeInThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeInThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFadeInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFadeInThemeAnimationStatics<R, F: FnOnce(&IFadeInThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeInThemeAnimation, IFadeInThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FadeInThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FadeInThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FadeInThemeAnimation {}
impl ::core::fmt::Debug for FadeInThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FadeInThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FadeInThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.FadeInThemeAnimation;{6d4bc8f5-a918-4477-8078-554c68812ab8})");
}
unsafe impl ::windows::core::Interface for FadeInThemeAnimation {
    type Vtable = IFadeInThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4bc8f5_a918_4477_8078_554c68812ab8);
}
impl ::windows::core::RuntimeName for FadeInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.FadeInThemeAnimation";
}
impl ::core::convert::From<FadeInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: FadeInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: FadeInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for Timeline {
    fn from(value: FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for Timeline {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for super::super::DependencyObject {
    fn from(value: FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for FadeInThemeAnimation {}
unsafe impl ::core::marker::Sync for FadeInThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct FadeOutThemeAnimation(::windows::core::IUnknown);
impl FadeOutThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeOutThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFadeOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFadeOutThemeAnimationStatics<R, F: FnOnce(&IFadeOutThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeOutThemeAnimation, IFadeOutThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FadeOutThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FadeOutThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FadeOutThemeAnimation {}
impl ::core::fmt::Debug for FadeOutThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FadeOutThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FadeOutThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.FadeOutThemeAnimation;{89276ba9-ffd4-45b6-9b9a-ced48951e712})");
}
unsafe impl ::windows::core::Interface for FadeOutThemeAnimation {
    type Vtable = IFadeOutThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89276ba9_ffd4_45b6_9b9a_ced48951e712);
}
impl ::windows::core::RuntimeName for FadeOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.FadeOutThemeAnimation";
}
impl ::core::convert::From<FadeOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: FadeOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: FadeOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for Timeline {
    fn from(value: FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for Timeline {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for FadeOutThemeAnimation {}
unsafe impl ::core::marker::Sync for FadeOutThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct FillBehavior(pub i32);
impl FillBehavior {
    pub const HoldEnd: Self = Self(0i32);
    pub const Stop: Self = Self(1i32);
}
impl ::core::marker::Copy for FillBehavior {}
impl ::core::clone::Clone for FillBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FillBehavior {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FillBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FillBehavior {}
impl ::core::fmt::Debug for FillBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FillBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FillBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.FillBehavior;i4)");
}
impl ::windows::core::DefaultType for FillBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct GravityConnectedAnimationConfiguration(::windows::core::IUnknown);
impl GravityConnectedAnimationConfiguration {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsShadowEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGravityConnectedAnimationConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsShadowEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGravityConnectedAnimationConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn new() -> ::windows::core::Result<GravityConnectedAnimationConfiguration> {
        Self::IGravityConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<GravityConnectedAnimationConfiguration>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGravityConnectedAnimationConfigurationFactory<R, F: FnOnce(&IGravityConnectedAnimationConfigurationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GravityConnectedAnimationConfiguration, IGravityConnectedAnimationConfigurationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GravityConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GravityConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GravityConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for GravityConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GravityConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GravityConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.GravityConnectedAnimationConfiguration;{c751a4b7-0459-5142-b891-aeaac1d41822})");
}
unsafe impl ::windows::core::Interface for GravityConnectedAnimationConfiguration {
    type Vtable = IGravityConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc751a4b7_0459_5142_b891_aeaac1d41822);
}
impl ::windows::core::RuntimeName for GravityConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.GravityConnectedAnimationConfiguration";
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for &GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for GravityConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for GravityConnectedAnimationConfiguration {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddDeleteThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAddDeleteThemeTransition {
    type Vtable = IAddDeleteThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadec852e_4424_4dab_99c1_3a04e36a3c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddDeleteThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackEase {
    type Vtable = IBackEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe47796e7_f805_4a8f_81c9_38e6472caa94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackEaseStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackEaseStatics {
    type Vtable = IBackEaseStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c70a2ff_a0a0_4786_926c_22321f8f25b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackEaseStaticsVtbl(
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
pub struct IBasicConnectedAnimationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBasicConnectedAnimationConfiguration {
    type Vtable = IBasicConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe675f9b5_a4d6_5353_83e6_c89e7cf8d456);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicConnectedAnimationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBasicConnectedAnimationConfigurationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBasicConnectedAnimationConfigurationFactory {
    type Vtable = IBasicConnectedAnimationConfigurationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95e6844a_4377_503c_bee2_11dfcd5570e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicConnectedAnimationConfigurationFactoryVtbl(
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
pub struct IBeginStoryboard(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBeginStoryboard {
    type Vtable = IBeginStoryboardVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64189fcd_49ec_4e52_a6f6_55324c921053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBeginStoryboardVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBeginStoryboardStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBeginStoryboardStatics {
    type Vtable = IBeginStoryboardStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12cff18c_aa91_4c4a_b82f_df34fc57f94b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBeginStoryboardStaticsVtbl(
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
pub struct IBounceEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBounceEase {
    type Vtable = IBounceEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bf1464e_fc71_47ed_85a1_3ba9577718b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBounceEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBounceEaseStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBounceEaseStatics {
    type Vtable = IBounceEaseStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0701da2_4f73_41c9_b2cb_2ea3105107ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBounceEaseStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICircleEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICircleEase {
    type Vtable = ICircleEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53a3bdb2_9177_4e6e_a043_5082d889ab1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICircleEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorAnimation {
    type Vtable = IColorAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8ae8a15_0f63_4694_9467_bdafac1253ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorAnimationStatics {
    type Vtable = IColorAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55eaf6e2_87e3_4f48_958f_855b2f9ea9ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorAnimationUsingKeyFrames(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorAnimationUsingKeyFrames {
    type Vtable = IColorAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c82640_13c3_42aa_9ae2_7e6b51c92f95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationUsingKeyFramesVtbl(
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
pub struct IColorAnimationUsingKeyFramesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorAnimationUsingKeyFramesStatics {
    type Vtable = IColorAnimationUsingKeyFramesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4723cdc_96e9_48f9_8d92_9b648b2f1cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationUsingKeyFramesStaticsVtbl(
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
pub struct IColorKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorKeyFrame {
    type Vtable = IColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb51d82d9_0910_4589_a284_b0c9205858e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorKeyFrameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorKeyFrameFactory {
    type Vtable = IColorKeyFrameFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x769bd88a_9cfb_4a7d_96c4_a1e7de6fdb4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrameFactoryVtbl(
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
pub struct IColorKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorKeyFrameStatics {
    type Vtable = IColorKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc043ae99_210c_430f_9da5_df1082692055);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrameStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommonNavigationTransitionInfo {
    type Vtable = ICommonNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50345692_a555_4624_a361_0a91c1706473);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommonNavigationTransitionInfoStatics {
    type Vtable = ICommonNavigationTransitionInfoStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3efe33_50be_4443_883c_e5627201c2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfoStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectedAnimation {
    type Vtable = IConnectedAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3518628c_f387_4c25_ac98_44e86c3cadf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectedAnimation2 {
    type Vtable = IConnectedAnimation2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2f8e5c_584b_4ddd_b668_973891431459);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, coordinatedelements: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, component: ConnectedAnimationComponent, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectedAnimation3 {
    type Vtable = IConnectedAnimation3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e3040c6_0430_59c0_a80c_cceed2e778dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectedAnimationConfiguration {
    type Vtable = IConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00218aae_cd8c_5651_92a0_c1db95c03998);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationConfigurationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectedAnimationConfigurationFactory {
    type Vtable = IConnectedAnimationConfigurationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f9b84b_dd7e_593e_bf75_e959dc0ec52a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationConfigurationFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectedAnimationService {
    type Vtable = IConnectedAnimationServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c6875c9_19bb_4d47_b9aa_66c802dcb9ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationServiceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectedAnimationServiceStatics {
    type Vtable = IConnectedAnimationServiceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7078ea5_d688_40e8_8f90_96a6279273d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationServiceStaticsVtbl(
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
pub struct IContentThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentThemeTransition {
    type Vtable = IContentThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf66fc5c3_5915_437d_8e3b_adf8e7f0ab57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentThemeTransitionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentThemeTransitionStatics {
    type Vtable = IContentThemeTransitionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e8ee385_9a42_4459_afa9_337dc41e1587);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentThemeTransitionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContinuumNavigationTransitionInfo {
    type Vtable = IContinuumNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4be1dbad_8ba6_4004_8438_8a9017978543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContinuumNavigationTransitionInfoStatics {
    type Vtable = IContinuumNavigationTransitionInfoStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e25dd53_b18f_4bf1_b3bc_92f516f29903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfoStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICubicEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICubicEase {
    type Vtable = ICubicEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b94fc76_dad7_4354_b1a2_7969fbf6a70d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICubicEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDirectConnectedAnimationConfiguration {
    type Vtable = IDirectConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5d736f_5738_5d86_b770_151948cf365e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfigurationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDirectConnectedAnimationConfigurationFactory {
    type Vtable = IDirectConnectedAnimationConfigurationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x059263e9_d2b3_5a77_9cf4_e26d8b542608);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfigurationFactoryVtbl(
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
pub struct IDiscreteColorKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDiscreteColorKeyFrame {
    type Vtable = IDiscreteColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230c08f4_e062_4cb1_8e2a_14093d73ed8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteColorKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiscreteDoubleKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDiscreteDoubleKeyFrame {
    type Vtable = IDiscreteDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f51f3a_ad11_49ce_8e1c_08fdf1447446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteDoubleKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiscreteObjectKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDiscreteObjectKeyFrame {
    type Vtable = IDiscreteObjectKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7dcde89_f12d_4a9c_8199_e7a9ece3a473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteObjectKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiscretePointKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDiscretePointKeyFrame {
    type Vtable = IDiscretePointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0a9070d_4c42_4a90_983a_75f5a83a2fbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscretePointKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleAnimation {
    type Vtable = IDoubleAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e9f3d59_0f07_4bc9_977d_03763ff8154f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleAnimationStatics {
    type Vtable = IDoubleAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe27a935d_f111_43b7_b824_832b58d7786b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleAnimationUsingKeyFrames(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleAnimationUsingKeyFrames {
    type Vtable = IDoubleAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fee628f_bfee_4f75_83c2_a93b39488473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationUsingKeyFramesVtbl(
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
pub struct IDoubleAnimationUsingKeyFramesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleAnimationUsingKeyFramesStatics {
    type Vtable = IDoubleAnimationUsingKeyFramesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x109bf2f6_c60f_49aa_abf6_f696d492116b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationUsingKeyFramesStaticsVtbl(
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
pub struct IDoubleKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleKeyFrame {
    type Vtable = IDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674456fd_e81e_4f4e_b4ad_0acfed9ecd68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleKeyFrameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleKeyFrameFactory {
    type Vtable = IDoubleKeyFrameFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac97dec3_7538_40b9_b152_696f7fbf4722);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrameFactoryVtbl(
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
pub struct IDoubleKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleKeyFrameStatics {
    type Vtable = IDoubleKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x324641b0_7d37_427a_adeb_43f38bb61a4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrameStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragItemThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDragItemThemeAnimation {
    type Vtable = IDragItemThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c7d5db5_7ed6_4949_b4e6_a78c9f4f978d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragItemThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragItemThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDragItemThemeAnimationStatics {
    type Vtable = IDragItemThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6218b9f5_013a_4fb1_86fc_92bc4e8d0241);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragItemThemeAnimationStaticsVtbl(
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
pub struct IDragOverThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDragOverThemeAnimation {
    type Vtable = IDragOverThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72f762f7_7e51_4a6b_b937_dc4b4c1c5458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOverThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragOverThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDragOverThemeAnimationStatics {
    type Vtable = IDragOverThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146ffe57_3c9d_41d9_a5ff_8d7239516810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOverThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillInNavigationTransitionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDrillInNavigationTransitionInfo {
    type Vtable = IDrillInNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b86201a_45d3_463b_939e_c8595f439bcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInNavigationTransitionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillInThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDrillInThemeAnimation {
    type Vtable = IDrillInThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb090b824_f1d2_41b8_87ba_78034126594c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillInThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDrillInThemeAnimationStatics {
    type Vtable = IDrillInThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61fe488_a17a_4b11_b53b_a4f1a07d4ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillOutThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDrillOutThemeAnimation {
    type Vtable = IDrillOutThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd890ccdf_06d3_4f7e_8e4a_4fb76e256139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillOutThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDrillOutThemeAnimationStatics {
    type Vtable = IDrillOutThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb5db9b_2617_4888_80dd_72fa7bb6fac3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDropTargetItemThemeAnimation {
    type Vtable = IDropTargetItemThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1881c968_1824_462b_87e8_c357212b977b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDropTargetItemThemeAnimationStatics {
    type Vtable = IDropTargetItemThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae80f486_2e56_4513_bf18_d77470164ae5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimationStaticsVtbl(
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
pub struct IEasingColorKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingColorKeyFrame {
    type Vtable = IEasingColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc733d630_f4b9_4934_9bdd_27ac5ed1cfd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingColorKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingColorKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingColorKeyFrameStatics {
    type Vtable = IEasingColorKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f3837fc_8e3d_4522_9b0f_003db8609851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingColorKeyFrameStaticsVtbl(
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
pub struct IEasingDoubleKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingDoubleKeyFrame {
    type Vtable = IEasingDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965adb8d_9a54_4108_b4ff_b5a5212cb338);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingDoubleKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingDoubleKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingDoubleKeyFrameStatics {
    type Vtable = IEasingDoubleKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8d3d845_dbae_4e5b_8b84_d9537398e5b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingDoubleKeyFrameStaticsVtbl(
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
pub struct IEasingFunctionBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingFunctionBase {
    type Vtable = IEasingFunctionBaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc108383f_2c02_4151_8ecd_68ddaa3f0d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EasingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, normalizedtime: f64, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingFunctionBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingFunctionBaseFactory {
    type Vtable = IEasingFunctionBaseFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1830fe6a_f01b_43e0_b61f_b452a1c66fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBaseFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingFunctionBaseStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingFunctionBaseStatics {
    type Vtable = IEasingFunctionBaseStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a5031aa_2c50_4a1d_bb04_d75e07b71548);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBaseStaticsVtbl(
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
pub struct IEasingPointKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingPointKeyFrame {
    type Vtable = IEasingPointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c91380_6868_4225_a70b_3981cc0b2947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingPointKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingPointKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasingPointKeyFrameStatics {
    type Vtable = IEasingPointKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe22dbfc4_080c_402c_a6b5_f48d0a98116b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingPointKeyFrameStaticsVtbl(
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
pub struct IEdgeUIThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEdgeUIThemeTransition {
    type Vtable = IEdgeUIThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c86c19b_49d7_19ec_cf19_83a73c6de75e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeUIThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeUIThemeTransitionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEdgeUIThemeTransitionStatics {
    type Vtable = IEdgeUIThemeTransitionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a2b13b_4705_302b_27c6_2aac92f645ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeUIThemeTransitionStaticsVtbl(
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
pub struct IElasticEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElasticEase {
    type Vtable = IElasticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5ba58c_b0b6_4a6c_9ca8_fb4233f12459);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElasticEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IElasticEaseStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElasticEaseStatics {
    type Vtable = IElasticEaseStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9f566ec_fe9c_4b2b_8e52_bb785d562185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElasticEaseStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEntranceNavigationTransitionInfo {
    type Vtable = IEntranceNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x720a256b_1c8a_41ee_82ec_8a87c0cf47da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEntranceNavigationTransitionInfoStatics {
    type Vtable = IEntranceNavigationTransitionInfoStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf948c27a_40c9_469f_8f33_bf45c8811f21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfoStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEntranceThemeTransition {
    type Vtable = IEntranceThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07698c09_a8e3_419a_a01d_7410a0ae8ec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceThemeTransitionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEntranceThemeTransitionStatics {
    type Vtable = IEntranceThemeTransitionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37cc0577_ff98_4aed_b86e_5ec23702f877);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceThemeTransitionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IExponentialEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExponentialEase {
    type Vtable = IExponentialEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cb9e41d_f0bb_4bca_9da5_9ba3a11734c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExponentialEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IExponentialEaseStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExponentialEaseStatics {
    type Vtable = IExponentialEaseStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf37ee7e3_a761_4352_9ad6_70794567581a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExponentialEaseStaticsVtbl(
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
pub struct IFadeInThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFadeInThemeAnimation {
    type Vtable = IFadeInThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4bc8f5_a918_4477_8078_554c68812ab8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeInThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFadeInThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFadeInThemeAnimationStatics {
    type Vtable = IFadeInThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f0117e1_bea9_4923_b23a_0ddf4d7b8737);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeInThemeAnimationStaticsVtbl(
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
pub struct IFadeOutThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFadeOutThemeAnimation {
    type Vtable = IFadeOutThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89276ba9_ffd4_45b6_9b9a_ced48951e712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeOutThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFadeOutThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFadeOutThemeAnimationStatics {
    type Vtable = IFadeOutThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe17a81a_4168_4f68_a28c_e5dd98cf680f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeOutThemeAnimationStaticsVtbl(
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
pub struct IGravityConnectedAnimationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGravityConnectedAnimationConfiguration {
    type Vtable = IGravityConnectedAnimationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc751a4b7_0459_5142_b891_aeaac1d41822);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGravityConnectedAnimationConfiguration2 {
    type Vtable = IGravityConnectedAnimationConfiguration2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62333add_aed4_5fed_95ff_d128acce8be4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfiguration2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfigurationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGravityConnectedAnimationConfigurationFactory {
    type Vtable = IGravityConnectedAnimationConfigurationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe822c41f_3656_5090_92f5_c217eaacb682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfigurationFactoryVtbl(
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
pub struct IKeySpline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeySpline {
    type Vtable = IKeySplineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a163bb_d5ca_4a32_ba0b_7dff988e58a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeySplineVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyTimeHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyTimeHelper {
    type Vtable = IKeyTimeHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3643e480_4823_466a_abe5_5e79c8ed77ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyTimeHelperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyTimeHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyTimeHelperStatics {
    type Vtable = IKeyTimeHelperStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa2612c_22a9_45e9_9af7_c7416efff7a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyTimeHelperStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timespan: super::super::super::super::Foundation::TimeSpan, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearColorKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILinearColorKeyFrame {
    type Vtable = ILinearColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66fdb6ef_ac81_4611_b1d2_61f545983f03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearColorKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearDoubleKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILinearDoubleKeyFrame {
    type Vtable = ILinearDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efdf265_9a7b_431d_8f0c_14c56b5ea4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearDoubleKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearPointKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILinearPointKeyFrame {
    type Vtable = ILinearPointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7c9b8ef_af24_49ee_84f1_a86600a4e319);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearPointKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationThemeTransition {
    type Vtable = INavigationThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8833848c_4eb7_41f2_8799_9eef0a213b73);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationThemeTransitionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationThemeTransitionStatics {
    type Vtable = INavigationThemeTransitionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea2f06e0_5e60_4f8e_bcaf_431487a294ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationThemeTransitionStaticsVtbl(
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
pub struct INavigationTransitionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationTransitionInfo {
    type Vtable = INavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b05091_ae4a_4372_8625_21b7a8b98ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationTransitionInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationTransitionInfoFactory {
    type Vtable = INavigationTransitionInfoFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedf4f8d5_af63_4fab_9d4a_87927f82dd6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfoFactoryVtbl(
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
pub struct INavigationTransitionInfoOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationTransitionInfoOverrides {
    type Vtable = INavigationTransitionInfoOverridesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9517e6a_a9d0_4bf7_9db0_4633a69daff2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfoOverridesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, navigationstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IObjectAnimationUsingKeyFrames(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IObjectAnimationUsingKeyFrames {
    type Vtable = IObjectAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334a2d92_b74a_4c64_b9a6_58bcfa314f22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectAnimationUsingKeyFramesVtbl(
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
pub struct IObjectAnimationUsingKeyFramesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IObjectAnimationUsingKeyFramesStatics {
    type Vtable = IObjectAnimationUsingKeyFramesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb736182_6af1_49a3_97b6_783ed97400fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectAnimationUsingKeyFramesStaticsVtbl(
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
pub struct IObjectKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IObjectKeyFrame {
    type Vtable = IObjectKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9852a851_8593_48ee_a6a4_d5d4720f029a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IObjectKeyFrameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IObjectKeyFrameFactory {
    type Vtable = IObjectKeyFrameFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1626143e_3e6d_44d8_9b9a_04aea70f8492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrameFactoryVtbl(
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
pub struct IObjectKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IObjectKeyFrameStatics {
    type Vtable = IObjectKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd6ab00_5319_4286_8eed_4e755ea0cf9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrameStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaneThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaneThemeTransition {
    type Vtable = IPaneThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4708eb8e_4bfc_ee46_d4f9_708def3fbb2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaneThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaneThemeTransitionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaneThemeTransitionStatics {
    type Vtable = IPaneThemeTransitionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x316b382f_4be4_1797_b45c_cd900bbe0caa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaneThemeTransitionStaticsVtbl(
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
pub struct IPointAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointAnimation {
    type Vtable = IPointAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f04312_7726_4f88_b8e2_2fa54518963b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointAnimationStatics {
    type Vtable = IPointAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f99b356_e737_408b_a0fd_327826d32255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointAnimationUsingKeyFrames(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointAnimationUsingKeyFrames {
    type Vtable = IPointAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b944f72_446a_41d0_a129_41a620f4595d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationUsingKeyFramesVtbl(
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
pub struct IPointAnimationUsingKeyFramesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointAnimationUsingKeyFramesStatics {
    type Vtable = IPointAnimationUsingKeyFramesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f454c87_2390_46ea_baa7_762f4bc30d04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationUsingKeyFramesStaticsVtbl(
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
pub struct IPointKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointKeyFrame {
    type Vtable = IPointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcc88d01_7f82_4dae_8026_7b7e086878b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointKeyFrameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointKeyFrameFactory {
    type Vtable = IPointKeyFrameFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb214bdf_426a_4392_8355_c2ae52852623);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrameFactoryVtbl(
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
pub struct IPointKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointKeyFrameStatics {
    type Vtable = IPointKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95cf1b27_7965_4bec_b9fb_fbe94b65518e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrameStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDownThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerDownThemeAnimation {
    type Vtable = IPointerDownThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb58e714e_c49d_4788_a233_0ae85d99dd5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDownThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerDownThemeAnimationStatics {
    type Vtable = IPointerDownThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a7cb7b_6d46_4494_b94a_e72f3b492a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimationStaticsVtbl(
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
pub struct IPointerUpThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerUpThemeAnimation {
    type Vtable = IPointerUpThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9e9d07d_6340_4828_ad12_690694b9910b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerUpThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerUpThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerUpThemeAnimationStatics {
    type Vtable = IPointerUpThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c618f9c_7992_4139_8bfc_0883b9727a7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerUpThemeAnimationStaticsVtbl(
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
pub struct IPopInThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopInThemeAnimation {
    type Vtable = IPopInThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196938c1_1c07_4c28_8847_f9f055b32855);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopInThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopInThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopInThemeAnimationStatics {
    type Vtable = IPopInThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefaa99d3_218a_4701_977f_f1bfae8ba649);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopInThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopOutThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopOutThemeAnimation {
    type Vtable = IPopOutThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4786ab49_0e48_4e81_a2e5_cc5aa19e48d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopOutThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopOutThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopOutThemeAnimationStatics {
    type Vtable = IPopOutThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d492c09_03c1_4490_99dc_909feab357fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopOutThemeAnimationStaticsVtbl(
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
pub struct IPopupThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopupThemeTransition {
    type Vtable = IPopupThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47843552_4283_545e_c791_268dca22ce4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupThemeTransitionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopupThemeTransitionStatics {
    type Vtable = IPopupThemeTransitionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5a1640e_490d_1505_9f6b_8fafc044dec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupThemeTransitionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerEase {
    type Vtable = IPowerEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c80579_eedf_405b_8680_d9606880c937);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerEaseStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerEaseStatics {
    type Vtable = IPowerEaseStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5955103_91a2_460c_9c41_d28f6a939bda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerEaseStaticsVtbl(
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
pub struct IQuadraticEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IQuadraticEase {
    type Vtable = IQuadraticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1510e91_ef6d_44f0_803d_68d16de0ddfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuarticEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IQuarticEase {
    type Vtable = IQuarticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8698814_fe42_4a05_b5b8_081f41157815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuarticEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuinticEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IQuinticEase {
    type Vtable = IQuinticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ee793b_3c49_4108_aa11_ab786603da21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuinticEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IReorderThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReorderThemeTransition {
    type Vtable = IReorderThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2065c6c_d052_4ad1_8362_b71b36df7497);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReorderThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepeatBehaviorHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRepeatBehaviorHelper {
    type Vtable = IRepeatBehaviorHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6863ab72_4997_47f9_87ad_37efb75993ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepeatBehaviorHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRepeatBehaviorHelperStatics {
    type Vtable = IRepeatBehaviorHelperStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a795033_79f3_4dd9_b267_9cf50fb51f84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelperStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: f64, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: super::super::super::super::Foundation::TimeSpan, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: RepeatBehavior, value: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRepositionThemeAnimation {
    type Vtable = IRepositionThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecda24e8_8945_4949_a1bf_62109965a7e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRepositionThemeAnimationStatics {
    type Vtable = IRepositionThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d92b1b1_860b_4bf9_a59d_1eb1ccbe8fe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeTransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRepositionThemeTransition {
    type Vtable = IRepositionThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88329b82_98f3_455a_ac53_2e7083b6e22c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeTransition2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRepositionThemeTransition2 {
    type Vtable = IRepositionThemeTransition2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcebfe864_dbea_4404_8e6e_de55ada75239);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransition2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeTransitionStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRepositionThemeTransitionStatics2 {
    type Vtable = IRepositionThemeTransitionStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9240e930_0a19_468b_8c2a_68fab4500027);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransitionStatics2Vtbl(
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
pub struct ISineEase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISineEase {
    type Vtable = ISineEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9382962_230b_49da_9e0d_664987892343);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISineEaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISlideNavigationTransitionInfo {
    type Vtable = ISlideNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6ac9d77_2e03_405f_80ed_e62beef3668f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISlideNavigationTransitionInfo2 {
    type Vtable = ISlideNavigationTransitionInfo2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90e2d9c0_5c81_5001_8013_4fbfea4bf139);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfo2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SlideNavigationTransitionEffect) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SlideNavigationTransitionEffect) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfoStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISlideNavigationTransitionInfoStatics2 {
    type Vtable = ISlideNavigationTransitionInfoStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a861baa_981a_5ace_9f85_cb7fde648a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfoStatics2Vtbl(
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
pub struct ISplineColorKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplineColorKeyFrame {
    type Vtable = ISplineColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4a5941_1fe0_473a_8efe_4316d8c86229);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineColorKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplineColorKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplineColorKeyFrameStatics {
    type Vtable = ISplineColorKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61d1d997_8589_4f2f_8fbb_7d03edc98dd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineColorKeyFrameStaticsVtbl(
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
pub struct ISplineDoubleKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplineDoubleKeyFrame {
    type Vtable = ISplineDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00d72d38_6b2b_4843_838e_c8b115eec801);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineDoubleKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplineDoubleKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplineDoubleKeyFrameStatics {
    type Vtable = ISplineDoubleKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060a8ffc_975f_4e4e_9ec7_13c5aee02062);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineDoubleKeyFrameStaticsVtbl(
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
pub struct ISplinePointKeyFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplinePointKeyFrame {
    type Vtable = ISplinePointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f19f306_7036_494f_bc3c_780df0cc524a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplinePointKeyFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplinePointKeyFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplinePointKeyFrameStatics {
    type Vtable = ISplinePointKeyFrameStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe97a32c2_0a7a_4766_95cb_0d692611cb4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplinePointKeyFrameStaticsVtbl(
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
pub struct ISplitCloseThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplitCloseThemeAnimation {
    type Vtable = ISplitCloseThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f799518_ff39_4e90_bb74_2abd56027402);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitCloseThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitCloseThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplitCloseThemeAnimationStatics {
    type Vtable = ISplitCloseThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7aa94de9_cc9b_4e90_a11a_0050a2216a9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitCloseThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitOpenThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplitOpenThemeAnimation {
    type Vtable = ISplitOpenThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x785fd7aa_5456_4639_8fd2_26bae6a5ffe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitOpenThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplitOpenThemeAnimationStatics {
    type Vtable = ISplitOpenThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d4cfa89_3a91_458d_b0fb_4cad625cbf8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoryboard(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoryboard {
    type Vtable = IStoryboardVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45c1e6e_3594_460e_981a_32271bd3aa06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoryboardVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClockState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoryboardStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoryboardStatics {
    type Vtable = IStoryboardStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd82f07d8_73d5_4379_bd48_7e05184a8bad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoryboardStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeline: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISuppressNavigationTransitionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISuppressNavigationTransitionInfo {
    type Vtable = ISuppressNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x244d7b0c_b1b7_4871_9d3e_d56203a3a5b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuppressNavigationTransitionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeBackThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISwipeBackThemeAnimation {
    type Vtable = ISwipeBackThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa38a4214_0bca_4d2d_95f7_ceba57fbaf60);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeBackThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISwipeBackThemeAnimationStatics {
    type Vtable = ISwipeBackThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x693f31bf_4da6_468a_8ce0_996c9aad42e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeHintThemeAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISwipeHintThemeAnimation {
    type Vtable = ISwipeHintThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdd067c0_580e_4e40_be98_f202d3d84365);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeHintThemeAnimationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISwipeHintThemeAnimationStatics {
    type Vtable = ISwipeHintThemeAnimationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23d61a57_9115_4d63_b04a_b89f1c744dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimeline {
    type Vtable = ITimelineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc465dc_be4d_4d0d_9549_2208b715f40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Duration) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Duration) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FillBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FillBehavior) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimelineFactory {
    type Vtable = ITimelineFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d56bb07_bda4_478b_8ada_eb04d580cd5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineFactoryVtbl(
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
pub struct ITimelineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimelineStatics {
    type Vtable = ITimelineStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa902ed4e_ef10_4d6f_9a40_93cb8895f4e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransition {
    type Vtable = ITransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c677c7c_01d0_4dce_b333_976f93312b08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransitionFactory {
    type Vtable = ITransitionFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc9ab2cf_3bc9_44aa_b3fc_883a83233a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransitionFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct KeySpline(::windows::core::IUnknown);
impl KeySpline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeySpline, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ControlPoint1(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetControlPoint1<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ControlPoint2(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetControlPoint2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for KeySpline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeySpline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeySpline {}
impl ::core::fmt::Debug for KeySpline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeySpline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeySpline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.KeySpline;{77a163bb-d5ca-4a32-ba0b-7dff988e58a0})");
}
unsafe impl ::windows::core::Interface for KeySpline {
    type Vtable = IKeySplineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a163bb_d5ca_4a32_ba0b_7dff988e58a0);
}
impl ::windows::core::RuntimeName for KeySpline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.KeySpline";
}
impl ::core::convert::From<KeySpline> for ::windows::core::IUnknown {
    fn from(value: KeySpline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeySpline> for ::windows::core::IUnknown {
    fn from(value: &KeySpline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeySpline> for ::windows::core::IInspectable {
    fn from(value: KeySpline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeySpline> for ::windows::core::IInspectable {
    fn from(value: &KeySpline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeySpline> for super::super::DependencyObject {
    fn from(value: KeySpline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&KeySpline> for super::super::DependencyObject {
    fn from(value: &KeySpline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for KeySpline {}
unsafe impl ::core::marker::Sync for KeySpline {}
#[repr(C)]
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
#[cfg(feature = "Foundation")]
pub struct KeyTime {
    pub TimeSpan: super::super::super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for KeyTime {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for KeyTime {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for KeyTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KeyTime").field("TimeSpan", &self.TimeSpan).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for KeyTime {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for KeyTime {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Animation.KeyTime;struct(Windows.Foundation.TimeSpan;i8))");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for KeyTime {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for KeyTime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KeyTime>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for KeyTime {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for KeyTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct KeyTimeHelper(::windows::core::IUnknown);
impl KeyTimeHelper {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(timespan: Param0) -> ::windows::core::Result<KeyTime> {
        Self::IKeyTimeHelperStatics(|this| unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timespan.into_param().abi(), &mut result__).from_abi::<KeyTime>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyTimeHelperStatics<R, F: FnOnce(&IKeyTimeHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyTimeHelper, IKeyTimeHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyTimeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyTimeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyTimeHelper {}
impl ::core::fmt::Debug for KeyTimeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyTimeHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyTimeHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.KeyTimeHelper;{3643e480-4823-466a-abe5-5e79c8ed77ed})");
}
unsafe impl ::windows::core::Interface for KeyTimeHelper {
    type Vtable = IKeyTimeHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3643e480_4823_466a_abe5_5e79c8ed77ed);
}
impl ::windows::core::RuntimeName for KeyTimeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.KeyTimeHelper";
}
impl ::core::convert::From<KeyTimeHelper> for ::windows::core::IUnknown {
    fn from(value: KeyTimeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyTimeHelper> for ::windows::core::IUnknown {
    fn from(value: &KeyTimeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyTimeHelper> for ::windows::core::IInspectable {
    fn from(value: KeyTimeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyTimeHelper> for ::windows::core::IInspectable {
    fn from(value: &KeyTimeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyTimeHelper {}
unsafe impl ::core::marker::Sync for KeyTimeHelper {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct LinearColorKeyFrame(::windows::core::IUnknown);
impl LinearColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LinearColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LinearColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearColorKeyFrame {}
impl ::core::fmt::Debug for LinearColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LinearColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearColorKeyFrame;{66fdb6ef-ac81-4611-b1d2-61f545983f03})");
}
unsafe impl ::windows::core::Interface for LinearColorKeyFrame {
    type Vtable = ILinearColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66fdb6ef_ac81_4611_b1d2_61f545983f03);
}
impl ::windows::core::RuntimeName for LinearColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearColorKeyFrame";
}
impl ::core::convert::From<LinearColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: LinearColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: LinearColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for ColorKeyFrame {
    fn from(value: LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ColorKeyFrame {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LinearColorKeyFrame {}
unsafe impl ::core::marker::Sync for LinearColorKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct LinearDoubleKeyFrame(::windows::core::IUnknown);
impl LinearDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LinearDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LinearDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearDoubleKeyFrame {}
impl ::core::fmt::Debug for LinearDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LinearDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearDoubleKeyFrame;{8efdf265-9a7b-431d-8f0c-14c56b5ea4d9})");
}
unsafe impl ::windows::core::Interface for LinearDoubleKeyFrame {
    type Vtable = ILinearDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efdf265_9a7b_431d_8f0c_14c56b5ea4d9);
}
impl ::windows::core::RuntimeName for LinearDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearDoubleKeyFrame";
}
impl ::core::convert::From<LinearDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LinearDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for LinearDoubleKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct LinearPointKeyFrame(::windows::core::IUnknown);
impl LinearPointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearPointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LinearPointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LinearPointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearPointKeyFrame {}
impl ::core::fmt::Debug for LinearPointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearPointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LinearPointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearPointKeyFrame;{e7c9b8ef-af24-49ee-84f1-a86600a4e319})");
}
unsafe impl ::windows::core::Interface for LinearPointKeyFrame {
    type Vtable = ILinearPointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7c9b8ef_af24_49ee_84f1_a86600a4e319);
}
impl ::windows::core::RuntimeName for LinearPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearPointKeyFrame";
}
impl ::core::convert::From<LinearPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: LinearPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: LinearPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for PointKeyFrame {
    fn from(value: LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for PointKeyFrame {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LinearPointKeyFrame {}
unsafe impl ::core::marker::Sync for LinearPointKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct NavigationThemeTransition(::windows::core::IUnknown);
impl NavigationThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NavigationThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn DefaultNavigationTransitionInfo(&self) -> ::windows::core::Result<NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NavigationTransitionInfo>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetDefaultNavigationTransitionInfo<'a, Param0: ::windows::core::IntoParam<'a, NavigationTransitionInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn DefaultNavigationTransitionInfoProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::INavigationThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INavigationThemeTransitionStatics<R, F: FnOnce(&INavigationThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NavigationThemeTransition, INavigationThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NavigationThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationThemeTransition {}
impl ::core::fmt::Debug for NavigationThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.NavigationThemeTransition;{8833848c-4eb7-41f2-8799-9eef0a213b73})");
}
unsafe impl ::windows::core::Interface for NavigationThemeTransition {
    type Vtable = INavigationThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8833848c_4eb7_41f2_8799_9eef0a213b73);
}
impl ::windows::core::RuntimeName for NavigationThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.NavigationThemeTransition";
}
impl ::core::convert::From<NavigationThemeTransition> for ::windows::core::IUnknown {
    fn from(value: NavigationThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationThemeTransition> for ::windows::core::IInspectable {
    fn from(value: NavigationThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationThemeTransition> for Transition {
    fn from(value: NavigationThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for Transition {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<NavigationThemeTransition> for super::super::DependencyObject {
    fn from(value: NavigationThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for super::super::DependencyObject {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for NavigationThemeTransition {}
unsafe impl ::core::marker::Sync for NavigationThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct NavigationTransitionInfo(::windows::core::IUnknown);
impl NavigationTransitionInfo {}
impl ::core::clone::Clone for NavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationTransitionInfo {}
impl ::core::fmt::Debug for NavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.NavigationTransitionInfo;{a9b05091-ae4a-4372-8625-21b7a8b98ca4})");
}
unsafe impl ::windows::core::Interface for NavigationTransitionInfo {
    type Vtable = INavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b05091_ae4a_4372_8625_21b7a8b98ca4);
}
impl ::windows::core::RuntimeName for NavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.NavigationTransitionInfo";
}
impl ::core::convert::From<NavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: NavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &NavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: NavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &NavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: NavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &NavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for NavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for NavigationTransitionInfo {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ObjectAnimationUsingKeyFrames(::windows::core::IUnknown);
impl ObjectAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<ObjectKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ObjectKeyFrameCollection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IObjectAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IObjectAnimationUsingKeyFramesStatics<R, F: FnOnce(&IObjectAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectAnimationUsingKeyFrames, IObjectAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ObjectAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ObjectAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ObjectAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for ObjectAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ObjectAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectAnimationUsingKeyFrames;{334a2d92-b74a-4c64-b9a6-58bcfa314f22})");
}
unsafe impl ::windows::core::Interface for ObjectAnimationUsingKeyFrames {
    type Vtable = IObjectAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334a2d92_b74a_4c64_b9a6_58bcfa314f22);
}
impl ::windows::core::RuntimeName for ObjectAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectAnimationUsingKeyFrames";
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for Timeline {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for Timeline {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ObjectAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for ObjectAnimationUsingKeyFrames {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ObjectKeyFrame(::windows::core::IUnknown);
impl ObjectKeyFrame {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IObjectKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IObjectKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IObjectKeyFrameStatics<R, F: FnOnce(&IObjectKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectKeyFrame, IObjectKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ObjectKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ObjectKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ObjectKeyFrame {}
impl ::core::fmt::Debug for ObjectKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ObjectKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrame;{9852a851-8593-48ee-a6a4-d5d4720f029a})");
}
unsafe impl ::windows::core::Interface for ObjectKeyFrame {
    type Vtable = IObjectKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9852a851_8593_48ee_a6a4_d5d4720f029a);
}
impl ::windows::core::RuntimeName for ObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectKeyFrame";
}
impl ::core::convert::From<ObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: ObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &ObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: ObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &ObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: ObjectKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: &ObjectKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ObjectKeyFrame {}
unsafe impl ::core::marker::Sync for ObjectKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ObjectKeyFrameCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ObjectKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<ObjectKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<ObjectKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ObjectKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ObjectKeyFrame>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<ObjectKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<ObjectKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ObjectKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<ObjectKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ObjectKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ObjectKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ObjectKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ObjectKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ObjectKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrame;{9852a851-8593-48ee-a6a4-d5d4720f029a})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ObjectKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVectorVtbl<ObjectKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ObjectKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ObjectKeyFrameCollection {
    type Item = ObjectKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ObjectKeyFrameCollection {
    type Item = ObjectKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ObjectKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: ObjectKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ObjectKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &ObjectKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ObjectKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: ObjectKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ObjectKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &ObjectKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ObjectKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: ObjectKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ObjectKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ObjectKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame>> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame>> for &ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ObjectKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: ObjectKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ObjectKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ObjectKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> for &ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ObjectKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ObjectKeyFrameCollection {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PaneThemeTransition(::windows::core::IUnknown);
impl PaneThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaneThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::EdgeTransitionLocation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::EdgeTransitionLocation>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EdgeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPaneThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaneThemeTransitionStatics<R, F: FnOnce(&IPaneThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaneThemeTransition, IPaneThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaneThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaneThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaneThemeTransition {}
impl ::core::fmt::Debug for PaneThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaneThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaneThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PaneThemeTransition;{4708eb8e-4bfc-ee46-d4f9-708def3fbb2b})");
}
unsafe impl ::windows::core::Interface for PaneThemeTransition {
    type Vtable = IPaneThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4708eb8e_4bfc_ee46_d4f9_708def3fbb2b);
}
impl ::windows::core::RuntimeName for PaneThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PaneThemeTransition";
}
impl ::core::convert::From<PaneThemeTransition> for ::windows::core::IUnknown {
    fn from(value: PaneThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaneThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &PaneThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaneThemeTransition> for ::windows::core::IInspectable {
    fn from(value: PaneThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaneThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &PaneThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaneThemeTransition> for Transition {
    fn from(value: PaneThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PaneThemeTransition> for Transition {
    fn from(value: &PaneThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<PaneThemeTransition> for super::super::DependencyObject {
    fn from(value: PaneThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PaneThemeTransition> for super::super::DependencyObject {
    fn from(value: &PaneThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PaneThemeTransition {}
unsafe impl ::core::marker::Sync for PaneThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PointAnimation(::windows::core::IUnknown);
impl PointAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ByProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointAnimationStatics<R, F: FnOnce(&IPointAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimation, IPointAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointAnimation {}
impl ::core::fmt::Debug for PointAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointAnimation;{30f04312-7726-4f88-b8e2-2fa54518963b})");
}
unsafe impl ::windows::core::Interface for PointAnimation {
    type Vtable = IPointAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f04312_7726_4f88_b8e2_2fa54518963b);
}
impl ::windows::core::RuntimeName for PointAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointAnimation";
}
impl ::core::convert::From<PointAnimation> for ::windows::core::IUnknown {
    fn from(value: PointAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimation> for ::windows::core::IUnknown {
    fn from(value: &PointAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimation> for ::windows::core::IInspectable {
    fn from(value: PointAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimation> for ::windows::core::IInspectable {
    fn from(value: &PointAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimation> for Timeline {
    fn from(value: PointAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimation> for Timeline {
    fn from(value: &PointAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointAnimation> for super::super::DependencyObject {
    fn from(value: PointAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimation> for super::super::DependencyObject {
    fn from(value: &PointAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointAnimation {}
unsafe impl ::core::marker::Sync for PointAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PointAnimationUsingKeyFrames(::windows::core::IUnknown);
impl PointAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<PointKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointKeyFrameCollection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointAnimationUsingKeyFramesStatics<R, F: FnOnce(&IPointAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimationUsingKeyFrames, IPointAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for PointAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointAnimationUsingKeyFrames;{9b944f72-446a-41d0-a129-41a620f4595d})");
}
unsafe impl ::windows::core::Interface for PointAnimationUsingKeyFrames {
    type Vtable = IPointAnimationUsingKeyFramesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b944f72_446a_41d0_a129_41a620f4595d);
}
impl ::windows::core::RuntimeName for PointAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointAnimationUsingKeyFrames";
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for Timeline {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for Timeline {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for PointAnimationUsingKeyFrames {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PointKeyFrame(::windows::core::IUnknown);
impl PointKeyFrame {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointKeyFrameStatics<R, F: FnOnce(&IPointKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointKeyFrame, IPointKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointKeyFrame {}
impl ::core::fmt::Debug for PointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointKeyFrame;{fcc88d01-7f82-4dae-8026-7b7e086878b3})");
}
unsafe impl ::windows::core::Interface for PointKeyFrame {
    type Vtable = IPointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcc88d01_7f82_4dae_8026_7b7e086878b3);
}
impl ::windows::core::RuntimeName for PointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointKeyFrame";
}
impl ::core::convert::From<PointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: PointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &PointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: PointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &PointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointKeyFrame> for super::super::DependencyObject {
    fn from(value: PointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointKeyFrame> for super::super::DependencyObject {
    fn from(value: &PointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointKeyFrame {}
unsafe impl ::core::marker::Sync for PointKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PointKeyFrameCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PointKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<PointKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<PointKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<PointKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<PointKeyFrame>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<PointKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<PointKeyFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<PointKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<PointKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PointKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PointKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PointKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PointKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PointKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.PointKeyFrame;{fcc88d01-7f82-4dae-8026-7b7e086878b3})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PointKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVectorVtbl<PointKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PointKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PointKeyFrameCollection {
    type Item = PointKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PointKeyFrameCollection {
    type Item = PointKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: PointKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &PointKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: PointKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &PointKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PointKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: PointKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PointKeyFrameCollection> for super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PointKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame>> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame>> for &PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PointKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<PointKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: PointKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PointKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<PointKeyFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PointKeyFrameCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> for &PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PointKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PointKeyFrameCollection {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PointerDownThemeAnimation(::windows::core::IUnknown);
impl PointerDownThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerDownThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointerDownThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerDownThemeAnimationStatics<R, F: FnOnce(&IPointerDownThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerDownThemeAnimation, IPointerDownThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerDownThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerDownThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerDownThemeAnimation {}
impl ::core::fmt::Debug for PointerDownThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDownThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerDownThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointerDownThemeAnimation;{b58e714e-c49d-4788-a233-0ae85d99dd5a})");
}
unsafe impl ::windows::core::Interface for PointerDownThemeAnimation {
    type Vtable = IPointerDownThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb58e714e_c49d_4788_a233_0ae85d99dd5a);
}
impl ::windows::core::RuntimeName for PointerDownThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointerDownThemeAnimation";
}
impl ::core::convert::From<PointerDownThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PointerDownThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PointerDownThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for Timeline {
    fn from(value: PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for Timeline {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for super::super::DependencyObject {
    fn from(value: PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointerDownThemeAnimation {}
unsafe impl ::core::marker::Sync for PointerDownThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PointerUpThemeAnimation(::windows::core::IUnknown);
impl PointerUpThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerUpThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointerUpThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerUpThemeAnimationStatics<R, F: FnOnce(&IPointerUpThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerUpThemeAnimation, IPointerUpThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerUpThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerUpThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerUpThemeAnimation {}
impl ::core::fmt::Debug for PointerUpThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerUpThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerUpThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointerUpThemeAnimation;{e9e9d07d-6340-4828-ad12-690694b9910b})");
}
unsafe impl ::windows::core::Interface for PointerUpThemeAnimation {
    type Vtable = IPointerUpThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9e9d07d_6340_4828_ad12_690694b9910b);
}
impl ::windows::core::RuntimeName for PointerUpThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointerUpThemeAnimation";
}
impl ::core::convert::From<PointerUpThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PointerUpThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PointerUpThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for Timeline {
    fn from(value: PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for Timeline {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for super::super::DependencyObject {
    fn from(value: PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointerUpThemeAnimation {}
unsafe impl ::core::marker::Sync for PointerUpThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PopInThemeAnimation(::windows::core::IUnknown);
impl PopInThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopInThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPopInThemeAnimationStatics<R, F: FnOnce(&IPopInThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopInThemeAnimation, IPopInThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PopInThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopInThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopInThemeAnimation {}
impl ::core::fmt::Debug for PopInThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopInThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PopInThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopInThemeAnimation;{196938c1-1c07-4c28-8847-f9f055b32855})");
}
unsafe impl ::windows::core::Interface for PopInThemeAnimation {
    type Vtable = IPopInThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196938c1_1c07_4c28_8847_f9f055b32855);
}
impl ::windows::core::RuntimeName for PopInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopInThemeAnimation";
}
impl ::core::convert::From<PopInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PopInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PopInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopInThemeAnimation> for Timeline {
    fn from(value: PopInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for Timeline {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PopInThemeAnimation> for super::super::DependencyObject {
    fn from(value: PopInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PopInThemeAnimation {}
unsafe impl ::core::marker::Sync for PopInThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PopOutThemeAnimation(::windows::core::IUnknown);
impl PopOutThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopOutThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPopOutThemeAnimationStatics<R, F: FnOnce(&IPopOutThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopOutThemeAnimation, IPopOutThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PopOutThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopOutThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopOutThemeAnimation {}
impl ::core::fmt::Debug for PopOutThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopOutThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PopOutThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopOutThemeAnimation;{4786ab49-0e48-4e81-a2e5-cc5aa19e48d3})");
}
unsafe impl ::windows::core::Interface for PopOutThemeAnimation {
    type Vtable = IPopOutThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4786ab49_0e48_4e81_a2e5_cc5aa19e48d3);
}
impl ::windows::core::RuntimeName for PopOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopOutThemeAnimation";
}
impl ::core::convert::From<PopOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PopOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PopOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for Timeline {
    fn from(value: PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for Timeline {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PopOutThemeAnimation {}
unsafe impl ::core::marker::Sync for PopOutThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PopupThemeTransition(::windows::core::IUnknown);
impl PopupThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopupThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPopupThemeTransitionStatics<R, F: FnOnce(&IPopupThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopupThemeTransition, IPopupThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PopupThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopupThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopupThemeTransition {}
impl ::core::fmt::Debug for PopupThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopupThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PopupThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopupThemeTransition;{47843552-4283-545e-c791-268dca22ce4b})");
}
unsafe impl ::windows::core::Interface for PopupThemeTransition {
    type Vtable = IPopupThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47843552_4283_545e_c791_268dca22ce4b);
}
impl ::windows::core::RuntimeName for PopupThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopupThemeTransition";
}
impl ::core::convert::From<PopupThemeTransition> for ::windows::core::IUnknown {
    fn from(value: PopupThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &PopupThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopupThemeTransition> for ::windows::core::IInspectable {
    fn from(value: PopupThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &PopupThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopupThemeTransition> for Transition {
    fn from(value: PopupThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopupThemeTransition> for Transition {
    fn from(value: &PopupThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<PopupThemeTransition> for super::super::DependencyObject {
    fn from(value: PopupThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopupThemeTransition> for super::super::DependencyObject {
    fn from(value: &PopupThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PopupThemeTransition {}
unsafe impl ::core::marker::Sync for PopupThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct PowerEase(::windows::core::IUnknown);
impl PowerEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Power(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetPower(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn PowerProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPowerEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPowerEaseStatics<R, F: FnOnce(&IPowerEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerEase, IPowerEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PowerEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PowerEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PowerEase {}
impl ::core::fmt::Debug for PowerEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PowerEase;{69c80579-eedf-405b-8680-d9606880c937})");
}
unsafe impl ::windows::core::Interface for PowerEase {
    type Vtable = IPowerEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c80579_eedf_405b_8680_d9606880c937);
}
impl ::windows::core::RuntimeName for PowerEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PowerEase";
}
impl ::core::convert::From<PowerEase> for ::windows::core::IUnknown {
    fn from(value: PowerEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PowerEase> for ::windows::core::IUnknown {
    fn from(value: &PowerEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PowerEase> for ::windows::core::IInspectable {
    fn from(value: PowerEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PowerEase> for ::windows::core::IInspectable {
    fn from(value: &PowerEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PowerEase> for EasingFunctionBase {
    fn from(value: PowerEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PowerEase> for EasingFunctionBase {
    fn from(value: &PowerEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<PowerEase> for super::super::DependencyObject {
    fn from(value: PowerEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PowerEase> for super::super::DependencyObject {
    fn from(value: &PowerEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PowerEase {}
unsafe impl ::core::marker::Sync for PowerEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct QuadraticEase(::windows::core::IUnknown);
impl QuadraticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuadraticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QuadraticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QuadraticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuadraticEase {}
impl ::core::fmt::Debug for QuadraticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuadraticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for QuadraticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuadraticEase;{e1510e91-ef6d-44f0-803d-68d16de0ddfc})");
}
unsafe impl ::windows::core::Interface for QuadraticEase {
    type Vtable = IQuadraticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1510e91_ef6d_44f0_803d_68d16de0ddfc);
}
impl ::windows::core::RuntimeName for QuadraticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuadraticEase";
}
impl ::core::convert::From<QuadraticEase> for ::windows::core::IUnknown {
    fn from(value: QuadraticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuadraticEase> for ::windows::core::IUnknown {
    fn from(value: &QuadraticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuadraticEase> for ::windows::core::IInspectable {
    fn from(value: QuadraticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuadraticEase> for ::windows::core::IInspectable {
    fn from(value: &QuadraticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuadraticEase> for EasingFunctionBase {
    fn from(value: QuadraticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuadraticEase> for EasingFunctionBase {
    fn from(value: &QuadraticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<QuadraticEase> for super::super::DependencyObject {
    fn from(value: QuadraticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuadraticEase> for super::super::DependencyObject {
    fn from(value: &QuadraticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for QuadraticEase {}
unsafe impl ::core::marker::Sync for QuadraticEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct QuarticEase(::windows::core::IUnknown);
impl QuarticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuarticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QuarticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QuarticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuarticEase {}
impl ::core::fmt::Debug for QuarticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuarticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for QuarticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuarticEase;{e8698814-fe42-4a05-b5b8-081f41157815})");
}
unsafe impl ::windows::core::Interface for QuarticEase {
    type Vtable = IQuarticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8698814_fe42_4a05_b5b8_081f41157815);
}
impl ::windows::core::RuntimeName for QuarticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuarticEase";
}
impl ::core::convert::From<QuarticEase> for ::windows::core::IUnknown {
    fn from(value: QuarticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuarticEase> for ::windows::core::IUnknown {
    fn from(value: &QuarticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuarticEase> for ::windows::core::IInspectable {
    fn from(value: QuarticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuarticEase> for ::windows::core::IInspectable {
    fn from(value: &QuarticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuarticEase> for EasingFunctionBase {
    fn from(value: QuarticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuarticEase> for EasingFunctionBase {
    fn from(value: &QuarticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<QuarticEase> for super::super::DependencyObject {
    fn from(value: QuarticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuarticEase> for super::super::DependencyObject {
    fn from(value: &QuarticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for QuarticEase {}
unsafe impl ::core::marker::Sync for QuarticEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct QuinticEase(::windows::core::IUnknown);
impl QuinticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuinticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QuinticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QuinticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuinticEase {}
impl ::core::fmt::Debug for QuinticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuinticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for QuinticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuinticEase;{92ee793b-3c49-4108-aa11-ab786603da21})");
}
unsafe impl ::windows::core::Interface for QuinticEase {
    type Vtable = IQuinticEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ee793b_3c49_4108_aa11_ab786603da21);
}
impl ::windows::core::RuntimeName for QuinticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuinticEase";
}
impl ::core::convert::From<QuinticEase> for ::windows::core::IUnknown {
    fn from(value: QuinticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuinticEase> for ::windows::core::IUnknown {
    fn from(value: &QuinticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuinticEase> for ::windows::core::IInspectable {
    fn from(value: QuinticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuinticEase> for ::windows::core::IInspectable {
    fn from(value: &QuinticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuinticEase> for EasingFunctionBase {
    fn from(value: QuinticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuinticEase> for EasingFunctionBase {
    fn from(value: &QuinticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<QuinticEase> for super::super::DependencyObject {
    fn from(value: QuinticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuinticEase> for super::super::DependencyObject {
    fn from(value: &QuinticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for QuinticEase {}
unsafe impl ::core::marker::Sync for QuinticEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct ReorderThemeTransition(::windows::core::IUnknown);
impl ReorderThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ReorderThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ReorderThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReorderThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReorderThemeTransition {}
impl ::core::fmt::Debug for ReorderThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReorderThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReorderThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ReorderThemeTransition;{f2065c6c-d052-4ad1-8362-b71b36df7497})");
}
unsafe impl ::windows::core::Interface for ReorderThemeTransition {
    type Vtable = IReorderThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2065c6c_d052_4ad1_8362_b71b36df7497);
}
impl ::windows::core::RuntimeName for ReorderThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ReorderThemeTransition";
}
impl ::core::convert::From<ReorderThemeTransition> for ::windows::core::IUnknown {
    fn from(value: ReorderThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReorderThemeTransition> for ::windows::core::IInspectable {
    fn from(value: ReorderThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReorderThemeTransition> for Transition {
    fn from(value: ReorderThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for Transition {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<ReorderThemeTransition> for super::super::DependencyObject {
    fn from(value: ReorderThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for super::super::DependencyObject {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ReorderThemeTransition {}
unsafe impl ::core::marker::Sync for ReorderThemeTransition {}
#[repr(C)]
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
#[cfg(feature = "Foundation")]
pub struct RepeatBehavior {
    pub Count: f64,
    pub Duration: super::super::super::super::Foundation::TimeSpan,
    pub Type: RepeatBehaviorType,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for RepeatBehavior {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for RepeatBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for RepeatBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RepeatBehavior").field("Count", &self.Count).field("Duration", &self.Duration).field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for RepeatBehavior {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for RepeatBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Animation.RepeatBehavior;f8;struct(Windows.Foundation.TimeSpan;i8);enum(Windows.UI.Xaml.Media.Animation.RepeatBehaviorType;i4))");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for RepeatBehavior {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for RepeatBehavior {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RepeatBehavior>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for RepeatBehavior {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for RepeatBehavior {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct RepeatBehaviorHelper(::windows::core::IUnknown);
impl RepeatBehaviorHelper {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Forever() -> ::windows::core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RepeatBehavior>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromCount(count: f64) -> ::windows::core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<RepeatBehavior>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(duration: Param0) -> ::windows::core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), duration.into_param().abi(), &mut result__).from_abi::<RepeatBehavior>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetHasCount<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>>(target: Param0) -> ::windows::core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetHasDuration<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>>(target: Param0) -> ::windows::core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Equals<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>, Param1: ::windows::core::IntoParam<'a, RepeatBehavior>>(target: Param0, value: Param1) -> ::windows::core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), target.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRepeatBehaviorHelperStatics<R, F: FnOnce(&IRepeatBehaviorHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepeatBehaviorHelper, IRepeatBehaviorHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RepeatBehaviorHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RepeatBehaviorHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepeatBehaviorHelper {}
impl ::core::fmt::Debug for RepeatBehaviorHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepeatBehaviorHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RepeatBehaviorHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepeatBehaviorHelper;{6863ab72-4997-47f9-87ad-37efb75993ea})");
}
unsafe impl ::windows::core::Interface for RepeatBehaviorHelper {
    type Vtable = IRepeatBehaviorHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6863ab72_4997_47f9_87ad_37efb75993ea);
}
impl ::windows::core::RuntimeName for RepeatBehaviorHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepeatBehaviorHelper";
}
impl ::core::convert::From<RepeatBehaviorHelper> for ::windows::core::IUnknown {
    fn from(value: RepeatBehaviorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepeatBehaviorHelper> for ::windows::core::IUnknown {
    fn from(value: &RepeatBehaviorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepeatBehaviorHelper> for ::windows::core::IInspectable {
    fn from(value: RepeatBehaviorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepeatBehaviorHelper> for ::windows::core::IInspectable {
    fn from(value: &RepeatBehaviorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RepeatBehaviorHelper {}
unsafe impl ::core::marker::Sync for RepeatBehaviorHelper {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct RepeatBehaviorType(pub i32);
impl RepeatBehaviorType {
    pub const Count: Self = Self(0i32);
    pub const Duration: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
impl ::core::marker::Copy for RepeatBehaviorType {}
impl ::core::clone::Clone for RepeatBehaviorType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RepeatBehaviorType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RepeatBehaviorType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepeatBehaviorType {}
impl ::core::fmt::Debug for RepeatBehaviorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepeatBehaviorType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RepeatBehaviorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.RepeatBehaviorType;i4)");
}
impl ::windows::core::DefaultType for RepeatBehaviorType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct RepositionThemeAnimation(::windows::core::IUnknown);
impl RepositionThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRepositionThemeAnimationStatics<R, F: FnOnce(&IRepositionThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeAnimation, IRepositionThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RepositionThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RepositionThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepositionThemeAnimation {}
impl ::core::fmt::Debug for RepositionThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepositionThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RepositionThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepositionThemeAnimation;{ecda24e8-8945-4949-a1bf-62109965a7e9})");
}
unsafe impl ::windows::core::Interface for RepositionThemeAnimation {
    type Vtable = IRepositionThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecda24e8_8945_4949_a1bf_62109965a7e9);
}
impl ::windows::core::RuntimeName for RepositionThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepositionThemeAnimation";
}
impl ::core::convert::From<RepositionThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: RepositionThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: RepositionThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for Timeline {
    fn from(value: RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for Timeline {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for super::super::DependencyObject {
    fn from(value: RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for super::super::DependencyObject {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RepositionThemeAnimation {}
unsafe impl ::core::marker::Sync for RepositionThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct RepositionThemeTransition(::windows::core::IUnknown);
impl RepositionThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRepositionThemeTransition2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRepositionThemeTransition2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn IsStaggeringEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeTransitionStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRepositionThemeTransitionStatics2<R, F: FnOnce(&IRepositionThemeTransitionStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeTransition, IRepositionThemeTransitionStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RepositionThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RepositionThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepositionThemeTransition {}
impl ::core::fmt::Debug for RepositionThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepositionThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RepositionThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepositionThemeTransition;{88329b82-98f3-455a-ac53-2e7083b6e22c})");
}
unsafe impl ::windows::core::Interface for RepositionThemeTransition {
    type Vtable = IRepositionThemeTransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88329b82_98f3_455a_ac53_2e7083b6e22c);
}
impl ::windows::core::RuntimeName for RepositionThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepositionThemeTransition";
}
impl ::core::convert::From<RepositionThemeTransition> for ::windows::core::IUnknown {
    fn from(value: RepositionThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeTransition> for ::windows::core::IInspectable {
    fn from(value: RepositionThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeTransition> for Transition {
    fn from(value: RepositionThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for Transition {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<RepositionThemeTransition> for super::super::DependencyObject {
    fn from(value: RepositionThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for super::super::DependencyObject {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RepositionThemeTransition {}
unsafe impl ::core::marker::Sync for RepositionThemeTransition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SineEase(::windows::core::IUnknown);
impl SineEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SineEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SineEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SineEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SineEase {}
impl ::core::fmt::Debug for SineEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SineEase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SineEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SineEase;{a9382962-230b-49da-9e0d-664987892343})");
}
unsafe impl ::windows::core::Interface for SineEase {
    type Vtable = ISineEaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9382962_230b_49da_9e0d_664987892343);
}
impl ::windows::core::RuntimeName for SineEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SineEase";
}
impl ::core::convert::From<SineEase> for ::windows::core::IUnknown {
    fn from(value: SineEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SineEase> for ::windows::core::IUnknown {
    fn from(value: &SineEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SineEase> for ::windows::core::IInspectable {
    fn from(value: SineEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SineEase> for ::windows::core::IInspectable {
    fn from(value: &SineEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SineEase> for EasingFunctionBase {
    fn from(value: SineEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SineEase> for EasingFunctionBase {
    fn from(value: &SineEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<SineEase> for super::super::DependencyObject {
    fn from(value: SineEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SineEase> for super::super::DependencyObject {
    fn from(value: &SineEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SineEase {}
unsafe impl ::core::marker::Sync for SineEase {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SlideNavigationTransitionEffect(pub i32);
impl SlideNavigationTransitionEffect {
    pub const FromBottom: Self = Self(0i32);
    pub const FromLeft: Self = Self(1i32);
    pub const FromRight: Self = Self(2i32);
}
impl ::core::marker::Copy for SlideNavigationTransitionEffect {}
impl ::core::clone::Clone for SlideNavigationTransitionEffect {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SlideNavigationTransitionEffect {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SlideNavigationTransitionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlideNavigationTransitionEffect {}
impl ::core::fmt::Debug for SlideNavigationTransitionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlideNavigationTransitionEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SlideNavigationTransitionEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionEffect;i4)");
}
impl ::windows::core::DefaultType for SlideNavigationTransitionEffect {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SlideNavigationTransitionInfo(::windows::core::IUnknown);
impl SlideNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SlideNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Effect(&self) -> ::windows::core::Result<SlideNavigationTransitionEffect> {
        let this = &::windows::core::Interface::cast::<ISlideNavigationTransitionInfo2>(self)?;
        unsafe {
            let mut result__: SlideNavigationTransitionEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SlideNavigationTransitionEffect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetEffect(&self, value: SlideNavigationTransitionEffect) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlideNavigationTransitionInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn EffectProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISlideNavigationTransitionInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISlideNavigationTransitionInfoStatics2<R, F: FnOnce(&ISlideNavigationTransitionInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SlideNavigationTransitionInfo, ISlideNavigationTransitionInfoStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SlideNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlideNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlideNavigationTransitionInfo {}
impl ::core::fmt::Debug for SlideNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlideNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SlideNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionInfo;{d6ac9d77-2e03-405f-80ed-e62beef3668f})");
}
unsafe impl ::windows::core::Interface for SlideNavigationTransitionInfo {
    type Vtable = ISlideNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6ac9d77_2e03_405f_80ed_e62beef3668f);
}
impl ::windows::core::RuntimeName for SlideNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionInfo";
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SlideNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for SlideNavigationTransitionInfo {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SplineColorKeyFrame(::windows::core::IUnknown);
impl SplineColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeySpline(&self) -> ::windows::core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeySpline>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetKeySpline<'a, Param0: ::windows::core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeySplineProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplineColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISplineColorKeyFrameStatics<R, F: FnOnce(&ISplineColorKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineColorKeyFrame, ISplineColorKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplineColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplineColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplineColorKeyFrame {}
impl ::core::fmt::Debug for SplineColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplineColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SplineColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplineColorKeyFrame;{1a4a5941-1fe0-473a-8efe-4316d8c86229})");
}
unsafe impl ::windows::core::Interface for SplineColorKeyFrame {
    type Vtable = ISplineColorKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4a5941_1fe0_473a_8efe_4316d8c86229);
}
impl ::windows::core::RuntimeName for SplineColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplineColorKeyFrame";
}
impl ::core::convert::From<SplineColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: SplineColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: SplineColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for ColorKeyFrame {
    fn from(value: SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ColorKeyFrame {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for super::super::DependencyObject {
    fn from(value: SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplineColorKeyFrame {}
unsafe impl ::core::marker::Sync for SplineColorKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SplineDoubleKeyFrame(::windows::core::IUnknown);
impl SplineDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeySpline(&self) -> ::windows::core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeySpline>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetKeySpline<'a, Param0: ::windows::core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeySplineProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplineDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISplineDoubleKeyFrameStatics<R, F: FnOnce(&ISplineDoubleKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineDoubleKeyFrame, ISplineDoubleKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplineDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplineDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplineDoubleKeyFrame {}
impl ::core::fmt::Debug for SplineDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplineDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SplineDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplineDoubleKeyFrame;{00d72d38-6b2b-4843-838e-c8b115eec801})");
}
unsafe impl ::windows::core::Interface for SplineDoubleKeyFrame {
    type Vtable = ISplineDoubleKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00d72d38_6b2b_4843_838e_c8b115eec801);
}
impl ::windows::core::RuntimeName for SplineDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplineDoubleKeyFrame";
}
impl ::core::convert::From<SplineDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplineDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for SplineDoubleKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SplinePointKeyFrame(::windows::core::IUnknown);
impl SplinePointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplinePointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeySpline(&self) -> ::windows::core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeySpline>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetKeySpline<'a, Param0: ::windows::core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn KeySplineProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplinePointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISplinePointKeyFrameStatics<R, F: FnOnce(&ISplinePointKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplinePointKeyFrame, ISplinePointKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplinePointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplinePointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplinePointKeyFrame {}
impl ::core::fmt::Debug for SplinePointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplinePointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SplinePointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplinePointKeyFrame;{0f19f306-7036-494f-bc3c-780df0cc524a})");
}
unsafe impl ::windows::core::Interface for SplinePointKeyFrame {
    type Vtable = ISplinePointKeyFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f19f306_7036_494f_bc3c_780df0cc524a);
}
impl ::windows::core::RuntimeName for SplinePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplinePointKeyFrame";
}
impl ::core::convert::From<SplinePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: SplinePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: SplinePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for PointKeyFrame {
    fn from(value: SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for PointKeyFrame {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for super::super::DependencyObject {
    fn from(value: SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplinePointKeyFrame {}
unsafe impl ::core::marker::Sync for SplinePointKeyFrame {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SplitCloseThemeAnimation(::windows::core::IUnknown);
impl SplitCloseThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitCloseThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOpenedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOpenedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetClosedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetClosedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetContentTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetContentTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OffsetFromCenter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::AnimationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OffsetFromCenterProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTranslationDirectionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTranslationOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISplitCloseThemeAnimationStatics<R, F: FnOnce(&ISplitCloseThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitCloseThemeAnimation, ISplitCloseThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplitCloseThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplitCloseThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplitCloseThemeAnimation {}
impl ::core::fmt::Debug for SplitCloseThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplitCloseThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SplitCloseThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplitCloseThemeAnimation;{4f799518-ff39-4e90-bb74-2abd56027402})");
}
unsafe impl ::windows::core::Interface for SplitCloseThemeAnimation {
    type Vtable = ISplitCloseThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f799518_ff39_4e90_bb74_2abd56027402);
}
impl ::windows::core::RuntimeName for SplitCloseThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplitCloseThemeAnimation";
}
impl ::core::convert::From<SplitCloseThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for Timeline {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for Timeline {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for super::super::DependencyObject {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplitCloseThemeAnimation {}
unsafe impl ::core::marker::Sync for SplitCloseThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SplitOpenThemeAnimation(::windows::core::IUnknown);
impl SplitOpenThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitOpenThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOpenedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOpenedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetClosedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetClosedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetContentTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetContentTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OffsetFromCenter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::AnimationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'UI_Xaml_Controls_Primitives'*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OpenedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ClosedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn OffsetFromCenterProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTranslationDirectionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ContentTranslationOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISplitOpenThemeAnimationStatics<R, F: FnOnce(&ISplitOpenThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitOpenThemeAnimation, ISplitOpenThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplitOpenThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplitOpenThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplitOpenThemeAnimation {}
impl ::core::fmt::Debug for SplitOpenThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplitOpenThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SplitOpenThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplitOpenThemeAnimation;{785fd7aa-5456-4639-8fd2-26bae6a5ffe4})");
}
unsafe impl ::windows::core::Interface for SplitOpenThemeAnimation {
    type Vtable = ISplitOpenThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x785fd7aa_5456_4639_8fd2_26bae6a5ffe4);
}
impl ::windows::core::RuntimeName for SplitOpenThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplitOpenThemeAnimation";
}
impl ::core::convert::From<SplitOpenThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for Timeline {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for Timeline {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for super::super::DependencyObject {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplitOpenThemeAnimation {}
unsafe impl ::core::marker::Sync for SplitOpenThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct Storyboard(::windows::core::IUnknown);
impl Storyboard {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Storyboard, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<TimelineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimelineCollection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Seek<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, offset: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), offset.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Begin(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetCurrentState(&self) -> ::windows::core::Result<ClockState> {
        let this = self;
        unsafe {
            let mut result__: ClockState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClockState>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SeekAlignedToLastTick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, offset: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), offset.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SkipToFill(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetPropertyProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetTargetProperty<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetProperty<'a, Param0: ::windows::core::IntoParam<'a, Timeline>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, path: Param1) -> ::windows::core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), path.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn GetTargetName<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, Timeline>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, name: Param1) -> ::windows::core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), name.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTarget<'a, Param0: ::windows::core::IntoParam<'a, Timeline>, Param1: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(timeline: Param0, target: Param1) -> ::windows::core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), timeline.into_param().abi(), target.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IStoryboardStatics<R, F: FnOnce(&IStoryboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Storyboard, IStoryboardStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Storyboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Storyboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Storyboard {}
impl ::core::fmt::Debug for Storyboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Storyboard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Storyboard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Storyboard;{d45c1e6e-3594-460e-981a-32271bd3aa06})");
}
unsafe impl ::windows::core::Interface for Storyboard {
    type Vtable = IStoryboardVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45c1e6e_3594_460e_981a_32271bd3aa06);
}
impl ::windows::core::RuntimeName for Storyboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Storyboard";
}
impl ::core::convert::From<Storyboard> for ::windows::core::IUnknown {
    fn from(value: Storyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Storyboard> for ::windows::core::IUnknown {
    fn from(value: &Storyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Storyboard> for ::windows::core::IInspectable {
    fn from(value: Storyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Storyboard> for ::windows::core::IInspectable {
    fn from(value: &Storyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Storyboard> for Timeline {
    fn from(value: Storyboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Storyboard> for Timeline {
    fn from(value: &Storyboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<Storyboard> for super::super::DependencyObject {
    fn from(value: Storyboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Storyboard> for super::super::DependencyObject {
    fn from(value: &Storyboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Storyboard {}
unsafe impl ::core::marker::Sync for Storyboard {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SuppressNavigationTransitionInfo(::windows::core::IUnknown);
impl SuppressNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SuppressNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SuppressNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuppressNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuppressNavigationTransitionInfo {}
impl ::core::fmt::Debug for SuppressNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuppressNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SuppressNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SuppressNavigationTransitionInfo;{244d7b0c-b1b7-4871-9d3e-d56203a3a5b4})");
}
unsafe impl ::windows::core::Interface for SuppressNavigationTransitionInfo {
    type Vtable = ISuppressNavigationTransitionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x244d7b0c_b1b7_4871_9d3e_d56203a3a5b4);
}
impl ::windows::core::RuntimeName for SuppressNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SuppressNavigationTransitionInfo";
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SuppressNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for SuppressNavigationTransitionInfo {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SwipeBackThemeAnimation(::windows::core::IUnknown);
impl SwipeBackThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeBackThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISwipeBackThemeAnimationStatics<R, F: FnOnce(&ISwipeBackThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeBackThemeAnimation, ISwipeBackThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SwipeBackThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SwipeBackThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SwipeBackThemeAnimation {}
impl ::core::fmt::Debug for SwipeBackThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SwipeBackThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SwipeBackThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SwipeBackThemeAnimation;{a38a4214-0bca-4d2d-95f7-ceba57fbaf60})");
}
unsafe impl ::windows::core::Interface for SwipeBackThemeAnimation {
    type Vtable = ISwipeBackThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa38a4214_0bca_4d2d_95f7_ceba57fbaf60);
}
impl ::windows::core::RuntimeName for SwipeBackThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SwipeBackThemeAnimation";
}
impl ::core::convert::From<SwipeBackThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for Timeline {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for Timeline {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for super::super::DependencyObject {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SwipeBackThemeAnimation {}
unsafe impl ::core::marker::Sync for SwipeBackThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct SwipeHintThemeAnimation(::windows::core::IUnknown);
impl SwipeHintThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeHintThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetToHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetToVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn ToVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISwipeHintThemeAnimationStatics<R, F: FnOnce(&ISwipeHintThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeHintThemeAnimation, ISwipeHintThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SwipeHintThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SwipeHintThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SwipeHintThemeAnimation {}
impl ::core::fmt::Debug for SwipeHintThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SwipeHintThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SwipeHintThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SwipeHintThemeAnimation;{cdd067c0-580e-4e40-be98-f202d3d84365})");
}
unsafe impl ::windows::core::Interface for SwipeHintThemeAnimation {
    type Vtable = ISwipeHintThemeAnimationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdd067c0_580e_4e40_be98_f202d3d84365);
}
impl ::windows::core::RuntimeName for SwipeHintThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SwipeHintThemeAnimation";
}
impl ::core::convert::From<SwipeHintThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for Timeline {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for Timeline {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for super::super::DependencyObject {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SwipeHintThemeAnimation {}
unsafe impl ::core::marker::Sync for SwipeHintThemeAnimation {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct Timeline(::windows::core::IUnknown);
impl Timeline {
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn AutoReverse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetAutoReverse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBeginTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Duration> {
        let this = self;
        unsafe {
            let mut result__: super::super::Duration = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Duration>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Duration>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SpeedRatio(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetSpeedRatio(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FillBehavior(&self) -> ::windows::core::Result<FillBehavior> {
        let this = self;
        unsafe {
            let mut result__: FillBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FillBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetFillBehavior(&self, value: FillBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RepeatBehavior(&self) -> ::windows::core::Result<RepeatBehavior> {
        let this = self;
        unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RepeatBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRepeatBehavior<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn AllowDependentAnimations() -> ::windows::core::Result<bool> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SetAllowDependentAnimations(value: bool) -> ::windows::core::Result<()> {
        Self::ITimelineStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn AutoReverseProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn BeginTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn DurationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn SpeedRatioProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn FillBehaviorProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
    pub fn RepeatBehaviorProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimelineStatics<R, F: FnOnce(&ITimelineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Timeline, ITimelineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Timeline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Timeline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Timeline {}
impl ::core::fmt::Debug for Timeline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Timeline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Timeline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Timeline;{0bc465dc-be4d-4d0d-9549-2208b715f40d})");
}
unsafe impl ::windows::core::Interface for Timeline {
    type Vtable = ITimelineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc465dc_be4d_4d0d_9549_2208b715f40d);
}
impl ::windows::core::RuntimeName for Timeline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Timeline";
}
impl ::core::convert::From<Timeline> for ::windows::core::IUnknown {
    fn from(value: Timeline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Timeline> for ::windows::core::IUnknown {
    fn from(value: &Timeline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Timeline> for ::windows::core::IInspectable {
    fn from(value: Timeline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Timeline> for ::windows::core::IInspectable {
    fn from(value: &Timeline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Timeline> for super::super::DependencyObject {
    fn from(value: Timeline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Timeline> for super::super::DependencyObject {
    fn from(value: &Timeline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Timeline {}
unsafe impl ::core::marker::Sync for Timeline {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct TimelineCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl TimelineCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimelineCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<Timeline>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<Timeline>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<Timeline>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Timeline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Timeline>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<Timeline>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<Timeline>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Timeline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Timeline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Timeline as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Timeline as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for TimelineCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for TimelineCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for TimelineCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for TimelineCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for TimelineCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.TimelineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.Timeline;{0bc465dc-be4d-4d0d-9549-2208b715f40d})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for TimelineCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVectorVtbl<Timeline>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for TimelineCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.TimelineCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for TimelineCollection {
    type Item = Timeline;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &TimelineCollection {
    type Item = Timeline;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineCollection> for ::windows::core::IUnknown {
    fn from(value: TimelineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineCollection> for ::windows::core::IUnknown {
    fn from(value: &TimelineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineCollection> for ::windows::core::IInspectable {
    fn from(value: TimelineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineCollection> for ::windows::core::IInspectable {
    fn from(value: &TimelineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<TimelineCollection> for super::super::super::super::Foundation::Collections::IIterable<Timeline> {
    type Error = ::windows::core::Error;
    fn try_from(value: TimelineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&TimelineCollection> for super::super::super::super::Foundation::Collections::IIterable<Timeline> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimelineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<Timeline>> for TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<Timeline>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<Timeline>> for &TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<Timeline>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IIterable<Timeline>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<TimelineCollection> for super::super::super::super::Foundation::Collections::IVector<Timeline> {
    type Error = ::windows::core::Error;
    fn try_from(value: TimelineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&TimelineCollection> for super::super::super::super::Foundation::Collections::IVector<Timeline> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimelineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> for TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> for &TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IVector<Timeline>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for TimelineCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for TimelineCollection {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation'*"]
#[repr(transparent)]
pub struct Transition(::windows::core::IUnknown);
impl Transition {}
impl ::core::clone::Clone for Transition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Transition {}
impl ::core::fmt::Debug for Transition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Transition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Transition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Transition;{3c677c7c-01d0-4dce-b333-976f93312b08})");
}
unsafe impl ::windows::core::Interface for Transition {
    type Vtable = ITransitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c677c7c_01d0_4dce_b333_976f93312b08);
}
impl ::windows::core::RuntimeName for Transition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Transition";
}
impl ::core::convert::From<Transition> for ::windows::core::IUnknown {
    fn from(value: Transition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transition> for ::windows::core::IUnknown {
    fn from(value: &Transition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transition> for ::windows::core::IInspectable {
    fn from(value: Transition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transition> for ::windows::core::IInspectable {
    fn from(value: &Transition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transition> for super::super::DependencyObject {
    fn from(value: Transition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Transition> for super::super::DependencyObject {
    fn from(value: &Transition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Transition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Transition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Transition {}
unsafe impl ::core::marker::Sync for Transition {}
#[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct TransitionCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl TransitionCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransitionCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<Transition>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<Transition>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<Transition>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Transition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Transition>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<Transition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<Transition>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Transition>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Transition>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Transition>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Transition>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Transition as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Media_Animation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Transition as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for TransitionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for TransitionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for TransitionCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for TransitionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransitionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for TransitionCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.TransitionCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.Transition;{3c677c7c-01d0-4dce-b333-976f93312b08})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for TransitionCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVectorVtbl<Transition>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for TransitionCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.TransitionCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for TransitionCollection {
    type Item = Transition;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &TransitionCollection {
    type Item = Transition;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransitionCollection> for ::windows::core::IUnknown {
    fn from(value: TransitionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransitionCollection> for ::windows::core::IUnknown {
    fn from(value: &TransitionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransitionCollection> for ::windows::core::IInspectable {
    fn from(value: TransitionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransitionCollection> for ::windows::core::IInspectable {
    fn from(value: &TransitionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<TransitionCollection> for super::super::super::super::Foundation::Collections::IIterable<Transition> {
    type Error = ::windows::core::Error;
    fn try_from(value: TransitionCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&TransitionCollection> for super::super::super::super::Foundation::Collections::IIterable<Transition> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TransitionCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<Transition>> for TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<Transition>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<Transition>> for &TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IIterable<Transition>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IIterable<Transition>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<TransitionCollection> for super::super::super::super::Foundation::Collections::IVector<Transition> {
    type Error = ::windows::core::Error;
    fn try_from(value: TransitionCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&TransitionCollection> for super::super::super::super::Foundation::Collections::IVector<Transition> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TransitionCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> for TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> for &TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::Collections::IVector<Transition>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for TransitionCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for TransitionCollection {}
