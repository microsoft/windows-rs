#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AddDeleteThemeTransition(pub ::windows::core::IInspectable);
impl AddDeleteThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AddDeleteThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AddDeleteThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.AddDeleteThemeTransition;{adec852e-4424-4dab-99c1-3a04e36a3c48})");
}
unsafe impl ::windows::core::Interface for AddDeleteThemeTransition {
    type Vtable = IAddDeleteThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadec852e_4424_4dab_99c1_3a04e36a3c48);
}
impl ::windows::core::RuntimeName for AddDeleteThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.AddDeleteThemeTransition";
}
impl ::core::convert::From<AddDeleteThemeTransition> for ::windows::core::IUnknown {
    fn from(value: AddDeleteThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for ::windows::core::IInspectable {
    fn from(value: AddDeleteThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for Transition {
    fn from(value: AddDeleteThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for Transition {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for super::super::DependencyObject {
    fn from(value: AddDeleteThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for super::super::DependencyObject {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for AddDeleteThemeTransition {}
unsafe impl ::core::marker::Sync for AddDeleteThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BackEase(pub ::windows::core::IInspectable);
impl BackEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Amplitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetAmplitude(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AmplitudeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBackEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBackEaseStatics<R, F: FnOnce(&IBackEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackEase, IBackEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BackEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BackEase;{e47796e7-f805-4a8f-81c9-38e6472caa94})");
}
unsafe impl ::windows::core::Interface for BackEase {
    type Vtable = IBackEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe47796e7_f805_4a8f_81c9_38e6472caa94);
}
impl ::windows::core::RuntimeName for BackEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BackEase";
}
impl ::core::convert::From<BackEase> for ::windows::core::IUnknown {
    fn from(value: BackEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BackEase> for ::windows::core::IUnknown {
    fn from(value: &BackEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BackEase> for ::windows::core::IInspectable {
    fn from(value: BackEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BackEase> for ::windows::core::IInspectable {
    fn from(value: &BackEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BackEase> for EasingFunctionBase {
    fn from(value: BackEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&BackEase> for EasingFunctionBase {
    fn from(value: &BackEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BackEase> for super::super::DependencyObject {
    fn from(value: BackEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BackEase> for super::super::DependencyObject {
    fn from(value: &BackEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BackEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BackEase {}
unsafe impl ::core::marker::Sync for BackEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BasicConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
impl BasicConnectedAnimationConfiguration {
    pub fn new() -> ::windows::core::Result<BasicConnectedAnimationConfiguration> {
        Self::IBasicConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<BasicConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn IBasicConnectedAnimationConfigurationFactory<R, F: FnOnce(&IBasicConnectedAnimationConfigurationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BasicConnectedAnimationConfiguration, IBasicConnectedAnimationConfigurationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BasicConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BasicConnectedAnimationConfiguration;{e675f9b5-a4d6-5353-83e6-c89e7cf8d456})");
}
unsafe impl ::windows::core::Interface for BasicConnectedAnimationConfiguration {
    type Vtable = IBasicConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe675f9b5_a4d6_5353_83e6_c89e7cf8d456);
}
impl ::windows::core::RuntimeName for BasicConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BasicConnectedAnimationConfiguration";
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        ::core::convert::Into::<ConnectedAnimationConfiguration>::into(&value)
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for &BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BasicConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for BasicConnectedAnimationConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BeginStoryboard(pub ::windows::core::IInspectable);
impl BeginStoryboard {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BeginStoryboard, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Storyboard(&self) -> ::windows::core::Result<Storyboard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Storyboard>(result__)
        }
    }
    pub fn SetStoryboard<'a, Param0: ::windows::core::IntoParam<'a, Storyboard>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StoryboardProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBeginStoryboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBeginStoryboardStatics<R, F: FnOnce(&IBeginStoryboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BeginStoryboard, IBeginStoryboardStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BeginStoryboard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BeginStoryboard;{64189fcd-49ec-4e52-a6f6-55324c921053})");
}
unsafe impl ::windows::core::Interface for BeginStoryboard {
    type Vtable = IBeginStoryboard_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64189fcd_49ec_4e52_a6f6_55324c921053);
}
impl ::windows::core::RuntimeName for BeginStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BeginStoryboard";
}
impl ::core::convert::From<BeginStoryboard> for ::windows::core::IUnknown {
    fn from(value: BeginStoryboard) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BeginStoryboard> for ::windows::core::IUnknown {
    fn from(value: &BeginStoryboard) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BeginStoryboard> for ::windows::core::IInspectable {
    fn from(value: BeginStoryboard) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BeginStoryboard> for ::windows::core::IInspectable {
    fn from(value: &BeginStoryboard) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BeginStoryboard> for super::super::TriggerAction {
    fn from(value: BeginStoryboard) -> Self {
        ::core::convert::Into::<super::super::TriggerAction>::into(&value)
    }
}
impl ::core::convert::From<&BeginStoryboard> for super::super::TriggerAction {
    fn from(value: &BeginStoryboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::TriggerAction> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::TriggerAction> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::TriggerAction>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::TriggerAction> for &BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::TriggerAction> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::TriggerAction>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BeginStoryboard> for super::super::DependencyObject {
    fn from(value: BeginStoryboard) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BeginStoryboard> for super::super::DependencyObject {
    fn from(value: &BeginStoryboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BeginStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BeginStoryboard {}
unsafe impl ::core::marker::Sync for BeginStoryboard {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BounceEase(pub ::windows::core::IInspectable);
impl BounceEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BounceEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Bounces(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetBounces(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bounciness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetBounciness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BouncesProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBounceEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn BouncinessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IBounceEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBounceEaseStatics<R, F: FnOnce(&IBounceEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BounceEase, IBounceEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BounceEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BounceEase;{2bf1464e-fc71-47ed-85a1-3ba9577718b4})");
}
unsafe impl ::windows::core::Interface for BounceEase {
    type Vtable = IBounceEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bf1464e_fc71_47ed_85a1_3ba9577718b4);
}
impl ::windows::core::RuntimeName for BounceEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BounceEase";
}
impl ::core::convert::From<BounceEase> for ::windows::core::IUnknown {
    fn from(value: BounceEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BounceEase> for ::windows::core::IUnknown {
    fn from(value: &BounceEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BounceEase> for ::windows::core::IInspectable {
    fn from(value: BounceEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BounceEase> for ::windows::core::IInspectable {
    fn from(value: &BounceEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BounceEase> for EasingFunctionBase {
    fn from(value: BounceEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&BounceEase> for EasingFunctionBase {
    fn from(value: &BounceEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<BounceEase> for super::super::DependencyObject {
    fn from(value: BounceEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&BounceEase> for super::super::DependencyObject {
    fn from(value: &BounceEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &BounceEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for BounceEase {}
unsafe impl ::core::marker::Sync for BounceEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CircleEase(pub ::windows::core::IInspectable);
impl CircleEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CircleEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CircleEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CircleEase;{53a3bdb2-9177-4e6e-a043-5082d889ab1f})");
}
unsafe impl ::windows::core::Interface for CircleEase {
    type Vtable = ICircleEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53a3bdb2_9177_4e6e_a043_5082d889ab1f);
}
impl ::windows::core::RuntimeName for CircleEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CircleEase";
}
impl ::core::convert::From<CircleEase> for ::windows::core::IUnknown {
    fn from(value: CircleEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CircleEase> for ::windows::core::IUnknown {
    fn from(value: &CircleEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CircleEase> for ::windows::core::IInspectable {
    fn from(value: CircleEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CircleEase> for ::windows::core::IInspectable {
    fn from(value: &CircleEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CircleEase> for EasingFunctionBase {
    fn from(value: CircleEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&CircleEase> for EasingFunctionBase {
    fn from(value: &CircleEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CircleEase> for super::super::DependencyObject {
    fn from(value: CircleEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CircleEase> for super::super::DependencyObject {
    fn from(value: &CircleEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CircleEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CircleEase {}
unsafe impl ::core::marker::Sync for CircleEase {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClockState(pub i32);
impl ClockState {
    pub const Active: ClockState = ClockState(0i32);
    pub const Filling: ClockState = ClockState(1i32);
    pub const Stopped: ClockState = ClockState(2i32);
}
impl ::core::convert::From<i32> for ClockState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ClockState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClockState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.ClockState;i4)");
}
impl ::windows::core::DefaultType for ClockState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorAnimation(pub ::windows::core::IInspectable);
impl ColorAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ByProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorAnimationStatics<R, F: FnOnce(&IColorAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimation, IColorAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ColorAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorAnimation;{b8ae8a15-0f63-4694-9467-bdafac1253ea})");
}
unsafe impl ::windows::core::Interface for ColorAnimation {
    type Vtable = IColorAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8ae8a15_0f63_4694_9467_bdafac1253ea);
}
impl ::windows::core::RuntimeName for ColorAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorAnimation";
}
impl ::core::convert::From<ColorAnimation> for ::windows::core::IUnknown {
    fn from(value: ColorAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ColorAnimation> for ::windows::core::IUnknown {
    fn from(value: &ColorAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ColorAnimation> for ::windows::core::IInspectable {
    fn from(value: ColorAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ColorAnimation> for ::windows::core::IInspectable {
    fn from(value: &ColorAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ColorAnimation> for Timeline {
    fn from(value: ColorAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&ColorAnimation> for Timeline {
    fn from(value: &ColorAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorAnimation> for super::super::DependencyObject {
    fn from(value: ColorAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ColorAnimation> for super::super::DependencyObject {
    fn from(value: &ColorAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ColorAnimation {}
unsafe impl ::core::marker::Sync for ColorAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
impl ColorAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<ColorKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ColorKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorAnimationUsingKeyFramesStatics<R, F: FnOnce(&IColorAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorAnimationUsingKeyFrames, IColorAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ColorAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorAnimationUsingKeyFrames;{f5c82640-13c3-42aa-9ae2-7e6b51c92f95})");
}
unsafe impl ::windows::core::Interface for ColorAnimationUsingKeyFrames {
    type Vtable = IColorAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c82640_13c3_42aa_9ae2_7e6b51c92f95);
}
impl ::windows::core::RuntimeName for ColorAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorAnimationUsingKeyFrames";
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for Timeline {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for Timeline {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ColorAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for ColorAnimationUsingKeyFrames {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorKeyFrame(pub ::windows::core::IInspectable);
impl ColorKeyFrame {
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorKeyFrameStatics<R, F: FnOnce(&IColorKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorKeyFrame, IColorKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrame;{b51d82d9-0910-4589-a284-b0c9205858e9})");
}
unsafe impl ::windows::core::Interface for ColorKeyFrame {
    type Vtable = IColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb51d82d9_0910_4589_a284_b0c9205858e9);
}
impl ::windows::core::RuntimeName for ColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorKeyFrame";
}
impl ::core::convert::From<ColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: ColorKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &ColorKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: ColorKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &ColorKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ColorKeyFrame> for super::super::DependencyObject {
    fn from(value: ColorKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &ColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ColorKeyFrame {}
unsafe impl ::core::marker::Sync for ColorKeyFrame {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorKeyFrameCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ColorKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ColorKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ColorKeyFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<ColorKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<ColorKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ColorKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ColorKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<ColorKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<ColorKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<ColorKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<ColorKeyFrame>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ColorKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrame;{b51d82d9-0910-4589-a284-b0c9205858e9})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ColorKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVector_abi<ColorKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ColorKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ColorKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: ColorKeyFrameCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ColorKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &ColorKeyFrameCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ColorKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: ColorKeyFrameCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ColorKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &ColorKeyFrameCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ColorKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame> {
    fn from(value: ColorKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ColorKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame> {
    fn from(value: &ColorKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> for &ColorKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ColorKeyFrame>> {
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
unsafe impl ::core::marker::Send for ColorKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ColorKeyFrameCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ColorKeyFrameCollection {
    type Item = ColorKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ColorKeyFrameCollection {
    type Item = ColorKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommonNavigationTransitionInfo(pub ::windows::core::IInspectable);
impl CommonNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommonNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsStaggeringEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsStaggerElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsStaggerElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsStaggerElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    pub fn ICommonNavigationTransitionInfoStatics<R, F: FnOnce(&ICommonNavigationTransitionInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommonNavigationTransitionInfo, ICommonNavigationTransitionInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CommonNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CommonNavigationTransitionInfo;{50345692-a555-4624-a361-0a91c1706473})");
}
unsafe impl ::windows::core::Interface for CommonNavigationTransitionInfo {
    type Vtable = ICommonNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50345692_a555_4624_a361_0a91c1706473);
}
impl ::windows::core::RuntimeName for CommonNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CommonNavigationTransitionInfo";
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<NavigationTransitionInfo>::into(&value)
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CommonNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for CommonNavigationTransitionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConnectedAnimation(pub ::windows::core::IInspectable);
impl ConnectedAnimation {
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn TryStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(&self, destination: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn IsScaleAnimationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsScaleAnimationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryStartWithCoordinatedElements<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement>>>(&self, destination: Param0, coordinatedelements: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), coordinatedelements.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetAnimationComponent<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Composition::ICompositionAnimationBase>>(&self, component: ConnectedAnimationComponent, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), component, animation.into_param().abi()).ok() }
    }
    pub fn Configuration(&self) -> ::windows::core::Result<ConnectedAnimationConfiguration> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConnectedAnimationConfiguration>(result__)
        }
    }
    pub fn SetConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConnectedAnimation3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimation;{3518628c-f387-4c25-ac98-44e86c3cadf0})");
}
unsafe impl ::windows::core::Interface for ConnectedAnimation {
    type Vtable = IConnectedAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3518628c_f387_4c25_ac98_44e86c3cadf0);
}
impl ::windows::core::RuntimeName for ConnectedAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimation";
}
impl ::core::convert::From<ConnectedAnimation> for ::windows::core::IUnknown {
    fn from(value: ConnectedAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConnectedAnimation> for ::windows::core::IUnknown {
    fn from(value: &ConnectedAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConnectedAnimation> for ::windows::core::IInspectable {
    fn from(value: ConnectedAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConnectedAnimation> for ::windows::core::IInspectable {
    fn from(value: &ConnectedAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConnectedAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimation {}
unsafe impl ::core::marker::Sync for ConnectedAnimation {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConnectedAnimationComponent(pub i32);
impl ConnectedAnimationComponent {
    pub const OffsetX: ConnectedAnimationComponent = ConnectedAnimationComponent(0i32);
    pub const OffsetY: ConnectedAnimationComponent = ConnectedAnimationComponent(1i32);
    pub const CrossFade: ConnectedAnimationComponent = ConnectedAnimationComponent(2i32);
    pub const Scale: ConnectedAnimationComponent = ConnectedAnimationComponent(3i32);
}
impl ::core::convert::From<i32> for ConnectedAnimationComponent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConnectedAnimationComponent {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimationComponent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.ConnectedAnimationComponent;i4)");
}
impl ::windows::core::DefaultType for ConnectedAnimationComponent {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
impl ConnectedAnimationConfiguration {}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimationConfiguration;{00218aae-cd8c-5651-92a0-c1db95c03998})");
}
unsafe impl ::windows::core::Interface for ConnectedAnimationConfiguration {
    type Vtable = IConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00218aae_cd8c_5651_92a0_c1db95c03998);
}
impl ::windows::core::RuntimeName for ConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimationConfiguration";
}
impl ::core::convert::From<ConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: ConnectedAnimationConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &ConnectedAnimationConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: ConnectedAnimationConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &ConnectedAnimationConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for ConnectedAnimationConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConnectedAnimationService(pub ::windows::core::IInspectable);
impl ConnectedAnimationService {
    #[cfg(feature = "Foundation")]
    pub fn DefaultDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn DefaultEasingFunction(&self) -> ::windows::core::Result<super::super::super::Composition::CompositionEasingFunction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Composition::CompositionEasingFunction>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetDefaultEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::CompositionEasingFunction>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PrepareToAnimate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::UIElement>>(&self, key: Param0, source: Param1) -> ::windows::core::Result<ConnectedAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), source.into_param().abi(), &mut result__).from_abi::<ConnectedAnimation>(result__)
        }
    }
    pub fn GetAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<ConnectedAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<ConnectedAnimation>(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<ConnectedAnimationService> {
        Self::IConnectedAnimationServiceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConnectedAnimationService>(result__)
        })
    }
    pub fn IConnectedAnimationServiceStatics<R, F: FnOnce(&IConnectedAnimationServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConnectedAnimationService, IConnectedAnimationServiceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectedAnimationService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimationService;{1c6875c9-19bb-4d47-b9aa-66c802dcb9ff})");
}
unsafe impl ::windows::core::Interface for ConnectedAnimationService {
    type Vtable = IConnectedAnimationService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c6875c9_19bb_4d47_b9aa_66c802dcb9ff);
}
impl ::windows::core::RuntimeName for ConnectedAnimationService {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimationService";
}
impl ::core::convert::From<ConnectedAnimationService> for ::windows::core::IUnknown {
    fn from(value: ConnectedAnimationService) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConnectedAnimationService> for ::windows::core::IUnknown {
    fn from(value: &ConnectedAnimationService) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConnectedAnimationService> for ::windows::core::IInspectable {
    fn from(value: ConnectedAnimationService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConnectedAnimationService> for ::windows::core::IInspectable {
    fn from(value: &ConnectedAnimationService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConnectedAnimationService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimationService {}
unsafe impl ::core::marker::Sync for ConnectedAnimationService {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContentThemeTransition(pub ::windows::core::IInspectable);
impl ContentThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn HorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContentThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn VerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContentThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IContentThemeTransitionStatics<R, F: FnOnce(&IContentThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentThemeTransition, IContentThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ContentThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ContentThemeTransition;{f66fc5c3-5915-437d-8e3b-adf8e7f0ab57})");
}
unsafe impl ::windows::core::Interface for ContentThemeTransition {
    type Vtable = IContentThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf66fc5c3_5915_437d_8e3b_adf8e7f0ab57);
}
impl ::windows::core::RuntimeName for ContentThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ContentThemeTransition";
}
impl ::core::convert::From<ContentThemeTransition> for ::windows::core::IUnknown {
    fn from(value: ContentThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContentThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &ContentThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContentThemeTransition> for ::windows::core::IInspectable {
    fn from(value: ContentThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContentThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &ContentThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContentThemeTransition> for Transition {
    fn from(value: ContentThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&ContentThemeTransition> for Transition {
    fn from(value: &ContentThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ContentThemeTransition> for super::super::DependencyObject {
    fn from(value: ContentThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ContentThemeTransition> for super::super::DependencyObject {
    fn from(value: &ContentThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ContentThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ContentThemeTransition {}
unsafe impl ::core::marker::Sync for ContentThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContinuumNavigationTransitionInfo(pub ::windows::core::IInspectable);
impl ContinuumNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContinuumNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ExitElement(&self) -> ::windows::core::Result<super::super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UIElement>(result__)
        }
    }
    pub fn SetExitElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ExitElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsEntranceElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsEntranceElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsEntranceElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    pub fn IsExitElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsExitElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsExitElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    pub fn ExitElementContainerProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn GetExitElementContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Controls::ListViewBase>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn SetExitElementContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Controls::ListViewBase>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    pub fn IContinuumNavigationTransitionInfoStatics<R, F: FnOnce(&IContinuumNavigationTransitionInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContinuumNavigationTransitionInfo, IContinuumNavigationTransitionInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ContinuumNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ContinuumNavigationTransitionInfo;{4be1dbad-8ba6-4004-8438-8a9017978543})");
}
unsafe impl ::windows::core::Interface for ContinuumNavigationTransitionInfo {
    type Vtable = IContinuumNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4be1dbad_8ba6_4004_8438_8a9017978543);
}
impl ::windows::core::RuntimeName for ContinuumNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ContinuumNavigationTransitionInfo";
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<NavigationTransitionInfo>::into(&value)
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ContinuumNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for ContinuumNavigationTransitionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CubicEase(pub ::windows::core::IInspectable);
impl CubicEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CubicEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CubicEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CubicEase;{1b94fc76-dad7-4354-b1a2-7969fbf6a70d})");
}
unsafe impl ::windows::core::Interface for CubicEase {
    type Vtable = ICubicEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b94fc76_dad7_4354_b1a2_7969fbf6a70d);
}
impl ::windows::core::RuntimeName for CubicEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CubicEase";
}
impl ::core::convert::From<CubicEase> for ::windows::core::IUnknown {
    fn from(value: CubicEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CubicEase> for ::windows::core::IUnknown {
    fn from(value: &CubicEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CubicEase> for ::windows::core::IInspectable {
    fn from(value: CubicEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CubicEase> for ::windows::core::IInspectable {
    fn from(value: &CubicEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CubicEase> for EasingFunctionBase {
    fn from(value: CubicEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&CubicEase> for EasingFunctionBase {
    fn from(value: &CubicEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<CubicEase> for super::super::DependencyObject {
    fn from(value: CubicEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CubicEase> for super::super::DependencyObject {
    fn from(value: &CubicEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CubicEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CubicEase {}
unsafe impl ::core::marker::Sync for CubicEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DirectConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
impl DirectConnectedAnimationConfiguration {
    pub fn new() -> ::windows::core::Result<DirectConnectedAnimationConfiguration> {
        Self::IDirectConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<DirectConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn IDirectConnectedAnimationConfigurationFactory<R, F: FnOnce(&IDirectConnectedAnimationConfigurationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DirectConnectedAnimationConfiguration, IDirectConnectedAnimationConfigurationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DirectConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DirectConnectedAnimationConfiguration;{ee5d736f-5738-5d86-b770-151948cf365e})");
}
unsafe impl ::windows::core::Interface for DirectConnectedAnimationConfiguration {
    type Vtable = IDirectConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5d736f_5738_5d86_b770_151948cf365e);
}
impl ::windows::core::RuntimeName for DirectConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DirectConnectedAnimationConfiguration";
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        ::core::convert::Into::<ConnectedAnimationConfiguration>::into(&value)
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for &DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DirectConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for DirectConnectedAnimationConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DiscreteColorKeyFrame(pub ::windows::core::IInspectable);
impl DiscreteColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscreteColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DiscreteColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteColorKeyFrame;{230c08f4-e062-4cb1-8e2a-14093d73ed8c})");
}
unsafe impl ::windows::core::Interface for DiscreteColorKeyFrame {
    type Vtable = IDiscreteColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230c08f4_e062_4cb1_8e2a_14093d73ed8c);
}
impl ::windows::core::RuntimeName for DiscreteColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteColorKeyFrame";
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ColorKeyFrame {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        ::core::convert::Into::<ColorKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ColorKeyFrame {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DiscreteColorKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteColorKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DiscreteDoubleKeyFrame(pub ::windows::core::IInspectable);
impl DiscreteDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscreteDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DiscreteDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteDoubleKeyFrame;{f5f51f3a-ad11-49ce-8e1c-08fdf1447446})");
}
unsafe impl ::windows::core::Interface for DiscreteDoubleKeyFrame {
    type Vtable = IDiscreteDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f51f3a_ad11_49ce_8e1c_08fdf1447446);
}
impl ::windows::core::RuntimeName for DiscreteDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteDoubleKeyFrame";
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::Into::<DoubleKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DiscreteDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteDoubleKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DiscreteObjectKeyFrame(pub ::windows::core::IInspectable);
impl DiscreteObjectKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscreteObjectKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DiscreteObjectKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteObjectKeyFrame;{c7dcde89-f12d-4a9c-8199-e7a9ece3a473})");
}
unsafe impl ::windows::core::Interface for DiscreteObjectKeyFrame {
    type Vtable = IDiscreteObjectKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7dcde89_f12d_4a9c_8199_e7a9ece3a473);
}
impl ::windows::core::RuntimeName for DiscreteObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteObjectKeyFrame";
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ObjectKeyFrame {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        ::core::convert::Into::<ObjectKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ObjectKeyFrame {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ObjectKeyFrame> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ObjectKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ObjectKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ObjectKeyFrame> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ObjectKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ObjectKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DiscreteObjectKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteObjectKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DiscretePointKeyFrame(pub ::windows::core::IInspectable);
impl DiscretePointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DiscretePointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DiscretePointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscretePointKeyFrame;{e0a9070d-4c42-4a90-983a-75f5a83a2fbe})");
}
unsafe impl ::windows::core::Interface for DiscretePointKeyFrame {
    type Vtable = IDiscretePointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0a9070d_4c42_4a90_983a_75f5a83a2fbe);
}
impl ::windows::core::RuntimeName for DiscretePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscretePointKeyFrame";
}
impl ::core::convert::From<DiscretePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DiscretePointKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DiscretePointKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for PointKeyFrame {
    fn from(value: DiscretePointKeyFrame) -> Self {
        ::core::convert::Into::<PointKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for PointKeyFrame {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscretePointKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DiscretePointKeyFrame {}
unsafe impl ::core::marker::Sync for DiscretePointKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleAnimation(pub ::windows::core::IInspectable);
impl DoubleAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ByProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDoubleAnimationStatics<R, F: FnOnce(&IDoubleAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimation, IDoubleAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleAnimation;{7e9f3d59-0f07-4bc9-977d-03763ff8154f})");
}
unsafe impl ::windows::core::Interface for DoubleAnimation {
    type Vtable = IDoubleAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e9f3d59_0f07_4bc9_977d_03763ff8154f);
}
impl ::windows::core::RuntimeName for DoubleAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleAnimation";
}
impl ::core::convert::From<DoubleAnimation> for ::windows::core::IUnknown {
    fn from(value: DoubleAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DoubleAnimation> for ::windows::core::IUnknown {
    fn from(value: &DoubleAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DoubleAnimation> for ::windows::core::IInspectable {
    fn from(value: DoubleAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DoubleAnimation> for ::windows::core::IInspectable {
    fn from(value: &DoubleAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DoubleAnimation> for Timeline {
    fn from(value: DoubleAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&DoubleAnimation> for Timeline {
    fn from(value: &DoubleAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DoubleAnimation> for super::super::DependencyObject {
    fn from(value: DoubleAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DoubleAnimation> for super::super::DependencyObject {
    fn from(value: &DoubleAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DoubleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DoubleAnimation {}
unsafe impl ::core::marker::Sync for DoubleAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
impl DoubleAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<DoubleKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DoubleKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDoubleAnimationUsingKeyFramesStatics<R, F: FnOnce(&IDoubleAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleAnimationUsingKeyFrames, IDoubleAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleAnimationUsingKeyFrames;{4fee628f-bfee-4f75-83c2-a93b39488473})");
}
unsafe impl ::windows::core::Interface for DoubleAnimationUsingKeyFrames {
    type Vtable = IDoubleAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fee628f_bfee_4f75_83c2_a93b39488473);
}
impl ::windows::core::RuntimeName for DoubleAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleAnimationUsingKeyFrames";
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for Timeline {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for Timeline {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DoubleAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for DoubleAnimationUsingKeyFrames {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleKeyFrame(pub ::windows::core::IInspectable);
impl DoubleKeyFrame {
    pub fn Value(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDoubleKeyFrameStatics<R, F: FnOnce(&IDoubleKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleKeyFrame, IDoubleKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrame;{674456fd-e81e-4f4e-b4ad-0acfed9ecd68})");
}
unsafe impl ::windows::core::Interface for DoubleKeyFrame {
    type Vtable = IDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674456fd_e81e_4f4e_b4ad_0acfed9ecd68);
}
impl ::windows::core::RuntimeName for DoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleKeyFrame";
}
impl ::core::convert::From<DoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: DoubleKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &DoubleKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: DoubleKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &DoubleKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: DoubleKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &DoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DoubleKeyFrame {}
unsafe impl ::core::marker::Sync for DoubleKeyFrame {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleKeyFrameCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl DoubleKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<DoubleKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<DoubleKeyFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<DoubleKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<DoubleKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, DoubleKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<DoubleKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<DoubleKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<DoubleKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<DoubleKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<DoubleKeyFrame>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for DoubleKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrame;{674456fd-e81e-4f4e-b4ad-0acfed9ecd68})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for DoubleKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVector_abi<DoubleKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for DoubleKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: DoubleKeyFrameCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &DoubleKeyFrameCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: DoubleKeyFrameCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &DoubleKeyFrameCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DoubleKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame> {
    fn from(value: DoubleKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DoubleKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame> {
    fn from(value: &DoubleKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> for &DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<DoubleKeyFrame>> {
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
unsafe impl ::core::marker::Send for DoubleKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DoubleKeyFrameCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for DoubleKeyFrameCollection {
    type Item = DoubleKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &DoubleKeyFrameCollection {
    type Item = DoubleKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragItemThemeAnimation(pub ::windows::core::IInspectable);
impl DragItemThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragItemThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragItemThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDragItemThemeAnimationStatics<R, F: FnOnce(&IDragItemThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragItemThemeAnimation, IDragItemThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DragItemThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DragItemThemeAnimation;{0c7d5db5-7ed6-4949-b4e6-a78c9f4f978d})");
}
unsafe impl ::windows::core::Interface for DragItemThemeAnimation {
    type Vtable = IDragItemThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c7d5db5_7ed6_4949_b4e6_a78c9f4f978d);
}
impl ::windows::core::RuntimeName for DragItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DragItemThemeAnimation";
}
impl ::core::convert::From<DragItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DragItemThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DragItemThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DragItemThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DragItemThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for Timeline {
    fn from(value: DragItemThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for Timeline {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: DragItemThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DragItemThemeAnimation {}
unsafe impl ::core::marker::Sync for DragItemThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragOverThemeAnimation(pub ::windows::core::IInspectable);
impl DragOverThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragOverThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ToOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetToOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn Direction(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::AnimationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DirectionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDragOverThemeAnimationStatics<R, F: FnOnce(&IDragOverThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragOverThemeAnimation, IDragOverThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DragOverThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DragOverThemeAnimation;{72f762f7-7e51-4a6b-b937-dc4b4c1c5458})");
}
unsafe impl ::windows::core::Interface for DragOverThemeAnimation {
    type Vtable = IDragOverThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72f762f7_7e51_4a6b_b937_dc4b4c1c5458);
}
impl ::windows::core::RuntimeName for DragOverThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DragOverThemeAnimation";
}
impl ::core::convert::From<DragOverThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DragOverThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DragOverThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DragOverThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DragOverThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for Timeline {
    fn from(value: DragOverThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for Timeline {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for super::super::DependencyObject {
    fn from(value: DragOverThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DragOverThemeAnimation {}
unsafe impl ::core::marker::Sync for DragOverThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DrillInNavigationTransitionInfo(pub ::windows::core::IInspectable);
impl DrillInNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillInNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DrillInNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillInNavigationTransitionInfo;{3b86201a-45d3-463b-939e-c8595f439bcc})");
}
unsafe impl ::windows::core::Interface for DrillInNavigationTransitionInfo {
    type Vtable = IDrillInNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b86201a_45d3_463b_939e_c8595f439bcc);
}
impl ::windows::core::RuntimeName for DrillInNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillInNavigationTransitionInfo";
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<NavigationTransitionInfo>::into(&value)
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DrillInNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for DrillInNavigationTransitionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DrillInThemeAnimation(pub ::windows::core::IInspectable);
impl DrillInThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillInThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetEntranceTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetEntranceTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetExitTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetExitTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EntranceTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDrillInThemeAnimationStatics<R, F: FnOnce(&IDrillInThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillInThemeAnimation, IDrillInThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DrillInThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillInThemeAnimation;{b090b824-f1d2-41b8-87ba-78034126594c})");
}
unsafe impl ::windows::core::Interface for DrillInThemeAnimation {
    type Vtable = IDrillInThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb090b824_f1d2_41b8_87ba_78034126594c);
}
impl ::windows::core::RuntimeName for DrillInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillInThemeAnimation";
}
impl ::core::convert::From<DrillInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DrillInThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DrillInThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DrillInThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DrillInThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for Timeline {
    fn from(value: DrillInThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for Timeline {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for super::super::DependencyObject {
    fn from(value: DrillInThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DrillInThemeAnimation {}
unsafe impl ::core::marker::Sync for DrillInThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DrillOutThemeAnimation(pub ::windows::core::IInspectable);
impl DrillOutThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillOutThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetEntranceTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetEntranceTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetExitTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetExitTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EntranceTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDrillOutThemeAnimationStatics<R, F: FnOnce(&IDrillOutThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DrillOutThemeAnimation, IDrillOutThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DrillOutThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillOutThemeAnimation;{d890ccdf-06d3-4f7e-8e4a-4fb76e256139})");
}
unsafe impl ::windows::core::Interface for DrillOutThemeAnimation {
    type Vtable = IDrillOutThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd890ccdf_06d3_4f7e_8e4a_4fb76e256139);
}
impl ::windows::core::RuntimeName for DrillOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillOutThemeAnimation";
}
impl ::core::convert::From<DrillOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DrillOutThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DrillOutThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for Timeline {
    fn from(value: DrillOutThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for Timeline {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: DrillOutThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DrillOutThemeAnimation {}
unsafe impl ::core::marker::Sync for DrillOutThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DropTargetItemThemeAnimation(pub ::windows::core::IInspectable);
impl DropTargetItemThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DropTargetItemThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IDropTargetItemThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDropTargetItemThemeAnimationStatics<R, F: FnOnce(&IDropTargetItemThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DropTargetItemThemeAnimation, IDropTargetItemThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DropTargetItemThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DropTargetItemThemeAnimation;{1881c968-1824-462b-87e8-c357212b977b})");
}
unsafe impl ::windows::core::Interface for DropTargetItemThemeAnimation {
    type Vtable = IDropTargetItemThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1881c968_1824_462b_87e8_c357212b977b);
}
impl ::windows::core::RuntimeName for DropTargetItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DropTargetItemThemeAnimation";
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for Timeline {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for Timeline {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DropTargetItemThemeAnimation {}
unsafe impl ::core::marker::Sync for DropTargetItemThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EasingColorKeyFrame(pub ::windows::core::IInspectable);
impl EasingColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingColorKeyFrameStatics<R, F: FnOnce(&IEasingColorKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingColorKeyFrame, IEasingColorKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EasingColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingColorKeyFrame;{c733d630-f4b9-4934-9bdd-27ac5ed1cfd8})");
}
unsafe impl ::windows::core::Interface for EasingColorKeyFrame {
    type Vtable = IEasingColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc733d630_f4b9_4934_9bdd_27ac5ed1cfd8);
}
impl ::windows::core::RuntimeName for EasingColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingColorKeyFrame";
}
impl ::core::convert::From<EasingColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: EasingColorKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &EasingColorKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: EasingColorKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &EasingColorKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for ColorKeyFrame {
    fn from(value: EasingColorKeyFrame) -> Self {
        ::core::convert::Into::<ColorKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ColorKeyFrame {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingColorKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EasingColorKeyFrame {}
unsafe impl ::core::marker::Sync for EasingColorKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EasingDoubleKeyFrame(pub ::windows::core::IInspectable);
impl EasingDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingDoubleKeyFrameStatics<R, F: FnOnce(&IEasingDoubleKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingDoubleKeyFrame, IEasingDoubleKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EasingDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingDoubleKeyFrame;{965adb8d-9a54-4108-b4ff-b5a5212cb338})");
}
unsafe impl ::windows::core::Interface for EasingDoubleKeyFrame {
    type Vtable = IEasingDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965adb8d_9a54_4108_b4ff_b5a5212cb338);
}
impl ::windows::core::RuntimeName for EasingDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingDoubleKeyFrame";
}
impl ::core::convert::From<EasingDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        ::core::convert::Into::<DoubleKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EasingDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for EasingDoubleKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EasingFunctionBase(pub ::windows::core::IInspectable);
impl EasingFunctionBase {
    pub fn EasingMode(&self) -> ::windows::core::Result<EasingMode> {
        let this = self;
        unsafe {
            let mut result__: EasingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingMode>(result__)
        }
    }
    pub fn SetEasingMode(&self, value: EasingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Ease(&self, normalizedtime: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), normalizedtime, &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn EasingModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingFunctionBaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingFunctionBaseStatics<R, F: FnOnce(&IEasingFunctionBaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingFunctionBase, IEasingFunctionBaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EasingFunctionBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingFunctionBase;{c108383f-2c02-4151-8ecd-68ddaa3f0d9b})");
}
unsafe impl ::windows::core::Interface for EasingFunctionBase {
    type Vtable = IEasingFunctionBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc108383f_2c02_4151_8ecd_68ddaa3f0d9b);
}
impl ::windows::core::RuntimeName for EasingFunctionBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingFunctionBase";
}
impl ::core::convert::From<EasingFunctionBase> for ::windows::core::IUnknown {
    fn from(value: EasingFunctionBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EasingFunctionBase> for ::windows::core::IUnknown {
    fn from(value: &EasingFunctionBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EasingFunctionBase> for ::windows::core::IInspectable {
    fn from(value: EasingFunctionBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EasingFunctionBase> for ::windows::core::IInspectable {
    fn from(value: &EasingFunctionBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EasingFunctionBase> for super::super::DependencyObject {
    fn from(value: EasingFunctionBase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EasingFunctionBase> for super::super::DependencyObject {
    fn from(value: &EasingFunctionBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingFunctionBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EasingFunctionBase {}
unsafe impl ::core::marker::Sync for EasingFunctionBase {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasingMode(pub i32);
impl EasingMode {
    pub const EaseOut: EasingMode = EasingMode(0i32);
    pub const EaseIn: EasingMode = EasingMode(1i32);
    pub const EaseInOut: EasingMode = EasingMode(2i32);
}
impl ::core::convert::From<i32> for EasingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.EasingMode;i4)");
}
impl ::windows::core::DefaultType for EasingMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EasingPointKeyFrame(pub ::windows::core::IInspectable);
impl EasingPointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingPointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEasingPointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingPointKeyFrameStatics<R, F: FnOnce(&IEasingPointKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasingPointKeyFrame, IEasingPointKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EasingPointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingPointKeyFrame;{b3c91380-6868-4225-a70b-3981cc0b2947})");
}
unsafe impl ::windows::core::Interface for EasingPointKeyFrame {
    type Vtable = IEasingPointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c91380_6868_4225_a70b_3981cc0b2947);
}
impl ::windows::core::RuntimeName for EasingPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingPointKeyFrame";
}
impl ::core::convert::From<EasingPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: EasingPointKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &EasingPointKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: EasingPointKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &EasingPointKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for PointKeyFrame {
    fn from(value: EasingPointKeyFrame) -> Self {
        ::core::convert::Into::<PointKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for PointKeyFrame {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingPointKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EasingPointKeyFrame {}
unsafe impl ::core::marker::Sync for EasingPointKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EdgeUIThemeTransition(pub ::windows::core::IInspectable);
impl EdgeUIThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EdgeUIThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::EdgeTransitionLocation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::EdgeTransitionLocation>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EdgeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEdgeUIThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEdgeUIThemeTransitionStatics<R, F: FnOnce(&IEdgeUIThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EdgeUIThemeTransition, IEdgeUIThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EdgeUIThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EdgeUIThemeTransition;{5c86c19b-49d7-19ec-cf19-83a73c6de75e})");
}
unsafe impl ::windows::core::Interface for EdgeUIThemeTransition {
    type Vtable = IEdgeUIThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c86c19b_49d7_19ec_cf19_83a73c6de75e);
}
impl ::windows::core::RuntimeName for EdgeUIThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EdgeUIThemeTransition";
}
impl ::core::convert::From<EdgeUIThemeTransition> for ::windows::core::IUnknown {
    fn from(value: EdgeUIThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for ::windows::core::IInspectable {
    fn from(value: EdgeUIThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for Transition {
    fn from(value: EdgeUIThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for Transition {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for super::super::DependencyObject {
    fn from(value: EdgeUIThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for super::super::DependencyObject {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EdgeUIThemeTransition {}
unsafe impl ::core::marker::Sync for EdgeUIThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ElasticEase(pub ::windows::core::IInspectable);
impl ElasticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElasticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Oscillations(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetOscillations(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Springiness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpringiness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OscillationsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IElasticEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SpringinessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IElasticEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IElasticEaseStatics<R, F: FnOnce(&IElasticEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ElasticEase, IElasticEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ElasticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ElasticEase;{ef5ba58c-b0b6-4a6c-9ca8-fb4233f12459})");
}
unsafe impl ::windows::core::Interface for ElasticEase {
    type Vtable = IElasticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5ba58c_b0b6_4a6c_9ca8_fb4233f12459);
}
impl ::windows::core::RuntimeName for ElasticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ElasticEase";
}
impl ::core::convert::From<ElasticEase> for ::windows::core::IUnknown {
    fn from(value: ElasticEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ElasticEase> for ::windows::core::IUnknown {
    fn from(value: &ElasticEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ElasticEase> for ::windows::core::IInspectable {
    fn from(value: ElasticEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ElasticEase> for ::windows::core::IInspectable {
    fn from(value: &ElasticEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ElasticEase> for EasingFunctionBase {
    fn from(value: ElasticEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&ElasticEase> for EasingFunctionBase {
    fn from(value: &ElasticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ElasticEase> for super::super::DependencyObject {
    fn from(value: ElasticEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ElasticEase> for super::super::DependencyObject {
    fn from(value: &ElasticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ElasticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ElasticEase {}
unsafe impl ::core::marker::Sync for ElasticEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EntranceNavigationTransitionInfo(pub ::windows::core::IInspectable);
impl EntranceNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsTargetElementProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsTargetElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsTargetElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    pub fn IEntranceNavigationTransitionInfoStatics<R, F: FnOnce(&IEntranceNavigationTransitionInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceNavigationTransitionInfo, IEntranceNavigationTransitionInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EntranceNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EntranceNavigationTransitionInfo;{720a256b-1c8a-41ee-82ec-8a87c0cf47da})");
}
unsafe impl ::windows::core::Interface for EntranceNavigationTransitionInfo {
    type Vtable = IEntranceNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x720a256b_1c8a_41ee_82ec_8a87c0cf47da);
}
impl ::windows::core::RuntimeName for EntranceNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EntranceNavigationTransitionInfo";
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<NavigationTransitionInfo>::into(&value)
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EntranceNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for EntranceNavigationTransitionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EntranceThemeTransition(pub ::windows::core::IInspectable);
impl EntranceThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsStaggeringEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEntranceThemeTransitionStatics<R, F: FnOnce(&IEntranceThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EntranceThemeTransition, IEntranceThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EntranceThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EntranceThemeTransition;{07698c09-a8e3-419a-a01d-7410a0ae8ec8})");
}
unsafe impl ::windows::core::Interface for EntranceThemeTransition {
    type Vtable = IEntranceThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07698c09_a8e3_419a_a01d_7410a0ae8ec8);
}
impl ::windows::core::RuntimeName for EntranceThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EntranceThemeTransition";
}
impl ::core::convert::From<EntranceThemeTransition> for ::windows::core::IUnknown {
    fn from(value: EntranceThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &EntranceThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EntranceThemeTransition> for ::windows::core::IInspectable {
    fn from(value: EntranceThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &EntranceThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<EntranceThemeTransition> for Transition {
    fn from(value: EntranceThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for Transition {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<EntranceThemeTransition> for super::super::DependencyObject {
    fn from(value: EntranceThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for super::super::DependencyObject {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for EntranceThemeTransition {}
unsafe impl ::core::marker::Sync for EntranceThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExponentialEase(pub ::windows::core::IInspectable);
impl ExponentialEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExponentialEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Exponent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetExponent(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ExponentProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IExponentialEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IExponentialEaseStatics<R, F: FnOnce(&IExponentialEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExponentialEase, IExponentialEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ExponentialEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ExponentialEase;{7cb9e41d-f0bb-4bca-9da5-9ba3a11734c4})");
}
unsafe impl ::windows::core::Interface for ExponentialEase {
    type Vtable = IExponentialEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cb9e41d_f0bb_4bca_9da5_9ba3a11734c4);
}
impl ::windows::core::RuntimeName for ExponentialEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ExponentialEase";
}
impl ::core::convert::From<ExponentialEase> for ::windows::core::IUnknown {
    fn from(value: ExponentialEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExponentialEase> for ::windows::core::IUnknown {
    fn from(value: &ExponentialEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExponentialEase> for ::windows::core::IInspectable {
    fn from(value: ExponentialEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExponentialEase> for ::windows::core::IInspectable {
    fn from(value: &ExponentialEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ExponentialEase> for EasingFunctionBase {
    fn from(value: ExponentialEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&ExponentialEase> for EasingFunctionBase {
    fn from(value: &ExponentialEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ExponentialEase> for super::super::DependencyObject {
    fn from(value: ExponentialEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ExponentialEase> for super::super::DependencyObject {
    fn from(value: &ExponentialEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ExponentialEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ExponentialEase {}
unsafe impl ::core::marker::Sync for ExponentialEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FadeInThemeAnimation(pub ::windows::core::IInspectable);
impl FadeInThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeInThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFadeInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IFadeInThemeAnimationStatics<R, F: FnOnce(&IFadeInThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeInThemeAnimation, IFadeInThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FadeInThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.FadeInThemeAnimation;{6d4bc8f5-a918-4477-8078-554c68812ab8})");
}
unsafe impl ::windows::core::Interface for FadeInThemeAnimation {
    type Vtable = IFadeInThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4bc8f5_a918_4477_8078_554c68812ab8);
}
impl ::windows::core::RuntimeName for FadeInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.FadeInThemeAnimation";
}
impl ::core::convert::From<FadeInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: FadeInThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &FadeInThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: FadeInThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &FadeInThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for Timeline {
    fn from(value: FadeInThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for Timeline {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for super::super::DependencyObject {
    fn from(value: FadeInThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for FadeInThemeAnimation {}
unsafe impl ::core::marker::Sync for FadeInThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FadeOutThemeAnimation(pub ::windows::core::IInspectable);
impl FadeOutThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeOutThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IFadeOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IFadeOutThemeAnimationStatics<R, F: FnOnce(&IFadeOutThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FadeOutThemeAnimation, IFadeOutThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FadeOutThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.FadeOutThemeAnimation;{89276ba9-ffd4-45b6-9b9a-ced48951e712})");
}
unsafe impl ::windows::core::Interface for FadeOutThemeAnimation {
    type Vtable = IFadeOutThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89276ba9_ffd4_45b6_9b9a_ced48951e712);
}
impl ::windows::core::RuntimeName for FadeOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.FadeOutThemeAnimation";
}
impl ::core::convert::From<FadeOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: FadeOutThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: FadeOutThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for Timeline {
    fn from(value: FadeOutThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for Timeline {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: FadeOutThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for FadeOutThemeAnimation {}
unsafe impl ::core::marker::Sync for FadeOutThemeAnimation {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FillBehavior(pub i32);
impl FillBehavior {
    pub const HoldEnd: FillBehavior = FillBehavior(0i32);
    pub const Stop: FillBehavior = FillBehavior(1i32);
}
impl ::core::convert::From<i32> for FillBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FillBehavior {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FillBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.FillBehavior;i4)");
}
impl ::windows::core::DefaultType for FillBehavior {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GravityConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
impl GravityConnectedAnimationConfiguration {
    pub fn new() -> ::windows::core::Result<GravityConnectedAnimationConfiguration> {
        Self::IGravityConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<GravityConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn IsShadowEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGravityConnectedAnimationConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsShadowEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGravityConnectedAnimationConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IGravityConnectedAnimationConfigurationFactory<R, F: FnOnce(&IGravityConnectedAnimationConfigurationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GravityConnectedAnimationConfiguration, IGravityConnectedAnimationConfigurationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GravityConnectedAnimationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.GravityConnectedAnimationConfiguration;{c751a4b7-0459-5142-b891-aeaac1d41822})");
}
unsafe impl ::windows::core::Interface for GravityConnectedAnimationConfiguration {
    type Vtable = IGravityConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc751a4b7_0459_5142_b891_aeaac1d41822);
}
impl ::windows::core::RuntimeName for GravityConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.GravityConnectedAnimationConfiguration";
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        ::core::convert::Into::<ConnectedAnimationConfiguration>::into(&value)
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ConnectedAnimationConfiguration> for &GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GravityConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for GravityConnectedAnimationConfiguration {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAddDeleteThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAddDeleteThemeTransition {
    type Vtable = IAddDeleteThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadec852e_4424_4dab_99c1_3a04e36a3c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddDeleteThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackEase {
    type Vtable = IBackEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe47796e7_f805_4a8f_81c9_38e6472caa94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackEaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackEaseStatics {
    type Vtable = IBackEaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c70a2ff_a0a0_4786_926c_22321f8f25b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackEaseStatics_abi(
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
pub struct IBasicConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBasicConnectedAnimationConfiguration {
    type Vtable = IBasicConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe675f9b5_a4d6_5353_83e6_c89e7cf8d456);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicConnectedAnimationConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBasicConnectedAnimationConfigurationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBasicConnectedAnimationConfigurationFactory {
    type Vtable = IBasicConnectedAnimationConfigurationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95e6844a_4377_503c_bee2_11dfcd5570e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicConnectedAnimationConfigurationFactory_abi(
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
pub struct IBeginStoryboard(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBeginStoryboard {
    type Vtable = IBeginStoryboard_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64189fcd_49ec_4e52_a6f6_55324c921053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBeginStoryboard_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBeginStoryboardStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBeginStoryboardStatics {
    type Vtable = IBeginStoryboardStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12cff18c_aa91_4c4a_b82f_df34fc57f94b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBeginStoryboardStatics_abi(
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
pub struct IBounceEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBounceEase {
    type Vtable = IBounceEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bf1464e_fc71_47ed_85a1_3ba9577718b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBounceEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBounceEaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBounceEaseStatics {
    type Vtable = IBounceEaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0701da2_4f73_41c9_b2cb_2ea3105107ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBounceEaseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICircleEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICircleEase {
    type Vtable = ICircleEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53a3bdb2_9177_4e6e_a043_5082d889ab1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICircleEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorAnimation {
    type Vtable = IColorAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8ae8a15_0f63_4694_9467_bdafac1253ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorAnimationStatics {
    type Vtable = IColorAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55eaf6e2_87e3_4f48_958f_855b2f9ea9ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorAnimationUsingKeyFrames {
    type Vtable = IColorAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c82640_13c3_42aa_9ae2_7e6b51c92f95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationUsingKeyFrames_abi(
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
pub struct IColorAnimationUsingKeyFramesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorAnimationUsingKeyFramesStatics {
    type Vtable = IColorAnimationUsingKeyFramesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4723cdc_96e9_48f9_8d92_9b648b2f1cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationUsingKeyFramesStatics_abi(
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
pub struct IColorKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorKeyFrame {
    type Vtable = IColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb51d82d9_0910_4589_a284_b0c9205858e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorKeyFrameFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorKeyFrameFactory {
    type Vtable = IColorKeyFrameFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x769bd88a_9cfb_4a7d_96c4_a1e7de6fdb4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrameFactory_abi(
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
pub struct IColorKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IColorKeyFrameStatics {
    type Vtable = IColorKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc043ae99_210c_430f_9da5_df1082692055);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrameStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommonNavigationTransitionInfo {
    type Vtable = ICommonNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50345692_a555_4624_a361_0a91c1706473);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommonNavigationTransitionInfoStatics {
    type Vtable = ICommonNavigationTransitionInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3efe33_50be_4443_883c_e5627201c2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectedAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectedAnimation {
    type Vtable = IConnectedAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3518628c_f387_4c25_ac98_44e86c3cadf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectedAnimation2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectedAnimation2 {
    type Vtable = IConnectedAnimation2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2f8e5c_584b_4ddd_b668_973891431459);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, coordinatedelements: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, component: ConnectedAnimationComponent, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectedAnimation3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectedAnimation3 {
    type Vtable = IConnectedAnimation3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e3040c6_0430_59c0_a80c_cceed2e778dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectedAnimationConfiguration {
    type Vtable = IConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00218aae_cd8c_5651_92a0_c1db95c03998);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectedAnimationConfigurationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectedAnimationConfigurationFactory {
    type Vtable = IConnectedAnimationConfigurationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f9b84b_dd7e_593e_bf75_e959dc0ec52a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationConfigurationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectedAnimationService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectedAnimationService {
    type Vtable = IConnectedAnimationService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c6875c9_19bb_4d47_b9aa_66c802dcb9ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectedAnimationServiceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectedAnimationServiceStatics {
    type Vtable = IConnectedAnimationServiceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7078ea5_d688_40e8_8f90_96a6279273d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationServiceStatics_abi(
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
pub struct IContentThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContentThemeTransition {
    type Vtable = IContentThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf66fc5c3_5915_437d_8e3b_adf8e7f0ab57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContentThemeTransitionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContentThemeTransitionStatics {
    type Vtable = IContentThemeTransitionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e8ee385_9a42_4459_afa9_337dc41e1587);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentThemeTransitionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContinuumNavigationTransitionInfo {
    type Vtable = IContinuumNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4be1dbad_8ba6_4004_8438_8a9017978543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContinuumNavigationTransitionInfoStatics {
    type Vtable = IContinuumNavigationTransitionInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e25dd53_b18f_4bf1_b3bc_92f516f29903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICubicEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICubicEase {
    type Vtable = ICubicEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b94fc76_dad7_4354_b1a2_7969fbf6a70d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICubicEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDirectConnectedAnimationConfiguration {
    type Vtable = IDirectConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5d736f_5738_5d86_b770_151948cf365e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfigurationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDirectConnectedAnimationConfigurationFactory {
    type Vtable = IDirectConnectedAnimationConfigurationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x059263e9_d2b3_5a77_9cf4_e26d8b542608);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfigurationFactory_abi(
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
pub struct IDiscreteColorKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiscreteColorKeyFrame {
    type Vtable = IDiscreteColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230c08f4_e062_4cb1_8e2a_14093d73ed8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteColorKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDiscreteDoubleKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiscreteDoubleKeyFrame {
    type Vtable = IDiscreteDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f51f3a_ad11_49ce_8e1c_08fdf1447446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteDoubleKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDiscreteObjectKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiscreteObjectKeyFrame {
    type Vtable = IDiscreteObjectKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7dcde89_f12d_4a9c_8199_e7a9ece3a473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteObjectKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDiscretePointKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDiscretePointKeyFrame {
    type Vtable = IDiscretePointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0a9070d_4c42_4a90_983a_75f5a83a2fbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscretePointKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDoubleAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleAnimation {
    type Vtable = IDoubleAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e9f3d59_0f07_4bc9_977d_03763ff8154f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDoubleAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleAnimationStatics {
    type Vtable = IDoubleAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe27a935d_f111_43b7_b824_832b58d7786b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDoubleAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleAnimationUsingKeyFrames {
    type Vtable = IDoubleAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fee628f_bfee_4f75_83c2_a93b39488473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationUsingKeyFrames_abi(
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
pub struct IDoubleAnimationUsingKeyFramesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleAnimationUsingKeyFramesStatics {
    type Vtable = IDoubleAnimationUsingKeyFramesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x109bf2f6_c60f_49aa_abf6_f696d492116b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationUsingKeyFramesStatics_abi(
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
pub struct IDoubleKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleKeyFrame {
    type Vtable = IDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674456fd_e81e_4f4e_b4ad_0acfed9ecd68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDoubleKeyFrameFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleKeyFrameFactory {
    type Vtable = IDoubleKeyFrameFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac97dec3_7538_40b9_b152_696f7fbf4722);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrameFactory_abi(
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
pub struct IDoubleKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleKeyFrameStatics {
    type Vtable = IDoubleKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x324641b0_7d37_427a_adeb_43f38bb61a4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrameStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragItemThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragItemThemeAnimation {
    type Vtable = IDragItemThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c7d5db5_7ed6_4949_b4e6_a78c9f4f978d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragItemThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragItemThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragItemThemeAnimationStatics {
    type Vtable = IDragItemThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6218b9f5_013a_4fb1_86fc_92bc4e8d0241);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragItemThemeAnimationStatics_abi(
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
pub struct IDragOverThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragOverThemeAnimation {
    type Vtable = IDragOverThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72f762f7_7e51_4a6b_b937_dc4b4c1c5458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOverThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragOverThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragOverThemeAnimationStatics {
    type Vtable = IDragOverThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146ffe57_3c9d_41d9_a5ff_8d7239516810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOverThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDrillInNavigationTransitionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDrillInNavigationTransitionInfo {
    type Vtable = IDrillInNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b86201a_45d3_463b_939e_c8595f439bcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInNavigationTransitionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDrillInThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDrillInThemeAnimation {
    type Vtable = IDrillInThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb090b824_f1d2_41b8_87ba_78034126594c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDrillInThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDrillInThemeAnimationStatics {
    type Vtable = IDrillInThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61fe488_a17a_4b11_b53b_a4f1a07d4ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDrillOutThemeAnimation {
    type Vtable = IDrillOutThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd890ccdf_06d3_4f7e_8e4a_4fb76e256139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDrillOutThemeAnimationStatics {
    type Vtable = IDrillOutThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb5db9b_2617_4888_80dd_72fa7bb6fac3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDropTargetItemThemeAnimation {
    type Vtable = IDropTargetItemThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1881c968_1824_462b_87e8_c357212b977b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDropTargetItemThemeAnimationStatics {
    type Vtable = IDropTargetItemThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae80f486_2e56_4513_bf18_d77470164ae5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimationStatics_abi(
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
pub struct IEasingColorKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingColorKeyFrame {
    type Vtable = IEasingColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc733d630_f4b9_4934_9bdd_27ac5ed1cfd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingColorKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasingColorKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingColorKeyFrameStatics {
    type Vtable = IEasingColorKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f3837fc_8e3d_4522_9b0f_003db8609851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingColorKeyFrameStatics_abi(
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
pub struct IEasingDoubleKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingDoubleKeyFrame {
    type Vtable = IEasingDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965adb8d_9a54_4108_b4ff_b5a5212cb338);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingDoubleKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasingDoubleKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingDoubleKeyFrameStatics {
    type Vtable = IEasingDoubleKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8d3d845_dbae_4e5b_8b84_d9537398e5b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingDoubleKeyFrameStatics_abi(
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
pub struct IEasingFunctionBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingFunctionBase {
    type Vtable = IEasingFunctionBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc108383f_2c02_4151_8ecd_68ddaa3f0d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: EasingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, normalizedtime: f64, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasingFunctionBaseFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingFunctionBaseFactory {
    type Vtable = IEasingFunctionBaseFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1830fe6a_f01b_43e0_b61f_b452a1c66fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBaseFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasingFunctionBaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingFunctionBaseStatics {
    type Vtable = IEasingFunctionBaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a5031aa_2c50_4a1d_bb04_d75e07b71548);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBaseStatics_abi(
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
pub struct IEasingPointKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingPointKeyFrame {
    type Vtable = IEasingPointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c91380_6868_4225_a70b_3981cc0b2947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingPointKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasingPointKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasingPointKeyFrameStatics {
    type Vtable = IEasingPointKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe22dbfc4_080c_402c_a6b5_f48d0a98116b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingPointKeyFrameStatics_abi(
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
pub struct IEdgeUIThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEdgeUIThemeTransition {
    type Vtable = IEdgeUIThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c86c19b_49d7_19ec_cf19_83a73c6de75e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeUIThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEdgeUIThemeTransitionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEdgeUIThemeTransitionStatics {
    type Vtable = IEdgeUIThemeTransitionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a2b13b_4705_302b_27c6_2aac92f645ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeUIThemeTransitionStatics_abi(
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
pub struct IElasticEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElasticEase {
    type Vtable = IElasticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5ba58c_b0b6_4a6c_9ca8_fb4233f12459);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElasticEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IElasticEaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElasticEaseStatics {
    type Vtable = IElasticEaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9f566ec_fe9c_4b2b_8e52_bb785d562185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElasticEaseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEntranceNavigationTransitionInfo {
    type Vtable = IEntranceNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x720a256b_1c8a_41ee_82ec_8a87c0cf47da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEntranceNavigationTransitionInfoStatics {
    type Vtable = IEntranceNavigationTransitionInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf948c27a_40c9_469f_8f33_bf45c8811f21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEntranceThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEntranceThemeTransition {
    type Vtable = IEntranceThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07698c09_a8e3_419a_a01d_7410a0ae8ec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEntranceThemeTransitionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEntranceThemeTransitionStatics {
    type Vtable = IEntranceThemeTransitionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37cc0577_ff98_4aed_b86e_5ec23702f877);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceThemeTransitionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExponentialEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExponentialEase {
    type Vtable = IExponentialEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cb9e41d_f0bb_4bca_9da5_9ba3a11734c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExponentialEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExponentialEaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExponentialEaseStatics {
    type Vtable = IExponentialEaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf37ee7e3_a761_4352_9ad6_70794567581a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExponentialEaseStatics_abi(
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
pub struct IFadeInThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFadeInThemeAnimation {
    type Vtable = IFadeInThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4bc8f5_a918_4477_8078_554c68812ab8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeInThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFadeInThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFadeInThemeAnimationStatics {
    type Vtable = IFadeInThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f0117e1_bea9_4923_b23a_0ddf4d7b8737);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeInThemeAnimationStatics_abi(
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
pub struct IFadeOutThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFadeOutThemeAnimation {
    type Vtable = IFadeOutThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89276ba9_ffd4_45b6_9b9a_ced48951e712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeOutThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFadeOutThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFadeOutThemeAnimationStatics {
    type Vtable = IFadeOutThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe17a81a_4168_4f68_a28c_e5dd98cf680f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeOutThemeAnimationStatics_abi(
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
pub struct IGravityConnectedAnimationConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGravityConnectedAnimationConfiguration {
    type Vtable = IGravityConnectedAnimationConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc751a4b7_0459_5142_b891_aeaac1d41822);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfiguration2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGravityConnectedAnimationConfiguration2 {
    type Vtable = IGravityConnectedAnimationConfiguration2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62333add_aed4_5fed_95ff_d128acce8be4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfigurationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGravityConnectedAnimationConfigurationFactory {
    type Vtable = IGravityConnectedAnimationConfigurationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe822c41f_3656_5090_92f5_c217eaacb682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfigurationFactory_abi(
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
pub struct IKeySpline(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeySpline {
    type Vtable = IKeySpline_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a163bb_d5ca_4a32_ba0b_7dff988e58a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeySpline_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyTimeHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyTimeHelper {
    type Vtable = IKeyTimeHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3643e480_4823_466a_abe5_5e79c8ed77ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyTimeHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyTimeHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyTimeHelperStatics {
    type Vtable = IKeyTimeHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa2612c_22a9_45e9_9af7_c7416efff7a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyTimeHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timespan: super::super::super::super::Foundation::TimeSpan, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILinearColorKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILinearColorKeyFrame {
    type Vtable = ILinearColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66fdb6ef_ac81_4611_b1d2_61f545983f03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearColorKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILinearDoubleKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILinearDoubleKeyFrame {
    type Vtable = ILinearDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efdf265_9a7b_431d_8f0c_14c56b5ea4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearDoubleKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILinearPointKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILinearPointKeyFrame {
    type Vtable = ILinearPointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7c9b8ef_af24_49ee_84f1_a86600a4e319);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearPointKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationThemeTransition {
    type Vtable = INavigationThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8833848c_4eb7_41f2_8799_9eef0a213b73);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationThemeTransitionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationThemeTransitionStatics {
    type Vtable = INavigationThemeTransitionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea2f06e0_5e60_4f8e_bcaf_431487a294ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationThemeTransitionStatics_abi(
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
pub struct INavigationTransitionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationTransitionInfo {
    type Vtable = INavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b05091_ae4a_4372_8625_21b7a8b98ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationTransitionInfoFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationTransitionInfoFactory {
    type Vtable = INavigationTransitionInfoFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedf4f8d5_af63_4fab_9d4a_87927f82dd6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfoFactory_abi(
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
pub struct INavigationTransitionInfoOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationTransitionInfoOverrides {
    type Vtable = INavigationTransitionInfoOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9517e6a_a9d0_4bf7_9db0_4633a69daff2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfoOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, navigationstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IObjectAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IObjectAnimationUsingKeyFrames {
    type Vtable = IObjectAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334a2d92_b74a_4c64_b9a6_58bcfa314f22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectAnimationUsingKeyFrames_abi(
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
pub struct IObjectAnimationUsingKeyFramesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IObjectAnimationUsingKeyFramesStatics {
    type Vtable = IObjectAnimationUsingKeyFramesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb736182_6af1_49a3_97b6_783ed97400fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectAnimationUsingKeyFramesStatics_abi(
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
pub struct IObjectKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IObjectKeyFrame {
    type Vtable = IObjectKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9852a851_8593_48ee_a6a4_d5d4720f029a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IObjectKeyFrameFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IObjectKeyFrameFactory {
    type Vtable = IObjectKeyFrameFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1626143e_3e6d_44d8_9b9a_04aea70f8492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrameFactory_abi(
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
pub struct IObjectKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IObjectKeyFrameStatics {
    type Vtable = IObjectKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd6ab00_5319_4286_8eed_4e755ea0cf9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrameStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaneThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaneThemeTransition {
    type Vtable = IPaneThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4708eb8e_4bfc_ee46_d4f9_708def3fbb2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaneThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaneThemeTransitionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaneThemeTransitionStatics {
    type Vtable = IPaneThemeTransitionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x316b382f_4be4_1797_b45c_cd900bbe0caa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaneThemeTransitionStatics_abi(
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
pub struct IPointAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointAnimation {
    type Vtable = IPointAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f04312_7726_4f88_b8e2_2fa54518963b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointAnimationStatics {
    type Vtable = IPointAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f99b356_e737_408b_a0fd_327826d32255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointAnimationUsingKeyFrames {
    type Vtable = IPointAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b944f72_446a_41d0_a129_41a620f4595d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationUsingKeyFrames_abi(
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
pub struct IPointAnimationUsingKeyFramesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointAnimationUsingKeyFramesStatics {
    type Vtable = IPointAnimationUsingKeyFramesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f454c87_2390_46ea_baa7_762f4bc30d04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationUsingKeyFramesStatics_abi(
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
pub struct IPointKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointKeyFrame {
    type Vtable = IPointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcc88d01_7f82_4dae_8026_7b7e086878b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: KeyTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointKeyFrameFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointKeyFrameFactory {
    type Vtable = IPointKeyFrameFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb214bdf_426a_4392_8355_c2ae52852623);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrameFactory_abi(
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
pub struct IPointKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointKeyFrameStatics {
    type Vtable = IPointKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95cf1b27_7965_4bec_b9fb_fbe94b65518e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrameStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerDownThemeAnimation {
    type Vtable = IPointerDownThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb58e714e_c49d_4788_a233_0ae85d99dd5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerDownThemeAnimationStatics {
    type Vtable = IPointerDownThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a7cb7b_6d46_4494_b94a_e72f3b492a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimationStatics_abi(
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
pub struct IPointerUpThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerUpThemeAnimation {
    type Vtable = IPointerUpThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9e9d07d_6340_4828_ad12_690694b9910b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerUpThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerUpThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerUpThemeAnimationStatics {
    type Vtable = IPointerUpThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c618f9c_7992_4139_8bfc_0883b9727a7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerUpThemeAnimationStatics_abi(
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
pub struct IPopInThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopInThemeAnimation {
    type Vtable = IPopInThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196938c1_1c07_4c28_8847_f9f055b32855);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopInThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopInThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopInThemeAnimationStatics {
    type Vtable = IPopInThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefaa99d3_218a_4701_977f_f1bfae8ba649);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopInThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopOutThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopOutThemeAnimation {
    type Vtable = IPopOutThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4786ab49_0e48_4e81_a2e5_cc5aa19e48d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopOutThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopOutThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopOutThemeAnimationStatics {
    type Vtable = IPopOutThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d492c09_03c1_4490_99dc_909feab357fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopOutThemeAnimationStatics_abi(
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
pub struct IPopupThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopupThemeTransition {
    type Vtable = IPopupThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47843552_4283_545e_c791_268dca22ce4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopupThemeTransitionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopupThemeTransitionStatics {
    type Vtable = IPopupThemeTransitionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5a1640e_490d_1505_9f6b_8fafc044dec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupThemeTransitionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPowerEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPowerEase {
    type Vtable = IPowerEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c80579_eedf_405b_8680_d9606880c937);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPowerEaseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPowerEaseStatics {
    type Vtable = IPowerEaseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5955103_91a2_460c_9c41_d28f6a939bda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerEaseStatics_abi(
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
pub struct IQuadraticEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IQuadraticEase {
    type Vtable = IQuadraticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1510e91_ef6d_44f0_803d_68d16de0ddfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IQuarticEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IQuarticEase {
    type Vtable = IQuarticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8698814_fe42_4a05_b5b8_081f41157815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuarticEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IQuinticEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IQuinticEase {
    type Vtable = IQuinticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ee793b_3c49_4108_aa11_ab786603da21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuinticEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReorderThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReorderThemeTransition {
    type Vtable = IReorderThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2065c6c_d052_4ad1_8362_b71b36df7497);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReorderThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepeatBehaviorHelper {
    type Vtable = IRepeatBehaviorHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6863ab72_4997_47f9_87ad_37efb75993ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepeatBehaviorHelperStatics {
    type Vtable = IRepeatBehaviorHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a795033_79f3_4dd9_b267_9cf50fb51f84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: f64, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, duration: super::super::super::super::Foundation::TimeSpan, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: RepeatBehavior, value: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepositionThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepositionThemeAnimation {
    type Vtable = IRepositionThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecda24e8_8945_4949_a1bf_62109965a7e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepositionThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepositionThemeAnimationStatics {
    type Vtable = IRepositionThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d92b1b1_860b_4bf9_a59d_1eb1ccbe8fe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepositionThemeTransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepositionThemeTransition {
    type Vtable = IRepositionThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88329b82_98f3_455a_ac53_2e7083b6e22c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepositionThemeTransition2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepositionThemeTransition2 {
    type Vtable = IRepositionThemeTransition2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcebfe864_dbea_4404_8e6e_de55ada75239);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransition2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRepositionThemeTransitionStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRepositionThemeTransitionStatics2 {
    type Vtable = IRepositionThemeTransitionStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9240e930_0a19_468b_8c2a_68fab4500027);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransitionStatics2_abi(
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
pub struct ISineEase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISineEase {
    type Vtable = ISineEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9382962_230b_49da_9e0d_664987892343);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISineEase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISlideNavigationTransitionInfo {
    type Vtable = ISlideNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6ac9d77_2e03_405f_80ed_e62beef3668f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISlideNavigationTransitionInfo2 {
    type Vtable = ISlideNavigationTransitionInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90e2d9c0_5c81_5001_8013_4fbfea4bf139);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SlideNavigationTransitionEffect) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SlideNavigationTransitionEffect) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfoStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISlideNavigationTransitionInfoStatics2 {
    type Vtable = ISlideNavigationTransitionInfoStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a861baa_981a_5ace_9f85_cb7fde648a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfoStatics2_abi(
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
pub struct ISplineColorKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplineColorKeyFrame {
    type Vtable = ISplineColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4a5941_1fe0_473a_8efe_4316d8c86229);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineColorKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplineColorKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplineColorKeyFrameStatics {
    type Vtable = ISplineColorKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61d1d997_8589_4f2f_8fbb_7d03edc98dd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineColorKeyFrameStatics_abi(
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
pub struct ISplineDoubleKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplineDoubleKeyFrame {
    type Vtable = ISplineDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00d72d38_6b2b_4843_838e_c8b115eec801);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineDoubleKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplineDoubleKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplineDoubleKeyFrameStatics {
    type Vtable = ISplineDoubleKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060a8ffc_975f_4e4e_9ec7_13c5aee02062);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineDoubleKeyFrameStatics_abi(
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
pub struct ISplinePointKeyFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplinePointKeyFrame {
    type Vtable = ISplinePointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f19f306_7036_494f_bc3c_780df0cc524a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplinePointKeyFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplinePointKeyFrameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplinePointKeyFrameStatics {
    type Vtable = ISplinePointKeyFrameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe97a32c2_0a7a_4766_95cb_0d692611cb4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplinePointKeyFrameStatics_abi(
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
pub struct ISplitCloseThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplitCloseThemeAnimation {
    type Vtable = ISplitCloseThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f799518_ff39_4e90_bb74_2abd56027402);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitCloseThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplitCloseThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplitCloseThemeAnimationStatics {
    type Vtable = ISplitCloseThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7aa94de9_cc9b_4e90_a11a_0050a2216a9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitCloseThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplitOpenThemeAnimation {
    type Vtable = ISplitOpenThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x785fd7aa_5456_4639_8fd2_26bae6a5ffe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplitOpenThemeAnimationStatics {
    type Vtable = ISplitOpenThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d4cfa89_3a91_458d_b0fb_4cad625cbf8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStoryboard(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStoryboard {
    type Vtable = IStoryboard_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45c1e6e_3594_460e_981a_32271bd3aa06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoryboard_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClockState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStoryboardStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStoryboardStatics {
    type Vtable = IStoryboardStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd82f07d8_73d5_4379_bd48_7e05184a8bad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoryboardStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeline: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISuppressNavigationTransitionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISuppressNavigationTransitionInfo {
    type Vtable = ISuppressNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x244d7b0c_b1b7_4871_9d3e_d56203a3a5b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuppressNavigationTransitionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISwipeBackThemeAnimation {
    type Vtable = ISwipeBackThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa38a4214_0bca_4d2d_95f7_ceba57fbaf60);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISwipeBackThemeAnimationStatics {
    type Vtable = ISwipeBackThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x693f31bf_4da6_468a_8ce0_996c9aad42e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISwipeHintThemeAnimation {
    type Vtable = ISwipeHintThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdd067c0_580e_4e40_be98_f202d3d84365);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISwipeHintThemeAnimationStatics {
    type Vtable = ISwipeHintThemeAnimationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23d61a57_9115_4d63_b04a_b89f1c744dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimeline(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimeline {
    type Vtable = ITimeline_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc465dc_be4d_4d0d_9549_2208b715f40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeline_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Duration) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Duration) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FillBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FillBehavior) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: RepeatBehavior) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimelineFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimelineFactory {
    type Vtable = ITimelineFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d56bb07_bda4_478b_8ada_eb04d580cd5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineFactory_abi(
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
pub struct ITimelineStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimelineStatics {
    type Vtable = ITimelineStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa902ed4e_ef10_4d6f_9a40_93cb8895f4e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransition {
    type Vtable = ITransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c677c7c_01d0_4dce_b333_976f93312b08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransitionFactory {
    type Vtable = ITransitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc9ab2cf_3bc9_44aa_b3fc_883a83233a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeySpline(pub ::windows::core::IInspectable);
impl KeySpline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeySpline, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn ControlPoint1(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetControlPoint1<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ControlPoint2(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetControlPoint2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for KeySpline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.KeySpline;{77a163bb-d5ca-4a32-ba0b-7dff988e58a0})");
}
unsafe impl ::windows::core::Interface for KeySpline {
    type Vtable = IKeySpline_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a163bb_d5ca_4a32_ba0b_7dff988e58a0);
}
impl ::windows::core::RuntimeName for KeySpline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.KeySpline";
}
impl ::core::convert::From<KeySpline> for ::windows::core::IUnknown {
    fn from(value: KeySpline) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeySpline> for ::windows::core::IUnknown {
    fn from(value: &KeySpline) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeySpline> for ::windows::core::IInspectable {
    fn from(value: KeySpline) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeySpline> for ::windows::core::IInspectable {
    fn from(value: &KeySpline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<KeySpline> for super::super::DependencyObject {
    fn from(value: KeySpline) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&KeySpline> for super::super::DependencyObject {
    fn from(value: &KeySpline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &KeySpline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for KeySpline {}
unsafe impl ::core::marker::Sync for KeySpline {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct KeyTime {
    pub TimeSpan: super::super::super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl KeyTime {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for KeyTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for KeyTime {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KeyTime").field("TimeSpan", &self.TimeSpan).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for KeyTime {
    fn eq(&self, other: &Self) -> bool {
        self.TimeSpan == other.TimeSpan
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for KeyTime {}
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyTimeHelper(pub ::windows::core::IInspectable);
impl KeyTimeHelper {
    #[cfg(feature = "Foundation")]
    pub fn FromTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(timespan: Param0) -> ::windows::core::Result<KeyTime> {
        Self::IKeyTimeHelperStatics(|this| unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timespan.into_param().abi(), &mut result__).from_abi::<KeyTime>(result__)
        })
    }
    pub fn IKeyTimeHelperStatics<R, F: FnOnce(&IKeyTimeHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyTimeHelper, IKeyTimeHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyTimeHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.KeyTimeHelper;{3643e480-4823-466a-abe5-5e79c8ed77ed})");
}
unsafe impl ::windows::core::Interface for KeyTimeHelper {
    type Vtable = IKeyTimeHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3643e480_4823_466a_abe5_5e79c8ed77ed);
}
impl ::windows::core::RuntimeName for KeyTimeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.KeyTimeHelper";
}
impl ::core::convert::From<KeyTimeHelper> for ::windows::core::IUnknown {
    fn from(value: KeyTimeHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyTimeHelper> for ::windows::core::IUnknown {
    fn from(value: &KeyTimeHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyTimeHelper> for ::windows::core::IInspectable {
    fn from(value: KeyTimeHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyTimeHelper> for ::windows::core::IInspectable {
    fn from(value: &KeyTimeHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyTimeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for KeyTimeHelper {}
unsafe impl ::core::marker::Sync for KeyTimeHelper {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LinearColorKeyFrame(pub ::windows::core::IInspectable);
impl LinearColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LinearColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearColorKeyFrame;{66fdb6ef-ac81-4611-b1d2-61f545983f03})");
}
unsafe impl ::windows::core::Interface for LinearColorKeyFrame {
    type Vtable = ILinearColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66fdb6ef_ac81_4611_b1d2_61f545983f03);
}
impl ::windows::core::RuntimeName for LinearColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearColorKeyFrame";
}
impl ::core::convert::From<LinearColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: LinearColorKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &LinearColorKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: LinearColorKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &LinearColorKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for ColorKeyFrame {
    fn from(value: LinearColorKeyFrame) -> Self {
        ::core::convert::Into::<ColorKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ColorKeyFrame {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearColorKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LinearColorKeyFrame {}
unsafe impl ::core::marker::Sync for LinearColorKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LinearDoubleKeyFrame(pub ::windows::core::IInspectable);
impl LinearDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LinearDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearDoubleKeyFrame;{8efdf265-9a7b-431d-8f0c-14c56b5ea4d9})");
}
unsafe impl ::windows::core::Interface for LinearDoubleKeyFrame {
    type Vtable = ILinearDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efdf265_9a7b_431d_8f0c_14c56b5ea4d9);
}
impl ::windows::core::RuntimeName for LinearDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearDoubleKeyFrame";
}
impl ::core::convert::From<LinearDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        ::core::convert::Into::<DoubleKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LinearDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for LinearDoubleKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LinearPointKeyFrame(pub ::windows::core::IInspectable);
impl LinearPointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LinearPointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LinearPointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearPointKeyFrame;{e7c9b8ef-af24-49ee-84f1-a86600a4e319})");
}
unsafe impl ::windows::core::Interface for LinearPointKeyFrame {
    type Vtable = ILinearPointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7c9b8ef_af24_49ee_84f1_a86600a4e319);
}
impl ::windows::core::RuntimeName for LinearPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearPointKeyFrame";
}
impl ::core::convert::From<LinearPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: LinearPointKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &LinearPointKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: LinearPointKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &LinearPointKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for PointKeyFrame {
    fn from(value: LinearPointKeyFrame) -> Self {
        ::core::convert::Into::<PointKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for PointKeyFrame {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearPointKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LinearPointKeyFrame {}
unsafe impl ::core::marker::Sync for LinearPointKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigationThemeTransition(pub ::windows::core::IInspectable);
impl NavigationThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NavigationThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DefaultNavigationTransitionInfo(&self) -> ::windows::core::Result<NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NavigationTransitionInfo>(result__)
        }
    }
    pub fn SetDefaultNavigationTransitionInfo<'a, Param0: ::windows::core::IntoParam<'a, NavigationTransitionInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DefaultNavigationTransitionInfoProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::INavigationThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn INavigationThemeTransitionStatics<R, F: FnOnce(&INavigationThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NavigationThemeTransition, INavigationThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.NavigationThemeTransition;{8833848c-4eb7-41f2-8799-9eef0a213b73})");
}
unsafe impl ::windows::core::Interface for NavigationThemeTransition {
    type Vtable = INavigationThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8833848c_4eb7_41f2_8799_9eef0a213b73);
}
impl ::windows::core::RuntimeName for NavigationThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.NavigationThemeTransition";
}
impl ::core::convert::From<NavigationThemeTransition> for ::windows::core::IUnknown {
    fn from(value: NavigationThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &NavigationThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigationThemeTransition> for ::windows::core::IInspectable {
    fn from(value: NavigationThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &NavigationThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NavigationThemeTransition> for Transition {
    fn from(value: NavigationThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for Transition {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<NavigationThemeTransition> for super::super::DependencyObject {
    fn from(value: NavigationThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for super::super::DependencyObject {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for NavigationThemeTransition {}
unsafe impl ::core::marker::Sync for NavigationThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigationTransitionInfo(pub ::windows::core::IInspectable);
impl NavigationTransitionInfo {}
unsafe impl ::windows::core::RuntimeType for NavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.NavigationTransitionInfo;{a9b05091-ae4a-4372-8625-21b7a8b98ca4})");
}
unsafe impl ::windows::core::Interface for NavigationTransitionInfo {
    type Vtable = INavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b05091_ae4a_4372_8625_21b7a8b98ca4);
}
impl ::windows::core::RuntimeName for NavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.NavigationTransitionInfo";
}
impl ::core::convert::From<NavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: NavigationTransitionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &NavigationTransitionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: NavigationTransitionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &NavigationTransitionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: NavigationTransitionInfo) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &NavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &NavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for NavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for NavigationTransitionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ObjectAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
impl ObjectAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<ObjectKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ObjectKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IObjectAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IObjectAnimationUsingKeyFramesStatics<R, F: FnOnce(&IObjectAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectAnimationUsingKeyFrames, IObjectAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ObjectAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectAnimationUsingKeyFrames;{334a2d92-b74a-4c64-b9a6-58bcfa314f22})");
}
unsafe impl ::windows::core::Interface for ObjectAnimationUsingKeyFrames {
    type Vtable = IObjectAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334a2d92_b74a_4c64_b9a6_58bcfa314f22);
}
impl ::windows::core::RuntimeName for ObjectAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectAnimationUsingKeyFrames";
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for Timeline {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for Timeline {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ObjectAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for ObjectAnimationUsingKeyFrames {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ObjectKeyFrame(pub ::windows::core::IInspectable);
impl ObjectKeyFrame {
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IObjectKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IObjectKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IObjectKeyFrameStatics<R, F: FnOnce(&IObjectKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectKeyFrame, IObjectKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ObjectKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrame;{9852a851-8593-48ee-a6a4-d5d4720f029a})");
}
unsafe impl ::windows::core::Interface for ObjectKeyFrame {
    type Vtable = IObjectKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9852a851_8593_48ee_a6a4_d5d4720f029a);
}
impl ::windows::core::RuntimeName for ObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectKeyFrame";
}
impl ::core::convert::From<ObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: ObjectKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &ObjectKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: ObjectKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &ObjectKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: ObjectKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: &ObjectKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ObjectKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ObjectKeyFrame {}
unsafe impl ::core::marker::Sync for ObjectKeyFrame {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ObjectKeyFrameCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ObjectKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ObjectKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ObjectKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ObjectKeyFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<ObjectKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<ObjectKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ObjectKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ObjectKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<ObjectKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<ObjectKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<ObjectKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<ObjectKeyFrame>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ObjectKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrame;{9852a851-8593-48ee-a6a4-d5d4720f029a})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ObjectKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVector_abi<ObjectKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ObjectKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ObjectKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: ObjectKeyFrameCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ObjectKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &ObjectKeyFrameCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ObjectKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: ObjectKeyFrameCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ObjectKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &ObjectKeyFrameCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ObjectKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame> {
    fn from(value: ObjectKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ObjectKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame> {
    fn from(value: &ObjectKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> for &ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<ObjectKeyFrame>> {
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
unsafe impl ::core::marker::Send for ObjectKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ObjectKeyFrameCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ObjectKeyFrameCollection {
    type Item = ObjectKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ObjectKeyFrameCollection {
    type Item = ObjectKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PaneThemeTransition(pub ::windows::core::IInspectable);
impl PaneThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaneThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::EdgeTransitionLocation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::EdgeTransitionLocation>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EdgeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPaneThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPaneThemeTransitionStatics<R, F: FnOnce(&IPaneThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaneThemeTransition, IPaneThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PaneThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PaneThemeTransition;{4708eb8e-4bfc-ee46-d4f9-708def3fbb2b})");
}
unsafe impl ::windows::core::Interface for PaneThemeTransition {
    type Vtable = IPaneThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4708eb8e_4bfc_ee46_d4f9_708def3fbb2b);
}
impl ::windows::core::RuntimeName for PaneThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PaneThemeTransition";
}
impl ::core::convert::From<PaneThemeTransition> for ::windows::core::IUnknown {
    fn from(value: PaneThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PaneThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &PaneThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PaneThemeTransition> for ::windows::core::IInspectable {
    fn from(value: PaneThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PaneThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &PaneThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PaneThemeTransition> for Transition {
    fn from(value: PaneThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&PaneThemeTransition> for Transition {
    fn from(value: &PaneThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PaneThemeTransition> for super::super::DependencyObject {
    fn from(value: PaneThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PaneThemeTransition> for super::super::DependencyObject {
    fn from(value: &PaneThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PaneThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PaneThemeTransition {}
unsafe impl ::core::marker::Sync for PaneThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointAnimation(pub ::windows::core::IInspectable);
impl PointAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows::core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ByProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EasingFunctionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointAnimationStatics<R, F: FnOnce(&IPointAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimation, IPointAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PointAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointAnimation;{30f04312-7726-4f88-b8e2-2fa54518963b})");
}
unsafe impl ::windows::core::Interface for PointAnimation {
    type Vtable = IPointAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f04312_7726_4f88_b8e2_2fa54518963b);
}
impl ::windows::core::RuntimeName for PointAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointAnimation";
}
impl ::core::convert::From<PointAnimation> for ::windows::core::IUnknown {
    fn from(value: PointAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointAnimation> for ::windows::core::IUnknown {
    fn from(value: &PointAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointAnimation> for ::windows::core::IInspectable {
    fn from(value: PointAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointAnimation> for ::windows::core::IInspectable {
    fn from(value: &PointAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointAnimation> for Timeline {
    fn from(value: PointAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&PointAnimation> for Timeline {
    fn from(value: &PointAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PointAnimation> for super::super::DependencyObject {
    fn from(value: PointAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PointAnimation> for super::super::DependencyObject {
    fn from(value: &PointAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PointAnimation {}
unsafe impl ::core::marker::Sync for PointAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointAnimationUsingKeyFrames(pub ::windows::core::IInspectable);
impl PointAnimationUsingKeyFrames {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimationUsingKeyFrames, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyFrames(&self) -> ::windows::core::Result<PointKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointAnimationUsingKeyFramesStatics<R, F: FnOnce(&IPointAnimationUsingKeyFramesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointAnimationUsingKeyFrames, IPointAnimationUsingKeyFramesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PointAnimationUsingKeyFrames {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointAnimationUsingKeyFrames;{9b944f72-446a-41d0-a129-41a620f4595d})");
}
unsafe impl ::windows::core::Interface for PointAnimationUsingKeyFrames {
    type Vtable = IPointAnimationUsingKeyFrames_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b944f72_446a_41d0_a129_41a620f4595d);
}
impl ::windows::core::RuntimeName for PointAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointAnimationUsingKeyFrames";
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for ::windows::core::IUnknown {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for ::windows::core::IInspectable {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for Timeline {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for Timeline {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PointAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for PointAnimationUsingKeyFrames {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointKeyFrame(pub ::windows::core::IInspectable);
impl PointKeyFrame {
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyTime(&self) -> ::windows::core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__: KeyTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetKeyTime<'a, Param0: ::windows::core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointKeyFrameStatics<R, F: FnOnce(&IPointKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointKeyFrame, IPointKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointKeyFrame;{fcc88d01-7f82-4dae-8026-7b7e086878b3})");
}
unsafe impl ::windows::core::Interface for PointKeyFrame {
    type Vtable = IPointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcc88d01_7f82_4dae_8026_7b7e086878b3);
}
impl ::windows::core::RuntimeName for PointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointKeyFrame";
}
impl ::core::convert::From<PointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: PointKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &PointKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: PointKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &PointKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointKeyFrame> for super::super::DependencyObject {
    fn from(value: PointKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PointKeyFrame> for super::super::DependencyObject {
    fn from(value: &PointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PointKeyFrame {}
unsafe impl ::core::marker::Sync for PointKeyFrame {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointKeyFrameCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PointKeyFrameCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointKeyFrameCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<PointKeyFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<PointKeyFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<PointKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<PointKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, PointKeyFrame>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<PointKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<PointKeyFrame as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<PointKeyFrame>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<PointKeyFrame>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<PointKeyFrame>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PointKeyFrameCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.PointKeyFrame;{fcc88d01-7f82-4dae-8026-7b7e086878b3})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PointKeyFrameCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVector_abi<PointKeyFrame>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::super::Foundation::Collections::IVector<PointKeyFrame> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PointKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointKeyFrameCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: PointKeyFrameCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointKeyFrameCollection> for ::windows::core::IUnknown {
    fn from(value: &PointKeyFrameCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: PointKeyFrameCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointKeyFrameCollection> for ::windows::core::IInspectable {
    fn from(value: &PointKeyFrameCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PointKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<PointKeyFrame> {
    fn from(value: PointKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PointKeyFrameCollection> for super::super::super::super::Foundation::Collections::IVector<PointKeyFrame> {
    fn from(value: &PointKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> for &PointKeyFrameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<PointKeyFrame>> {
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
unsafe impl ::core::marker::Send for PointKeyFrameCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PointKeyFrameCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PointKeyFrameCollection {
    type Item = PointKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PointKeyFrameCollection {
    type Item = PointKeyFrame;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointerDownThemeAnimation(pub ::windows::core::IInspectable);
impl PointerDownThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerDownThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointerDownThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointerDownThemeAnimationStatics<R, F: FnOnce(&IPointerDownThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerDownThemeAnimation, IPointerDownThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PointerDownThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointerDownThemeAnimation;{b58e714e-c49d-4788-a233-0ae85d99dd5a})");
}
unsafe impl ::windows::core::Interface for PointerDownThemeAnimation {
    type Vtable = IPointerDownThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb58e714e_c49d_4788_a233_0ae85d99dd5a);
}
impl ::windows::core::RuntimeName for PointerDownThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointerDownThemeAnimation";
}
impl ::core::convert::From<PointerDownThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PointerDownThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PointerDownThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for Timeline {
    fn from(value: PointerDownThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for Timeline {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for super::super::DependencyObject {
    fn from(value: PointerDownThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PointerDownThemeAnimation {}
unsafe impl ::core::marker::Sync for PointerDownThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointerUpThemeAnimation(pub ::windows::core::IInspectable);
impl PointerUpThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerUpThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPointerUpThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointerUpThemeAnimationStatics<R, F: FnOnce(&IPointerUpThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerUpThemeAnimation, IPointerUpThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PointerUpThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointerUpThemeAnimation;{e9e9d07d-6340-4828-ad12-690694b9910b})");
}
unsafe impl ::windows::core::Interface for PointerUpThemeAnimation {
    type Vtable = IPointerUpThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9e9d07d_6340_4828_ad12_690694b9910b);
}
impl ::windows::core::RuntimeName for PointerUpThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointerUpThemeAnimation";
}
impl ::core::convert::From<PointerUpThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PointerUpThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PointerUpThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for Timeline {
    fn from(value: PointerUpThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for Timeline {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for super::super::DependencyObject {
    fn from(value: PointerUpThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PointerUpThemeAnimation {}
unsafe impl ::core::marker::Sync for PointerUpThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PopInThemeAnimation(pub ::windows::core::IInspectable);
impl PopInThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopInThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopInThemeAnimationStatics<R, F: FnOnce(&IPopInThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopInThemeAnimation, IPopInThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PopInThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopInThemeAnimation;{196938c1-1c07-4c28-8847-f9f055b32855})");
}
unsafe impl ::windows::core::Interface for PopInThemeAnimation {
    type Vtable = IPopInThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196938c1_1c07_4c28_8847_f9f055b32855);
}
impl ::windows::core::RuntimeName for PopInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopInThemeAnimation";
}
impl ::core::convert::From<PopInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PopInThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PopInThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PopInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PopInThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PopInThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PopInThemeAnimation> for Timeline {
    fn from(value: PopInThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for Timeline {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PopInThemeAnimation> for super::super::DependencyObject {
    fn from(value: PopInThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PopInThemeAnimation {}
unsafe impl ::core::marker::Sync for PopInThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PopOutThemeAnimation(pub ::windows::core::IInspectable);
impl PopOutThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopOutThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopOutThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopOutThemeAnimationStatics<R, F: FnOnce(&IPopOutThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopOutThemeAnimation, IPopOutThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PopOutThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopOutThemeAnimation;{4786ab49-0e48-4e81-a2e5-cc5aa19e48d3})");
}
unsafe impl ::windows::core::Interface for PopOutThemeAnimation {
    type Vtable = IPopOutThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4786ab49_0e48_4e81_a2e5_cc5aa19e48d3);
}
impl ::windows::core::RuntimeName for PopOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopOutThemeAnimation";
}
impl ::core::convert::From<PopOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: PopOutThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &PopOutThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: PopOutThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &PopOutThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for Timeline {
    fn from(value: PopOutThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for Timeline {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: PopOutThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PopOutThemeAnimation {}
unsafe impl ::core::marker::Sync for PopOutThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PopupThemeTransition(pub ::windows::core::IInspectable);
impl PopupThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopupThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPopupThemeTransitionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopupThemeTransitionStatics<R, F: FnOnce(&IPopupThemeTransitionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopupThemeTransition, IPopupThemeTransitionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PopupThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopupThemeTransition;{47843552-4283-545e-c791-268dca22ce4b})");
}
unsafe impl ::windows::core::Interface for PopupThemeTransition {
    type Vtable = IPopupThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47843552_4283_545e_c791_268dca22ce4b);
}
impl ::windows::core::RuntimeName for PopupThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopupThemeTransition";
}
impl ::core::convert::From<PopupThemeTransition> for ::windows::core::IUnknown {
    fn from(value: PopupThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PopupThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &PopupThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PopupThemeTransition> for ::windows::core::IInspectable {
    fn from(value: PopupThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PopupThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &PopupThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PopupThemeTransition> for Transition {
    fn from(value: PopupThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&PopupThemeTransition> for Transition {
    fn from(value: &PopupThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PopupThemeTransition> for super::super::DependencyObject {
    fn from(value: PopupThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PopupThemeTransition> for super::super::DependencyObject {
    fn from(value: &PopupThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PopupThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PopupThemeTransition {}
unsafe impl ::core::marker::Sync for PopupThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PowerEase(pub ::windows::core::IInspectable);
impl PowerEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Power(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetPower(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PowerProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IPowerEaseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPowerEaseStatics<R, F: FnOnce(&IPowerEaseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerEase, IPowerEaseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PowerEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PowerEase;{69c80579-eedf-405b-8680-d9606880c937})");
}
unsafe impl ::windows::core::Interface for PowerEase {
    type Vtable = IPowerEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c80579_eedf_405b_8680_d9606880c937);
}
impl ::windows::core::RuntimeName for PowerEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PowerEase";
}
impl ::core::convert::From<PowerEase> for ::windows::core::IUnknown {
    fn from(value: PowerEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PowerEase> for ::windows::core::IUnknown {
    fn from(value: &PowerEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PowerEase> for ::windows::core::IInspectable {
    fn from(value: PowerEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PowerEase> for ::windows::core::IInspectable {
    fn from(value: &PowerEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PowerEase> for EasingFunctionBase {
    fn from(value: PowerEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&PowerEase> for EasingFunctionBase {
    fn from(value: &PowerEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<PowerEase> for super::super::DependencyObject {
    fn from(value: PowerEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PowerEase> for super::super::DependencyObject {
    fn from(value: &PowerEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &PowerEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PowerEase {}
unsafe impl ::core::marker::Sync for PowerEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct QuadraticEase(pub ::windows::core::IInspectable);
impl QuadraticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuadraticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for QuadraticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuadraticEase;{e1510e91-ef6d-44f0-803d-68d16de0ddfc})");
}
unsafe impl ::windows::core::Interface for QuadraticEase {
    type Vtable = IQuadraticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1510e91_ef6d_44f0_803d_68d16de0ddfc);
}
impl ::windows::core::RuntimeName for QuadraticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuadraticEase";
}
impl ::core::convert::From<QuadraticEase> for ::windows::core::IUnknown {
    fn from(value: QuadraticEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&QuadraticEase> for ::windows::core::IUnknown {
    fn from(value: &QuadraticEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<QuadraticEase> for ::windows::core::IInspectable {
    fn from(value: QuadraticEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&QuadraticEase> for ::windows::core::IInspectable {
    fn from(value: &QuadraticEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<QuadraticEase> for EasingFunctionBase {
    fn from(value: QuadraticEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&QuadraticEase> for EasingFunctionBase {
    fn from(value: &QuadraticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<QuadraticEase> for super::super::DependencyObject {
    fn from(value: QuadraticEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&QuadraticEase> for super::super::DependencyObject {
    fn from(value: &QuadraticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &QuadraticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for QuadraticEase {}
unsafe impl ::core::marker::Sync for QuadraticEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct QuarticEase(pub ::windows::core::IInspectable);
impl QuarticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuarticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for QuarticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuarticEase;{e8698814-fe42-4a05-b5b8-081f41157815})");
}
unsafe impl ::windows::core::Interface for QuarticEase {
    type Vtable = IQuarticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8698814_fe42_4a05_b5b8_081f41157815);
}
impl ::windows::core::RuntimeName for QuarticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuarticEase";
}
impl ::core::convert::From<QuarticEase> for ::windows::core::IUnknown {
    fn from(value: QuarticEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&QuarticEase> for ::windows::core::IUnknown {
    fn from(value: &QuarticEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<QuarticEase> for ::windows::core::IInspectable {
    fn from(value: QuarticEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&QuarticEase> for ::windows::core::IInspectable {
    fn from(value: &QuarticEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<QuarticEase> for EasingFunctionBase {
    fn from(value: QuarticEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&QuarticEase> for EasingFunctionBase {
    fn from(value: &QuarticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<QuarticEase> for super::super::DependencyObject {
    fn from(value: QuarticEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&QuarticEase> for super::super::DependencyObject {
    fn from(value: &QuarticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &QuarticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for QuarticEase {}
unsafe impl ::core::marker::Sync for QuarticEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct QuinticEase(pub ::windows::core::IInspectable);
impl QuinticEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QuinticEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for QuinticEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuinticEase;{92ee793b-3c49-4108-aa11-ab786603da21})");
}
unsafe impl ::windows::core::Interface for QuinticEase {
    type Vtable = IQuinticEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ee793b_3c49_4108_aa11_ab786603da21);
}
impl ::windows::core::RuntimeName for QuinticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuinticEase";
}
impl ::core::convert::From<QuinticEase> for ::windows::core::IUnknown {
    fn from(value: QuinticEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&QuinticEase> for ::windows::core::IUnknown {
    fn from(value: &QuinticEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<QuinticEase> for ::windows::core::IInspectable {
    fn from(value: QuinticEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&QuinticEase> for ::windows::core::IInspectable {
    fn from(value: &QuinticEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<QuinticEase> for EasingFunctionBase {
    fn from(value: QuinticEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&QuinticEase> for EasingFunctionBase {
    fn from(value: &QuinticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<QuinticEase> for super::super::DependencyObject {
    fn from(value: QuinticEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&QuinticEase> for super::super::DependencyObject {
    fn from(value: &QuinticEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &QuinticEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for QuinticEase {}
unsafe impl ::core::marker::Sync for QuinticEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ReorderThemeTransition(pub ::windows::core::IInspectable);
impl ReorderThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ReorderThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ReorderThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ReorderThemeTransition;{f2065c6c-d052-4ad1-8362-b71b36df7497})");
}
unsafe impl ::windows::core::Interface for ReorderThemeTransition {
    type Vtable = IReorderThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2065c6c_d052_4ad1_8362_b71b36df7497);
}
impl ::windows::core::RuntimeName for ReorderThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ReorderThemeTransition";
}
impl ::core::convert::From<ReorderThemeTransition> for ::windows::core::IUnknown {
    fn from(value: ReorderThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &ReorderThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ReorderThemeTransition> for ::windows::core::IInspectable {
    fn from(value: ReorderThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &ReorderThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ReorderThemeTransition> for Transition {
    fn from(value: ReorderThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for Transition {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ReorderThemeTransition> for super::super::DependencyObject {
    fn from(value: ReorderThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for super::super::DependencyObject {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ReorderThemeTransition {}
unsafe impl ::core::marker::Sync for ReorderThemeTransition {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct RepeatBehavior {
    pub Count: f64,
    pub Duration: super::super::super::super::Foundation::TimeSpan,
    pub Type: RepeatBehaviorType,
}
#[cfg(feature = "Foundation")]
impl RepeatBehavior {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for RepeatBehavior {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for RepeatBehavior {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RepeatBehavior").field("Count", &self.Count).field("Duration", &self.Duration).field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for RepeatBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Duration == other.Duration && self.Type == other.Type
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for RepeatBehavior {}
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RepeatBehaviorHelper(pub ::windows::core::IInspectable);
impl RepeatBehaviorHelper {
    #[cfg(feature = "Foundation")]
    pub fn Forever() -> ::windows::core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RepeatBehavior>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromCount(count: f64) -> ::windows::core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<RepeatBehavior>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(duration: Param0) -> ::windows::core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), duration.into_param().abi(), &mut result__).from_abi::<RepeatBehavior>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetHasCount<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>>(target: Param0) -> ::windows::core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetHasDuration<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>>(target: Param0) -> ::windows::core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Equals<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>, Param1: ::windows::core::IntoParam<'a, RepeatBehavior>>(target: Param0, value: Param1) -> ::windows::core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), target.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IRepeatBehaviorHelperStatics<R, F: FnOnce(&IRepeatBehaviorHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepeatBehaviorHelper, IRepeatBehaviorHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RepeatBehaviorHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepeatBehaviorHelper;{6863ab72-4997-47f9-87ad-37efb75993ea})");
}
unsafe impl ::windows::core::Interface for RepeatBehaviorHelper {
    type Vtable = IRepeatBehaviorHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6863ab72_4997_47f9_87ad_37efb75993ea);
}
impl ::windows::core::RuntimeName for RepeatBehaviorHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepeatBehaviorHelper";
}
impl ::core::convert::From<RepeatBehaviorHelper> for ::windows::core::IUnknown {
    fn from(value: RepeatBehaviorHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RepeatBehaviorHelper> for ::windows::core::IUnknown {
    fn from(value: &RepeatBehaviorHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RepeatBehaviorHelper> for ::windows::core::IInspectable {
    fn from(value: RepeatBehaviorHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RepeatBehaviorHelper> for ::windows::core::IInspectable {
    fn from(value: &RepeatBehaviorHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RepeatBehaviorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RepeatBehaviorHelper {}
unsafe impl ::core::marker::Sync for RepeatBehaviorHelper {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RepeatBehaviorType(pub i32);
impl RepeatBehaviorType {
    pub const Count: RepeatBehaviorType = RepeatBehaviorType(0i32);
    pub const Duration: RepeatBehaviorType = RepeatBehaviorType(1i32);
    pub const Forever: RepeatBehaviorType = RepeatBehaviorType(2i32);
}
impl ::core::convert::From<i32> for RepeatBehaviorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RepeatBehaviorType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RepeatBehaviorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.RepeatBehaviorType;i4)");
}
impl ::windows::core::DefaultType for RepeatBehaviorType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RepositionThemeAnimation(pub ::windows::core::IInspectable);
impl RepositionThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRepositionThemeAnimationStatics<R, F: FnOnce(&IRepositionThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeAnimation, IRepositionThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RepositionThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepositionThemeAnimation;{ecda24e8-8945-4949-a1bf-62109965a7e9})");
}
unsafe impl ::windows::core::Interface for RepositionThemeAnimation {
    type Vtable = IRepositionThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecda24e8_8945_4949_a1bf_62109965a7e9);
}
impl ::windows::core::RuntimeName for RepositionThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepositionThemeAnimation";
}
impl ::core::convert::From<RepositionThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: RepositionThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &RepositionThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: RepositionThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &RepositionThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for Timeline {
    fn from(value: RepositionThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for Timeline {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for super::super::DependencyObject {
    fn from(value: RepositionThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for super::super::DependencyObject {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RepositionThemeAnimation {}
unsafe impl ::core::marker::Sync for RepositionThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RepositionThemeTransition(pub ::windows::core::IInspectable);
impl RepositionThemeTransition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeTransition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRepositionThemeTransition2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRepositionThemeTransition2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsStaggeringEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeTransitionStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRepositionThemeTransitionStatics2<R, F: FnOnce(&IRepositionThemeTransitionStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RepositionThemeTransition, IRepositionThemeTransitionStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RepositionThemeTransition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepositionThemeTransition;{88329b82-98f3-455a-ac53-2e7083b6e22c})");
}
unsafe impl ::windows::core::Interface for RepositionThemeTransition {
    type Vtable = IRepositionThemeTransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88329b82_98f3_455a_ac53_2e7083b6e22c);
}
impl ::windows::core::RuntimeName for RepositionThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepositionThemeTransition";
}
impl ::core::convert::From<RepositionThemeTransition> for ::windows::core::IUnknown {
    fn from(value: RepositionThemeTransition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for ::windows::core::IUnknown {
    fn from(value: &RepositionThemeTransition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RepositionThemeTransition> for ::windows::core::IInspectable {
    fn from(value: RepositionThemeTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for ::windows::core::IInspectable {
    fn from(value: &RepositionThemeTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RepositionThemeTransition> for Transition {
    fn from(value: RepositionThemeTransition) -> Self {
        ::core::convert::Into::<Transition>::into(&value)
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for Transition {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Transition> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, Transition> {
        ::windows::core::Param::Owned(::core::convert::Into::<Transition>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<RepositionThemeTransition> for super::super::DependencyObject {
    fn from(value: RepositionThemeTransition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for super::super::DependencyObject {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RepositionThemeTransition {}
unsafe impl ::core::marker::Sync for RepositionThemeTransition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SineEase(pub ::windows::core::IInspectable);
impl SineEase {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SineEase, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SineEase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SineEase;{a9382962-230b-49da-9e0d-664987892343})");
}
unsafe impl ::windows::core::Interface for SineEase {
    type Vtable = ISineEase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9382962_230b_49da_9e0d_664987892343);
}
impl ::windows::core::RuntimeName for SineEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SineEase";
}
impl ::core::convert::From<SineEase> for ::windows::core::IUnknown {
    fn from(value: SineEase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SineEase> for ::windows::core::IUnknown {
    fn from(value: &SineEase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SineEase> for ::windows::core::IInspectable {
    fn from(value: SineEase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SineEase> for ::windows::core::IInspectable {
    fn from(value: &SineEase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SineEase> for EasingFunctionBase {
    fn from(value: SineEase) -> Self {
        ::core::convert::Into::<EasingFunctionBase>::into(&value)
    }
}
impl ::core::convert::From<&SineEase> for EasingFunctionBase {
    fn from(value: &SineEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, EasingFunctionBase> for &SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, EasingFunctionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SineEase> for super::super::DependencyObject {
    fn from(value: SineEase) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SineEase> for super::super::DependencyObject {
    fn from(value: &SineEase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SineEase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SineEase {}
unsafe impl ::core::marker::Sync for SineEase {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SlideNavigationTransitionEffect(pub i32);
impl SlideNavigationTransitionEffect {
    pub const FromBottom: SlideNavigationTransitionEffect = SlideNavigationTransitionEffect(0i32);
    pub const FromLeft: SlideNavigationTransitionEffect = SlideNavigationTransitionEffect(1i32);
    pub const FromRight: SlideNavigationTransitionEffect = SlideNavigationTransitionEffect(2i32);
}
impl ::core::convert::From<i32> for SlideNavigationTransitionEffect {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SlideNavigationTransitionEffect {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SlideNavigationTransitionEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionEffect;i4)");
}
impl ::windows::core::DefaultType for SlideNavigationTransitionEffect {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SlideNavigationTransitionInfo(pub ::windows::core::IInspectable);
impl SlideNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SlideNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Effect(&self) -> ::windows::core::Result<SlideNavigationTransitionEffect> {
        let this = &::windows::core::Interface::cast::<ISlideNavigationTransitionInfo2>(self)?;
        unsafe {
            let mut result__: SlideNavigationTransitionEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SlideNavigationTransitionEffect>(result__)
        }
    }
    pub fn SetEffect(&self, value: SlideNavigationTransitionEffect) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlideNavigationTransitionInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EffectProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISlideNavigationTransitionInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISlideNavigationTransitionInfoStatics2<R, F: FnOnce(&ISlideNavigationTransitionInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SlideNavigationTransitionInfo, ISlideNavigationTransitionInfoStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SlideNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionInfo;{d6ac9d77-2e03-405f-80ed-e62beef3668f})");
}
unsafe impl ::windows::core::Interface for SlideNavigationTransitionInfo {
    type Vtable = ISlideNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6ac9d77_2e03_405f_80ed_e62beef3668f);
}
impl ::windows::core::RuntimeName for SlideNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionInfo";
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<NavigationTransitionInfo>::into(&value)
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SlideNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for SlideNavigationTransitionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplineColorKeyFrame(pub ::windows::core::IInspectable);
impl SplineColorKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineColorKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeySpline(&self) -> ::windows::core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeySpline>(result__)
        }
    }
    pub fn SetKeySpline<'a, Param0: ::windows::core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn KeySplineProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplineColorKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplineColorKeyFrameStatics<R, F: FnOnce(&ISplineColorKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineColorKeyFrame, ISplineColorKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SplineColorKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplineColorKeyFrame;{1a4a5941-1fe0-473a-8efe-4316d8c86229})");
}
unsafe impl ::windows::core::Interface for SplineColorKeyFrame {
    type Vtable = ISplineColorKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4a5941_1fe0_473a_8efe_4316d8c86229);
}
impl ::windows::core::RuntimeName for SplineColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplineColorKeyFrame";
}
impl ::core::convert::From<SplineColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: SplineColorKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &SplineColorKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: SplineColorKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &SplineColorKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for ColorKeyFrame {
    fn from(value: SplineColorKeyFrame) -> Self {
        ::core::convert::Into::<ColorKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ColorKeyFrame {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ColorKeyFrame> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ColorKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for super::super::DependencyObject {
    fn from(value: SplineColorKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SplineColorKeyFrame {}
unsafe impl ::core::marker::Sync for SplineColorKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplineDoubleKeyFrame(pub ::windows::core::IInspectable);
impl SplineDoubleKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineDoubleKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeySpline(&self) -> ::windows::core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeySpline>(result__)
        }
    }
    pub fn SetKeySpline<'a, Param0: ::windows::core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn KeySplineProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplineDoubleKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplineDoubleKeyFrameStatics<R, F: FnOnce(&ISplineDoubleKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplineDoubleKeyFrame, ISplineDoubleKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SplineDoubleKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplineDoubleKeyFrame;{00d72d38-6b2b-4843-838e-c8b115eec801})");
}
unsafe impl ::windows::core::Interface for SplineDoubleKeyFrame {
    type Vtable = ISplineDoubleKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00d72d38_6b2b_4843_838e_c8b115eec801);
}
impl ::windows::core::RuntimeName for SplineDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplineDoubleKeyFrame";
}
impl ::core::convert::From<SplineDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        ::core::convert::Into::<DoubleKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, DoubleKeyFrame> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, DoubleKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SplineDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for SplineDoubleKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplinePointKeyFrame(pub ::windows::core::IInspectable);
impl SplinePointKeyFrame {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplinePointKeyFrame, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeySpline(&self) -> ::windows::core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeySpline>(result__)
        }
    }
    pub fn SetKeySpline<'a, Param0: ::windows::core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn KeySplineProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplinePointKeyFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplinePointKeyFrameStatics<R, F: FnOnce(&ISplinePointKeyFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplinePointKeyFrame, ISplinePointKeyFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SplinePointKeyFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplinePointKeyFrame;{0f19f306-7036-494f-bc3c-780df0cc524a})");
}
unsafe impl ::windows::core::Interface for SplinePointKeyFrame {
    type Vtable = ISplinePointKeyFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f19f306_7036_494f_bc3c_780df0cc524a);
}
impl ::windows::core::RuntimeName for SplinePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplinePointKeyFrame";
}
impl ::core::convert::From<SplinePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: SplinePointKeyFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for ::windows::core::IUnknown {
    fn from(value: &SplinePointKeyFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: SplinePointKeyFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for ::windows::core::IInspectable {
    fn from(value: &SplinePointKeyFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for PointKeyFrame {
    fn from(value: SplinePointKeyFrame) -> Self {
        ::core::convert::Into::<PointKeyFrame>::into(&value)
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for PointKeyFrame {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PointKeyFrame> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, PointKeyFrame> {
        ::windows::core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for super::super::DependencyObject {
    fn from(value: SplinePointKeyFrame) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SplinePointKeyFrame {}
unsafe impl ::core::marker::Sync for SplinePointKeyFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplitCloseThemeAnimation(pub ::windows::core::IInspectable);
impl SplitCloseThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitCloseThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetOpenedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetOpenedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetClosedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetClosedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContentTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetContentTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ClosedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OffsetFromCenter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::AnimationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OpenedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetFromCenterProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationDirectionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplitCloseThemeAnimationStatics<R, F: FnOnce(&ISplitCloseThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitCloseThemeAnimation, ISplitCloseThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SplitCloseThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplitCloseThemeAnimation;{4f799518-ff39-4e90-bb74-2abd56027402})");
}
unsafe impl ::windows::core::Interface for SplitCloseThemeAnimation {
    type Vtable = ISplitCloseThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f799518_ff39_4e90_bb74_2abd56027402);
}
impl ::windows::core::RuntimeName for SplitCloseThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplitCloseThemeAnimation";
}
impl ::core::convert::From<SplitCloseThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for Timeline {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for Timeline {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for super::super::DependencyObject {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SplitCloseThemeAnimation {}
unsafe impl ::core::marker::Sync for SplitCloseThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplitOpenThemeAnimation(pub ::windows::core::IInspectable);
impl SplitOpenThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitOpenThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetOpenedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetOpenedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetClosedTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetClosedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContentTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetContentTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ClosedLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OffsetFromCenter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__: super::super::Controls::Primitives::AnimationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OpenedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedLengthProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetFromCenterProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationDirectionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplitOpenThemeAnimationStatics<R, F: FnOnce(&ISplitOpenThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SplitOpenThemeAnimation, ISplitOpenThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SplitOpenThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplitOpenThemeAnimation;{785fd7aa-5456-4639-8fd2-26bae6a5ffe4})");
}
unsafe impl ::windows::core::Interface for SplitOpenThemeAnimation {
    type Vtable = ISplitOpenThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x785fd7aa_5456_4639_8fd2_26bae6a5ffe4);
}
impl ::windows::core::RuntimeName for SplitOpenThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplitOpenThemeAnimation";
}
impl ::core::convert::From<SplitOpenThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for Timeline {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for Timeline {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for super::super::DependencyObject {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SplitOpenThemeAnimation {}
unsafe impl ::core::marker::Sync for SplitOpenThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Storyboard(pub ::windows::core::IInspectable);
impl Storyboard {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Storyboard, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<TimelineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimelineCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Seek<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, offset: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), offset.into_param().abi()).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Begin(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn GetCurrentState(&self) -> ::windows::core::Result<ClockState> {
        let this = self;
        unsafe {
            let mut result__: ClockState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClockState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SeekAlignedToLastTick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, offset: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), offset.into_param().abi()).ok() }
    }
    pub fn SkipToFill(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn TargetPropertyProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetTargetProperty<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SetTargetProperty<'a, Param0: ::windows::core::IntoParam<'a, Timeline>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, path: Param1) -> ::windows::core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), path.into_param().abi()).ok() })
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetTargetName<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, Timeline>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, name: Param1) -> ::windows::core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), name.into_param().abi()).ok() })
    }
    pub fn SetTarget<'a, Param0: ::windows::core::IntoParam<'a, Timeline>, Param1: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(timeline: Param0, target: Param1) -> ::windows::core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), timeline.into_param().abi(), target.into_param().abi()).ok() })
    }
    pub fn IStoryboardStatics<R, F: FnOnce(&IStoryboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Storyboard, IStoryboardStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Storyboard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Storyboard;{d45c1e6e-3594-460e-981a-32271bd3aa06})");
}
unsafe impl ::windows::core::Interface for Storyboard {
    type Vtable = IStoryboard_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45c1e6e_3594_460e_981a_32271bd3aa06);
}
impl ::windows::core::RuntimeName for Storyboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Storyboard";
}
impl ::core::convert::From<Storyboard> for ::windows::core::IUnknown {
    fn from(value: Storyboard) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Storyboard> for ::windows::core::IUnknown {
    fn from(value: &Storyboard) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Storyboard> for ::windows::core::IInspectable {
    fn from(value: Storyboard) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Storyboard> for ::windows::core::IInspectable {
    fn from(value: &Storyboard) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Storyboard> for Timeline {
    fn from(value: Storyboard) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&Storyboard> for Timeline {
    fn from(value: &Storyboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<Storyboard> for super::super::DependencyObject {
    fn from(value: Storyboard) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Storyboard> for super::super::DependencyObject {
    fn from(value: &Storyboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Storyboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Storyboard {}
unsafe impl ::core::marker::Sync for Storyboard {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SuppressNavigationTransitionInfo(pub ::windows::core::IInspectable);
impl SuppressNavigationTransitionInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SuppressNavigationTransitionInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SuppressNavigationTransitionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SuppressNavigationTransitionInfo;{244d7b0c-b1b7-4871-9d3e-d56203a3a5b4})");
}
unsafe impl ::windows::core::Interface for SuppressNavigationTransitionInfo {
    type Vtable = ISuppressNavigationTransitionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x244d7b0c_b1b7_4871_9d3e_d56203a3a5b4);
}
impl ::windows::core::RuntimeName for SuppressNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SuppressNavigationTransitionInfo";
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for ::windows::core::IUnknown {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for ::windows::core::IInspectable {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<NavigationTransitionInfo>::into(&value)
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, NavigationTransitionInfo> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, NavigationTransitionInfo> {
        ::windows::core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SuppressNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for SuppressNavigationTransitionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SwipeBackThemeAnimation(pub ::windows::core::IInspectable);
impl SwipeBackThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeBackThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISwipeBackThemeAnimationStatics<R, F: FnOnce(&ISwipeBackThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeBackThemeAnimation, ISwipeBackThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SwipeBackThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SwipeBackThemeAnimation;{a38a4214-0bca-4d2d-95f7-ceba57fbaf60})");
}
unsafe impl ::windows::core::Interface for SwipeBackThemeAnimation {
    type Vtable = ISwipeBackThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa38a4214_0bca_4d2d_95f7_ceba57fbaf60);
}
impl ::windows::core::RuntimeName for SwipeBackThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SwipeBackThemeAnimation";
}
impl ::core::convert::From<SwipeBackThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for Timeline {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for Timeline {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for super::super::DependencyObject {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SwipeBackThemeAnimation {}
unsafe impl ::core::marker::Sync for SwipeBackThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SwipeHintThemeAnimation(pub ::windows::core::IInspectable);
impl SwipeHintThemeAnimation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeHintThemeAnimation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ToHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetToHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ToVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetToVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToHorizontalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToVerticalOffsetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISwipeHintThemeAnimationStatics<R, F: FnOnce(&ISwipeHintThemeAnimationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SwipeHintThemeAnimation, ISwipeHintThemeAnimationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SwipeHintThemeAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SwipeHintThemeAnimation;{cdd067c0-580e-4e40-be98-f202d3d84365})");
}
unsafe impl ::windows::core::Interface for SwipeHintThemeAnimation {
    type Vtable = ISwipeHintThemeAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdd067c0_580e_4e40_be98_f202d3d84365);
}
impl ::windows::core::RuntimeName for SwipeHintThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SwipeHintThemeAnimation";
}
impl ::core::convert::From<SwipeHintThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for ::windows::core::IUnknown {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for ::windows::core::IInspectable {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for Timeline {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        ::core::convert::Into::<Timeline>::into(&value)
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for Timeline {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, Timeline> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, Timeline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Timeline>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for super::super::DependencyObject {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SwipeHintThemeAnimation {}
unsafe impl ::core::marker::Sync for SwipeHintThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Timeline(pub ::windows::core::IInspectable);
impl Timeline {
    pub fn AutoReverse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoReverse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetBeginTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Duration> {
        let this = self;
        unsafe {
            let mut result__: super::super::Duration = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Duration>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Duration>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SpeedRatio(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeedRatio(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FillBehavior(&self) -> ::windows::core::Result<FillBehavior> {
        let this = self;
        unsafe {
            let mut result__: FillBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FillBehavior>(result__)
        }
    }
    pub fn SetFillBehavior(&self, value: FillBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RepeatBehavior(&self) -> ::windows::core::Result<RepeatBehavior> {
        let this = self;
        unsafe {
            let mut result__: RepeatBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RepeatBehavior>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetRepeatBehavior<'a, Param0: ::windows::core::IntoParam<'a, RepeatBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AllowDependentAnimations() -> ::windows::core::Result<bool> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetAllowDependentAnimations(value: bool) -> ::windows::core::Result<()> {
        Self::ITimelineStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn AutoReverseProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn BeginTimeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DurationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SpeedRatioProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FillBehaviorProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RepeatBehaviorProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ITimelineStatics<R, F: FnOnce(&ITimelineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Timeline, ITimelineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Timeline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Timeline;{0bc465dc-be4d-4d0d-9549-2208b715f40d})");
}
unsafe impl ::windows::core::Interface for Timeline {
    type Vtable = ITimeline_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc465dc_be4d_4d0d_9549_2208b715f40d);
}
impl ::windows::core::RuntimeName for Timeline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Timeline";
}
impl ::core::convert::From<Timeline> for ::windows::core::IUnknown {
    fn from(value: Timeline) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Timeline> for ::windows::core::IUnknown {
    fn from(value: &Timeline) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Timeline> for ::windows::core::IInspectable {
    fn from(value: Timeline) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Timeline> for ::windows::core::IInspectable {
    fn from(value: &Timeline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Timeline> for super::super::DependencyObject {
    fn from(value: Timeline) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Timeline> for super::super::DependencyObject {
    fn from(value: &Timeline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Timeline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Timeline {}
unsafe impl ::core::marker::Sync for Timeline {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimelineCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl TimelineCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimelineCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Timeline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Timeline>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<Timeline>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<Timeline>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Timeline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Timeline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Timeline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Timeline as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Timeline as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<Timeline>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<Timeline>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<Timeline>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for TimelineCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.TimelineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.Timeline;{0bc465dc-be4d-4d0d-9549-2208b715f40d})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for TimelineCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVector_abi<Timeline>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::super::Foundation::Collections::IVector<Timeline> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for TimelineCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.TimelineCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineCollection> for ::windows::core::IUnknown {
    fn from(value: TimelineCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineCollection> for ::windows::core::IUnknown {
    fn from(value: &TimelineCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineCollection> for ::windows::core::IInspectable {
    fn from(value: TimelineCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineCollection> for ::windows::core::IInspectable {
    fn from(value: &TimelineCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TimelineCollection> for super::super::super::super::Foundation::Collections::IVector<Timeline> {
    fn from(value: TimelineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TimelineCollection> for super::super::super::super::Foundation::Collections::IVector<Timeline> {
    fn from(value: &TimelineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> for TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> for &TimelineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Timeline>> {
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
unsafe impl ::core::marker::Send for TimelineCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for TimelineCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for TimelineCollection {
    type Item = Timeline;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &TimelineCollection {
    type Item = Timeline;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Transition(pub ::windows::core::IInspectable);
impl Transition {}
unsafe impl ::windows::core::RuntimeType for Transition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Transition;{3c677c7c-01d0-4dce-b333-976f93312b08})");
}
unsafe impl ::windows::core::Interface for Transition {
    type Vtable = ITransition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c677c7c_01d0_4dce_b333_976f93312b08);
}
impl ::windows::core::RuntimeName for Transition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Transition";
}
impl ::core::convert::From<Transition> for ::windows::core::IUnknown {
    fn from(value: Transition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Transition> for ::windows::core::IUnknown {
    fn from(value: &Transition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Transition> for ::windows::core::IInspectable {
    fn from(value: Transition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Transition> for ::windows::core::IInspectable {
    fn from(value: &Transition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Transition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Transition> for super::super::DependencyObject {
    fn from(value: Transition) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Transition> for super::super::DependencyObject {
    fn from(value: &Transition) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for Transition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &Transition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for Transition {}
unsafe impl ::core::marker::Sync for Transition {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TransitionCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl TransitionCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransitionCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Transition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Transition>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<Transition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<Transition>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Transition>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Transition>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Transition>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Transition>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Transition as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Transition as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterator<Transition>> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::Collections::IIterable<Transition>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IIterator<Transition>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for TransitionCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.TransitionCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.Transition;{3c677c7c-01d0-4dce-b333-976f93312b08})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for TransitionCollection {
    type Vtable = super::super::super::super::Foundation::Collections::IVector_abi<Transition>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::super::Foundation::Collections::IVector<Transition> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for TransitionCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.TransitionCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransitionCollection> for ::windows::core::IUnknown {
    fn from(value: TransitionCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransitionCollection> for ::windows::core::IUnknown {
    fn from(value: &TransitionCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransitionCollection> for ::windows::core::IInspectable {
    fn from(value: TransitionCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransitionCollection> for ::windows::core::IInspectable {
    fn from(value: &TransitionCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<TransitionCollection> for super::super::super::super::Foundation::Collections::IVector<Transition> {
    fn from(value: TransitionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&TransitionCollection> for super::super::super::super::Foundation::Collections::IVector<Transition> {
    fn from(value: &TransitionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> for TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> for &TransitionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::super::Foundation::Collections::IVector<Transition>> {
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
unsafe impl ::core::marker::Send for TransitionCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for TransitionCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for TransitionCollection {
    type Item = Transition;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &TransitionCollection {
    type Item = Transition;
    type IntoIter = super::super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
