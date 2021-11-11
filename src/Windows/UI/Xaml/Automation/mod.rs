#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub mod Peers;
#[cfg(feature = "UI_Xaml_Automation_Provider")]
pub mod Provider;
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub mod Text;
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AnnotationPatternIdentifiers(pub ::windows::core::IInspectable);
impl AnnotationPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationTypeIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationTypeNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AuthorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DateTimeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn TargetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAnnotationPatternIdentifiersStatics<R, F: FnOnce(&IAnnotationPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AnnotationPatternIdentifiers, IAnnotationPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AnnotationPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers;{d475a0c1-48b2-4e40-a6cf-3dc4b638c0de})");
}
unsafe impl ::windows::core::Interface for AnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd475a0c1_48b2_4e40_a6cf_3dc4b638c0de);
}
impl ::windows::core::RuntimeName for AnnotationPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers";
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AnnotationPatternIdentifiers {}
unsafe impl ::core::marker::Sync for AnnotationPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: AnnotationType = AnnotationType(60000i32);
    pub const SpellingError: AnnotationType = AnnotationType(60001i32);
    pub const GrammarError: AnnotationType = AnnotationType(60002i32);
    pub const Comment: AnnotationType = AnnotationType(60003i32);
    pub const FormulaError: AnnotationType = AnnotationType(60004i32);
    pub const TrackChanges: AnnotationType = AnnotationType(60005i32);
    pub const Header: AnnotationType = AnnotationType(60006i32);
    pub const Footer: AnnotationType = AnnotationType(60007i32);
    pub const Highlighted: AnnotationType = AnnotationType(60008i32);
    pub const Endnote: AnnotationType = AnnotationType(60009i32);
    pub const Footnote: AnnotationType = AnnotationType(60010i32);
    pub const InsertionChange: AnnotationType = AnnotationType(60011i32);
    pub const DeletionChange: AnnotationType = AnnotationType(60012i32);
    pub const MoveChange: AnnotationType = AnnotationType(60013i32);
    pub const FormatChange: AnnotationType = AnnotationType(60014i32);
    pub const UnsyncedChange: AnnotationType = AnnotationType(60015i32);
    pub const EditingLockedChange: AnnotationType = AnnotationType(60016i32);
    pub const ExternalChange: AnnotationType = AnnotationType(60017i32);
    pub const ConflictingChange: AnnotationType = AnnotationType(60018i32);
    pub const Author: AnnotationType = AnnotationType(60019i32);
    pub const AdvancedProofingIssue: AnnotationType = AnnotationType(60020i32);
    pub const DataValidationError: AnnotationType = AnnotationType(60021i32);
    pub const CircularReferenceError: AnnotationType = AnnotationType(60022i32);
}
impl ::core::convert::From<i32> for AnnotationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AnnotationType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AnnotationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AnnotationType;i4)");
}
impl ::windows::core::DefaultType for AnnotationType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: AutomationActiveEnd = AutomationActiveEnd(0i32);
    pub const Start: AutomationActiveEnd = AutomationActiveEnd(1i32);
    pub const End: AutomationActiveEnd = AutomationActiveEnd(2i32);
}
impl ::core::convert::From<i32> for AutomationActiveEnd {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationActiveEnd {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationActiveEnd {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationActiveEnd;i4)");
}
impl ::windows::core::DefaultType for AutomationActiveEnd {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: AutomationAnimationStyle = AutomationAnimationStyle(0i32);
    pub const LasVegasLights: AutomationAnimationStyle = AutomationAnimationStyle(1i32);
    pub const BlinkingBackground: AutomationAnimationStyle = AutomationAnimationStyle(2i32);
    pub const SparkleText: AutomationAnimationStyle = AutomationAnimationStyle(3i32);
    pub const MarchingBlackAnts: AutomationAnimationStyle = AutomationAnimationStyle(4i32);
    pub const MarchingRedAnts: AutomationAnimationStyle = AutomationAnimationStyle(5i32);
    pub const Shimmer: AutomationAnimationStyle = AutomationAnimationStyle(6i32);
    pub const Other: AutomationAnimationStyle = AutomationAnimationStyle(7i32);
}
impl ::core::convert::From<i32> for AutomationAnimationStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationAnimationStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationAnimationStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationAnimationStyle;i4)");
}
impl ::windows::core::DefaultType for AutomationAnimationStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AutomationAnnotation(pub ::windows::core::IInspectable);
impl AutomationAnnotation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationAnnotation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn Type(&self) -> ::windows::core::Result<AnnotationType> {
        let this = self;
        unsafe {
            let mut result__: AnnotationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AnnotationType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetType(&self, value: AnnotationType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn Element(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetElement<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CreateInstance(r#type: AnnotationType) -> ::windows::core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CreateWithElementParameter<'a, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(r#type: AnnotationType, element: Param1) -> ::windows::core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), r#type, element.into_param().abi(), &mut result__).from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn TypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ElementProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IAutomationAnnotationFactory<R, F: FnOnce(&IAutomationAnnotationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationAnnotation, IAutomationAnnotationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationAnnotationStatics<R, F: FnOnce(&IAutomationAnnotationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationAnnotation, IAutomationAnnotationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationAnnotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationAnnotation;{fb3c30ca-03d8-4618-91bf-e4d84f4af318})");
}
unsafe impl ::windows::core::Interface for AutomationAnnotation {
    type Vtable = IAutomationAnnotation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb3c30ca_03d8_4618_91bf_e4d84f4af318);
}
impl ::windows::core::RuntimeName for AutomationAnnotation {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationAnnotation";
}
impl ::core::convert::From<AutomationAnnotation> for ::windows::core::IUnknown {
    fn from(value: AutomationAnnotation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows::core::IUnknown {
    fn from(value: &AutomationAnnotation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationAnnotation> for ::windows::core::IInspectable {
    fn from(value: AutomationAnnotation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows::core::IInspectable {
    fn from(value: &AutomationAnnotation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AutomationAnnotation> for super::DependencyObject {
    fn from(value: AutomationAnnotation) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&AutomationAnnotation> for super::DependencyObject {
    fn from(value: &AutomationAnnotation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for AutomationAnnotation {}
unsafe impl ::core::marker::Sync for AutomationAnnotation {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: AutomationBulletStyle = AutomationBulletStyle(0i32);
    pub const HollowRoundBullet: AutomationBulletStyle = AutomationBulletStyle(1i32);
    pub const FilledRoundBullet: AutomationBulletStyle = AutomationBulletStyle(2i32);
    pub const HollowSquareBullet: AutomationBulletStyle = AutomationBulletStyle(3i32);
    pub const FilledSquareBullet: AutomationBulletStyle = AutomationBulletStyle(4i32);
    pub const DashBullet: AutomationBulletStyle = AutomationBulletStyle(5i32);
    pub const Other: AutomationBulletStyle = AutomationBulletStyle(6i32);
}
impl ::core::convert::From<i32> for AutomationBulletStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationBulletStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationBulletStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationBulletStyle;i4)");
}
impl ::windows::core::DefaultType for AutomationBulletStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: AutomationCaretBidiMode = AutomationCaretBidiMode(0i32);
    pub const RTL: AutomationCaretBidiMode = AutomationCaretBidiMode(1i32);
}
impl ::core::convert::From<i32> for AutomationCaretBidiMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationCaretBidiMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationCaretBidiMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationCaretBidiMode;i4)");
}
impl ::windows::core::DefaultType for AutomationCaretBidiMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: AutomationCaretPosition = AutomationCaretPosition(0i32);
    pub const EndOfLine: AutomationCaretPosition = AutomationCaretPosition(1i32);
    pub const BeginningOfLine: AutomationCaretPosition = AutomationCaretPosition(2i32);
}
impl ::core::convert::From<i32> for AutomationCaretPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationCaretPosition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationCaretPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationCaretPosition;i4)");
}
impl ::windows::core::DefaultType for AutomationCaretPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AutomationElementIdentifiers(pub ::windows::core::IInspectable);
impl AutomationElementIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AcceleratorKeyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AutomationIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn BoundingRectangleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ClassNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ClickablePointProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ControlTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HasKeyboardFocusProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HelpTextProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsContentElementProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsControlElementProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsEnabledProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsKeyboardFocusableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsOffscreenProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsPasswordProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsRequiredForFormProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemStatusProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LabeledByProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedControlTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn NameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn OrientationProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LiveSettingProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ControlledPeersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn PositionInSetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SizeOfSetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LandmarkTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsPeripheralProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDataValidForFormProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FullDescriptionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DescribedByProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsToProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsFromProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CultureProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics6(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HeadingLevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics7(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDialogProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAutomationElementIdentifiersStatics<R, F: FnOnce(&IAutomationElementIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics2<R, F: FnOnce(&IAutomationElementIdentifiersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics3<R, F: FnOnce(&IAutomationElementIdentifiersStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics4<R, F: FnOnce(&IAutomationElementIdentifiersStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics5<R, F: FnOnce(&IAutomationElementIdentifiersStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics6<R, F: FnOnce(&IAutomationElementIdentifiersStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics7<R, F: FnOnce(&IAutomationElementIdentifiersStatics7) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics7> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics8<R, F: FnOnce(&IAutomationElementIdentifiersStatics8) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics8> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationElementIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationElementIdentifiers;{e68a63cf-4345-4e2d-8a6a-49cce1fa2dcc})");
}
unsafe impl ::windows::core::Interface for AutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe68a63cf_4345_4e2d_8a6a_49cce1fa2dcc);
}
impl ::windows::core::RuntimeName for AutomationElementIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationElementIdentifiers";
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows::core::IUnknown {
    fn from(value: AutomationElementIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows::core::IInspectable {
    fn from(value: AutomationElementIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutomationElementIdentifiers {}
unsafe impl ::core::marker::Sync for AutomationElementIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: AutomationFlowDirections = AutomationFlowDirections(0i32);
    pub const RightToLeft: AutomationFlowDirections = AutomationFlowDirections(1i32);
    pub const BottomToTop: AutomationFlowDirections = AutomationFlowDirections(2i32);
    pub const Vertical: AutomationFlowDirections = AutomationFlowDirections(3i32);
}
impl ::core::convert::From<i32> for AutomationFlowDirections {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationFlowDirections {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationFlowDirections {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationFlowDirections;i4)");
}
impl ::windows::core::DefaultType for AutomationFlowDirections {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: AutomationOutlineStyles = AutomationOutlineStyles(0i32);
    pub const Outline: AutomationOutlineStyles = AutomationOutlineStyles(1i32);
    pub const Shadow: AutomationOutlineStyles = AutomationOutlineStyles(2i32);
    pub const Engraved: AutomationOutlineStyles = AutomationOutlineStyles(3i32);
    pub const Embossed: AutomationOutlineStyles = AutomationOutlineStyles(4i32);
}
impl ::core::convert::From<i32> for AutomationOutlineStyles {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationOutlineStyles {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationOutlineStyles {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationOutlineStyles;i4)");
}
impl ::windows::core::DefaultType for AutomationOutlineStyles {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AutomationProperties(pub ::windows::core::IInspectable);
impl AutomationProperties {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AcceleratorKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAcceleratorKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetAcceleratorKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AutomationIdProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAutomationId<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetAutomationId<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HelpTextProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetHelpText<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetHelpText<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsRequiredForFormProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsRequiredForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsRequiredForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemStatusProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetItemStatus<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetItemStatus<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetItemType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetItemType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LabeledByProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLabeledBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::UIElement> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLabeledBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn NameProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetName<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LiveSettingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetLiveSetting<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AutomationLiveSetting> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationLiveSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationLiveSetting>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetLiveSetting<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationLiveSetting) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AccessibilityViewProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetAccessibilityView<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AccessibilityView> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: Peers::AccessibilityView = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AccessibilityView>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetAccessibilityView<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AccessibilityView) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ControlledPeersProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `Foundation_Collections`*"]
    pub fn GetControlledPeers<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::UIElement>> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::UIElement>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn PositionInSetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetPositionInSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetPositionInSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SizeOfSetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetSizeOfSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetSizeOfSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LevelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `Foundation_Collections`*"]
    pub fn GetAnnotations<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LandmarkTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AutomationLandmarkType> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: Peers::AutomationLandmarkType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationLandmarkType>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationLandmarkType) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLocalizedLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLocalizedLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsPeripheralProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsPeripheral<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsPeripheral<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDataValidForFormProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsDataValidForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsDataValidForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FullDescriptionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetFullDescription<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetFullDescription<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedControlTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLocalizedControlType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLocalizedControlType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DescribedByProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `Foundation_Collections`*"]
    pub fn GetDescribedBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsToProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `Foundation_Collections`*"]
    pub fn GetFlowsTo<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsFromProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `Foundation_Collections`*"]
    pub fn GetFlowsFrom<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CultureProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics6(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetCulture<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics6(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetCulture<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HeadingLevelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics7(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetHeadingLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AutomationHeadingLevel> {
        Self::IAutomationPropertiesStatics7(|this| unsafe {
            let mut result__: Peers::AutomationHeadingLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationHeadingLevel>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetHeadingLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationHeadingLevel) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics7(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDialogProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsDialog<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics8(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsDialog<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics8(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AutomationControlTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics9(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetAutomationControlType<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<Peers::AutomationControlType> {
        Self::IAutomationPropertiesStatics9(|this| unsafe {
            let mut result__: Peers::AutomationControlType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationControlType>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetAutomationControlType<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: Peers::AutomationControlType) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics9(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    pub fn IAutomationPropertiesStatics<R, F: FnOnce(&IAutomationPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics2<R, F: FnOnce(&IAutomationPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics3<R, F: FnOnce(&IAutomationPropertiesStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics4<R, F: FnOnce(&IAutomationPropertiesStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics5<R, F: FnOnce(&IAutomationPropertiesStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics6<R, F: FnOnce(&IAutomationPropertiesStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics7<R, F: FnOnce(&IAutomationPropertiesStatics7) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics7> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics8<R, F: FnOnce(&IAutomationPropertiesStatics8) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics8> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics9<R, F: FnOnce(&IAutomationPropertiesStatics9) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics9> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationProperties;{68d7232c-e622-48e9-af0b-1ffa33cc5cba})");
}
unsafe impl ::windows::core::Interface for AutomationProperties {
    type Vtable = IAutomationProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68d7232c_e622_48e9_af0b_1ffa33cc5cba);
}
impl ::windows::core::RuntimeName for AutomationProperties {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationProperties";
}
impl ::core::convert::From<AutomationProperties> for ::windows::core::IUnknown {
    fn from(value: AutomationProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows::core::IUnknown {
    fn from(value: &AutomationProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationProperties> for ::windows::core::IInspectable {
    fn from(value: AutomationProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows::core::IInspectable {
    fn from(value: &AutomationProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutomationProperties {}
unsafe impl ::core::marker::Sync for AutomationProperties {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AutomationProperty(pub ::windows::core::IInspectable);
impl AutomationProperty {}
unsafe impl ::windows::core::RuntimeType for AutomationProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationProperty;{b627195b-3227-4e16-9534-ddece30ddb46})");
}
unsafe impl ::windows::core::Interface for AutomationProperty {
    type Vtable = IAutomationProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb627195b_3227_4e16_9534_ddece30ddb46);
}
impl ::windows::core::RuntimeName for AutomationProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationProperty";
}
impl ::core::convert::From<AutomationProperty> for ::windows::core::IUnknown {
    fn from(value: AutomationProperty) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows::core::IUnknown {
    fn from(value: &AutomationProperty) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationProperty> for ::windows::core::IInspectable {
    fn from(value: AutomationProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows::core::IInspectable {
    fn from(value: &AutomationProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutomationProperty {}
unsafe impl ::core::marker::Sync for AutomationProperty {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: AutomationStyleId = AutomationStyleId(70001i32);
    pub const Heading2: AutomationStyleId = AutomationStyleId(70002i32);
    pub const Heading3: AutomationStyleId = AutomationStyleId(70003i32);
    pub const Heading4: AutomationStyleId = AutomationStyleId(70004i32);
    pub const Heading5: AutomationStyleId = AutomationStyleId(70005i32);
    pub const Heading6: AutomationStyleId = AutomationStyleId(70006i32);
    pub const Heading7: AutomationStyleId = AutomationStyleId(70007i32);
    pub const Heading8: AutomationStyleId = AutomationStyleId(70008i32);
    pub const Heading9: AutomationStyleId = AutomationStyleId(70009i32);
    pub const Title: AutomationStyleId = AutomationStyleId(70010i32);
    pub const Subtitle: AutomationStyleId = AutomationStyleId(70011i32);
    pub const Normal: AutomationStyleId = AutomationStyleId(70012i32);
    pub const Emphasis: AutomationStyleId = AutomationStyleId(70013i32);
    pub const Quote: AutomationStyleId = AutomationStyleId(70014i32);
    pub const BulletedList: AutomationStyleId = AutomationStyleId(70015i32);
}
impl ::core::convert::From<i32> for AutomationStyleId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationStyleId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationStyleId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationStyleId;i4)");
}
impl ::windows::core::DefaultType for AutomationStyleId {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(0i32);
    pub const Single: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(1i32);
    pub const WordsOnly: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(2i32);
    pub const Double: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(3i32);
    pub const Dot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(4i32);
    pub const Dash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(5i32);
    pub const DashDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(6i32);
    pub const DashDotDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(7i32);
    pub const Wavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(8i32);
    pub const ThickSingle: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(9i32);
    pub const DoubleWavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(10i32);
    pub const ThickWavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(11i32);
    pub const LongDash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(12i32);
    pub const ThickDash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(13i32);
    pub const ThickDashDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(14i32);
    pub const ThickDashDotDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(15i32);
    pub const ThickDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(16i32);
    pub const ThickLongDash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(17i32);
    pub const Other: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(18i32);
}
impl ::core::convert::From<i32> for AutomationTextDecorationLineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationTextDecorationLineStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationTextDecorationLineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationTextDecorationLineStyle;i4)");
}
impl ::windows::core::DefaultType for AutomationTextDecorationLineStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: AutomationTextEditChangeType = AutomationTextEditChangeType(0i32);
    pub const AutoCorrect: AutomationTextEditChangeType = AutomationTextEditChangeType(1i32);
    pub const Composition: AutomationTextEditChangeType = AutomationTextEditChangeType(2i32);
    pub const CompositionFinalized: AutomationTextEditChangeType = AutomationTextEditChangeType(3i32);
}
impl ::core::convert::From<i32> for AutomationTextEditChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationTextEditChangeType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationTextEditChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationTextEditChangeType;i4)");
}
impl ::windows::core::DefaultType for AutomationTextEditChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DockPatternIdentifiers(pub ::windows::core::IInspectable);
impl DockPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DockPositionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDockPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDockPatternIdentifiersStatics<R, F: FnOnce(&IDockPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DockPatternIdentifiers, IDockPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DockPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DockPatternIdentifiers;{ccd7f4e6-e4f9-47ff-bde7-378b11f78e09})");
}
unsafe impl ::windows::core::Interface for DockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccd7f4e6_e4f9_47ff_bde7_378b11f78e09);
}
impl ::windows::core::RuntimeName for DockPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DockPatternIdentifiers";
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DockPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DockPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DockPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DockPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DockPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DockPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: DockPosition = DockPosition(0i32);
    pub const Left: DockPosition = DockPosition(1i32);
    pub const Bottom: DockPosition = DockPosition(2i32);
    pub const Right: DockPosition = DockPosition(3i32);
    pub const Fill: DockPosition = DockPosition(4i32);
    pub const None: DockPosition = DockPosition(5i32);
}
impl ::core::convert::From<i32> for DockPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DockPosition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DockPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.DockPosition;i4)");
}
impl ::windows::core::DefaultType for DockPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DragPatternIdentifiers(pub ::windows::core::IInspectable);
impl DragPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropEffectProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropEffectsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GrabbedItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsGrabbedProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDragPatternIdentifiersStatics<R, F: FnOnce(&IDragPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragPatternIdentifiers, IDragPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DragPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DragPatternIdentifiers;{6266e985-4d07-4e80-82eb-8f96690a1a0c})");
}
unsafe impl ::windows::core::Interface for DragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6266e985_4d07_4e80_82eb_8f96690a1a0c);
}
impl ::windows::core::RuntimeName for DragPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DragPatternIdentifiers";
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DragPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DragPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DragPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DragPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DragPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DragPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DropTargetPatternIdentifiers(pub ::windows::core::IInspectable);
impl DropTargetPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropTargetEffectProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropTargetEffectsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDropTargetPatternIdentifiersStatics<R, F: FnOnce(&IDropTargetPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DropTargetPatternIdentifiers, IDropTargetPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DropTargetPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers;{11865133-a6fe-4634-bd18-0ef612b7b208})");
}
unsafe impl ::windows::core::Interface for DropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11865133_a6fe_4634_bd18_0ef612b7b208);
}
impl ::windows::core::RuntimeName for DropTargetPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers";
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DropTargetPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DropTargetPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExpandCollapsePatternIdentifiers(pub ::windows::core::IInspectable);
impl ExpandCollapsePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ExpandCollapseStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IExpandCollapsePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IExpandCollapsePatternIdentifiersStatics<R, F: FnOnce(&IExpandCollapsePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExpandCollapsePatternIdentifiers, IExpandCollapsePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ExpandCollapsePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers;{b006bac0-751b-4d55-92cb-613ec1bdf5d0})");
}
unsafe impl ::windows::core::Interface for ExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb006bac0_751b_4d55_92cb_613ec1bdf5d0);
}
impl ::windows::core::RuntimeName for ExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExpandCollapsePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ExpandCollapsePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: ExpandCollapseState = ExpandCollapseState(0i32);
    pub const Expanded: ExpandCollapseState = ExpandCollapseState(1i32);
    pub const PartiallyExpanded: ExpandCollapseState = ExpandCollapseState(2i32);
    pub const LeafNode: ExpandCollapseState = ExpandCollapseState(3i32);
}
impl ::core::convert::From<i32> for ExpandCollapseState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExpandCollapseState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExpandCollapseState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ExpandCollapseState;i4)");
}
impl ::windows::core::DefaultType for ExpandCollapseState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GridItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl GridItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnSpanProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ContainingGridProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowSpanProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridItemPatternIdentifiersStatics<R, F: FnOnce(&IGridItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GridItemPatternIdentifiers, IGridItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GridItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.GridItemPatternIdentifiers;{757744f1-3285-4fb1-803b-2545bd431599})");
}
unsafe impl ::windows::core::Interface for GridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757744f1_3285_4fb1_803b_2545bd431599);
}
impl ::windows::core::RuntimeName for GridItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.GridItemPatternIdentifiers";
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GridItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GridPatternIdentifiers(pub ::windows::core::IInspectable);
impl GridPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnCountProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowCountProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridPatternIdentifiersStatics<R, F: FnOnce(&IGridPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GridPatternIdentifiers, IGridPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GridPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.GridPatternIdentifiers;{c902980f-96c5-450c-9044-7e52c24f9e94})");
}
unsafe impl ::windows::core::Interface for GridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc902980f_96c5_450c_9044_7e52c24f9e94);
}
impl ::windows::core::RuntimeName for GridPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.GridPatternIdentifiers";
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: GridPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &GridPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: GridPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &GridPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GridPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridPatternIdentifiers {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd475a0c1_48b2_4e40_a6cf_3dc4b638c0de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAnnotationPatternIdentifiersStatics {
    type Vtable = IAnnotationPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0e3a35d_d167_46dc_95ab_330af61aebb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersStatics_abi(
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
pub struct IAutomationAnnotation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationAnnotation {
    type Vtable = IAutomationAnnotation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb3c30ca_03d8_4618_91bf_e4d84f4af318);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AnnotationType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AnnotationType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationAnnotationFactory {
    type Vtable = IAutomationAnnotationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4906fa52_ddc0_4e69_b76b_019d928d822f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AnnotationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AnnotationType, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationAnnotationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationAnnotationStatics {
    type Vtable = IAutomationAnnotationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe503eab7_4ee5_48cb_b5b8_bbcd46c9d1da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationStatics_abi(
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
pub struct IAutomationElementIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe68a63cf_4345_4e2d_8a6a_49cce1fa2dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics {
    type Vtable = IAutomationElementIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4549399f_8340_4d67_b9bf_8c2ac6a0773a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics2 {
    type Vtable = IAutomationElementIdentifiersStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5cbb1e2_d55f_46a9_9eda_1a4742515dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics2_abi(
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
pub struct IAutomationElementIdentifiersStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics3 {
    type Vtable = IAutomationElementIdentifiersStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f5cbebd_b3eb_4083_adc7_0c2f39bb3543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics3_abi(
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
pub struct IAutomationElementIdentifiersStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics4 {
    type Vtable = IAutomationElementIdentifiersStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5af51f75_5913_4d78_b330_a6f50b73ed9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics4_abi(
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
pub struct IAutomationElementIdentifiersStatics5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics5 {
    type Vtable = IAutomationElementIdentifiersStatics5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986a8206_de59_42f9_a1e7_62b8af9e756d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics5_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics6 {
    type Vtable = IAutomationElementIdentifiersStatics6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde52b00d_8328_4eae_8035_f8db99c8bac4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics6_abi(
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
pub struct IAutomationElementIdentifiersStatics7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics7 {
    type Vtable = IAutomationElementIdentifiersStatics7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f1abb2_742c_446a_a8f6_1672b10d2874);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics7_abi(
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
pub struct IAutomationElementIdentifiersStatics8(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics8 {
    type Vtable = IAutomationElementIdentifiersStatics8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8517b060_806c_5dc5_bc41_891bb5a47adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics8_abi(
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
pub struct IAutomationProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationProperties {
    type Vtable = IAutomationProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68d7232c_e622_48e9_af0b_1ffa33cc5cba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics {
    type Vtable = IAutomationPropertiesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb618fd7b_32d0_4970_9c42_7c039ac7be78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLiveSetting) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: Peers::AutomationLiveSetting) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics2 {
    type Vtable = IAutomationPropertiesStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3976547f_7089_4801_8f1d_aab78090d1a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut Peers::AccessibilityView) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: Peers::AccessibilityView) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics3 {
    type Vtable = IAutomationPropertiesStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b75d735_5cb1_42ad_9b57_5faba8c1867f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics4 {
    type Vtable = IAutomationPropertiesStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7d62655_311a_4b7c_a131_524e89cd3cf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLandmarkType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: Peers::AutomationLandmarkType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics5 {
    type Vtable = IAutomationPropertiesStatics5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be35b26_c8f9_41a2_b4db_e6a7a32b0c34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics6 {
    type Vtable = IAutomationPropertiesStatics6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61e030f_eb49_4e5d_b012_4c1c96c3901b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics7 {
    type Vtable = IAutomationPropertiesStatics7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7e98bf3_8f91_4068_a4ad_b7b402d10a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics7_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics8(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics8 {
    type Vtable = IAutomationPropertiesStatics8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x432eca20_171a_560d_8524_3e651d3ad6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics8_abi(
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
pub struct IAutomationPropertiesStatics9(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics9 {
    type Vtable = IAutomationPropertiesStatics9_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f20b1d1_87b2_5562_8077_da593edafd2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics9_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationControlType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: Peers::AutomationControlType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationProperty(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationProperty {
    type Vtable = IAutomationProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb627195b_3227_4e16_9534_ddece30ddb46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDockPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccd7f4e6_e4f9_47ff_bde7_378b11f78e09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDockPatternIdentifiersStatics {
    type Vtable = IDockPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b87245c_ed80_4fe5_8eb4_708a39c841e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersStatics_abi(
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
pub struct IDragPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6266e985_4d07_4e80_82eb_8f96690a1a0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragPatternIdentifiersStatics {
    type Vtable = IDragPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a05379d_1755_4082_9d90_46f1411d7986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersStatics_abi(
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
pub struct IDropTargetPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11865133_a6fe_4634_bd18_0ef612b7b208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDropTargetPatternIdentifiersStatics {
    type Vtable = IDropTargetPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b693304_89fb_4b0a_9452_ca2c66aaf9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersStatics_abi(
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
pub struct IExpandCollapsePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb006bac0_751b_4d55_92cb_613ec1bdf5d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExpandCollapsePatternIdentifiersStatics {
    type Vtable = IExpandCollapsePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7816fd4_6ee0_4f38_8e14_56ef21adacfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersStatics_abi(
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
pub struct IGridItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757744f1_3285_4fb1_803b_2545bd431599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridItemPatternIdentifiersStatics {
    type Vtable = IGridItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x217d2402_5e46_4d61_8794_b8ee8e774714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersStatics_abi(
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
pub struct IGridPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc902980f_96c5_450c_9044_7e52c24f9e94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridPatternIdentifiersStatics {
    type Vtable = IGridPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bc452f3_a181_4137_8de9_1f9b1a8320ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersStatics_abi(
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
pub struct IMultipleViewPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d5cd3b8_1e12_488b_b0ea_5e6cb89816e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMultipleViewPatternIdentifiersStatics {
    type Vtable = IMultipleViewPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9cfa66f_6b84_4d71_9e48_d764d3bcda8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersStatics_abi(
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
pub struct IRangeValuePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8760f45_33c9_467d_bc9e_d1515263ace1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeValuePatternIdentifiersStatics {
    type Vtable = IRangeValuePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce23450f_1c27_457f_b815_7a5e46863dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x366b1003_425c_4951_ae83_d521e73bc696);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollPatternIdentifiersStatics {
    type Vtable = IScrollPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bf8e0a1_fb7f_4fa4_83b3_cfaeb103a685);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dafa41a_3ef8_4bb5_a02b_3ee1b2274740);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionItemPatternIdentifiersStatics {
    type Vtable = ISelectionItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa918d163_487e_4e3e_9f86_7b44acbe27ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersStatics_abi(
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
pub struct ISelectionPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aa66fb0_e3f7_475f_b78d_f8a83bb730c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionPatternIdentifiersStatics {
    type Vtable = ISelectionPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93035b4c_6b50_40a1_b23f_5c78ddbd479a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersStatics_abi(
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
pub struct ISpreadsheetItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84347e19_ca4b_46a2_a794_c87928a3b1ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpreadsheetItemPatternIdentifiersStatics {
    type Vtable = ISpreadsheetItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43658779_5380_4f12_b468_b4f368ad4499);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersStatics_abi(
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
pub struct IStylesPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0e4e201_e89d_436b_8287_4f7903466879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStylesPatternIdentifiersStatics {
    type Vtable = IStylesPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x528a457a_bc3c_4d48_94af_1f68703ca296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc326e5ad_8077_4c64_98e4_e83bcf1b4389);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITableItemPatternIdentifiersStatics {
    type Vtable = ITableItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c4b923_e9a2_4de9_b2a4_a8b22d0be362);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersStatics_abi(
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
pub struct ITablePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38d104fe_0d0c_412a_bf8d_51ede683baf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITablePatternIdentifiersStatics {
    type Vtable = ITablePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75073d25_32c9_4903_aecf_dc3504cbd244);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersStatics_abi(
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
pub struct ITogglePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e191f6b_34d4_4ae7_83ac_29f88882d985);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITogglePatternIdentifiersStatics {
    type Vtable = ITogglePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7f75544_14a5_4f2f_92fc_760524de06ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersStatics_abi(
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
pub struct ITransformPattern2Identifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08aaa03d_dea7_402f_8097_9a2783d60e5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2Identifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPattern2IdentifiersStatics {
    type Vtable = ITransformPattern2IdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78963644_11f0_467c_a72b_5dac41c1f6fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersStatics_abi(
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
pub struct ITransformPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4115b8c_c3c8_4a37_b994_2709a7811665);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPatternIdentifiersStatics {
    type Vtable = ITransformPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4570edab_d705_40c4_a1dc_e9acfcef85f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersStatics_abi(
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
pub struct IValuePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x425bf64c_5333_4e41_b470_2bad14ecd085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IValuePatternIdentifiersStatics {
    type Vtable = IValuePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc247e8f7_adcc_440f_b123_33788a40525a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersStatics_abi(
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
pub struct IWindowPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f78bb4_7032_41e2_b79e_27b74a8628de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowPatternIdentifiersStatics {
    type Vtable = IWindowPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07d0ad06_6302_4d29_878b_19da03fc228d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersStatics_abi(
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
);
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MultipleViewPatternIdentifiers(pub ::windows::core::IInspectable);
impl MultipleViewPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CurrentViewProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SupportedViewsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IMultipleViewPatternIdentifiersStatics<R, F: FnOnce(&IMultipleViewPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MultipleViewPatternIdentifiers, IMultipleViewPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MultipleViewPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers;{5d5cd3b8-1e12-488b-b0ea-5e6cb89816e1})");
}
unsafe impl ::windows::core::Interface for MultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d5cd3b8_1e12_488b_b0ea_5e6cb89816e1);
}
impl ::windows::core::RuntimeName for MultipleViewPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MultipleViewPatternIdentifiers {}
unsafe impl ::core::marker::Sync for MultipleViewPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RangeValuePatternIdentifiers(pub ::windows::core::IInspectable);
impl RangeValuePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsReadOnlyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LargeChangeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MaximumProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MinimumProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SmallChangeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ValueProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IRangeValuePatternIdentifiersStatics<R, F: FnOnce(&IRangeValuePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RangeValuePatternIdentifiers, IRangeValuePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RangeValuePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers;{f8760f45-33c9-467d-bc9e-d1515263ace1})");
}
unsafe impl ::windows::core::Interface for RangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8760f45_33c9_467d_bc9e_d1515263ace1);
}
impl ::windows::core::RuntimeName for RangeValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers";
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RangeValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for RangeValuePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: RowOrColumnMajor = RowOrColumnMajor(0i32);
    pub const ColumnMajor: RowOrColumnMajor = RowOrColumnMajor(1i32);
    pub const Indeterminate: RowOrColumnMajor = RowOrColumnMajor(2i32);
}
impl ::core::convert::From<i32> for RowOrColumnMajor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RowOrColumnMajor {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RowOrColumnMajor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.RowOrColumnMajor;i4)");
}
impl ::windows::core::DefaultType for RowOrColumnMajor {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: ScrollAmount = ScrollAmount(0i32);
    pub const SmallDecrement: ScrollAmount = ScrollAmount(1i32);
    pub const NoAmount: ScrollAmount = ScrollAmount(2i32);
    pub const LargeIncrement: ScrollAmount = ScrollAmount(3i32);
    pub const SmallIncrement: ScrollAmount = ScrollAmount(4i32);
}
impl ::core::convert::From<i32> for ScrollAmount {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ScrollAmount {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ScrollAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ScrollAmount;i4)");
}
impl ::windows::core::DefaultType for ScrollAmount {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScrollPatternIdentifiers(pub ::windows::core::IInspectable);
impl ScrollPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HorizontallyScrollableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HorizontalScrollPercentProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HorizontalViewSizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn NoScroll() -> ::windows::core::Result<f64> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn VerticallyScrollableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn VerticalScrollPercentProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn VerticalViewSizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IScrollPatternIdentifiersStatics<R, F: FnOnce(&IScrollPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScrollPatternIdentifiers, IScrollPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ScrollPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ScrollPatternIdentifiers;{366b1003-425c-4951-ae83-d521e73bc696})");
}
unsafe impl ::windows::core::Interface for ScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x366b1003_425c_4951_ae83_d521e73bc696);
}
impl ::windows::core::RuntimeName for ScrollPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ScrollPatternIdentifiers";
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScrollPatternIdentifiers {}
unsafe impl ::core::marker::Sync for ScrollPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SelectionItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl SelectionItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsSelectedProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SelectionContainerProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionItemPatternIdentifiersStatics<R, F: FnOnce(&ISelectionItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SelectionItemPatternIdentifiers, ISelectionItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers;{2dafa41a-3ef8-4bb5-a02b-3ee1b2274740})");
}
unsafe impl ::windows::core::Interface for SelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dafa41a_3ef8_4bb5_a02b_3ee1b2274740);
}
impl ::windows::core::RuntimeName for SelectionItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SelectionItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SelectionPatternIdentifiers(pub ::windows::core::IInspectable);
impl SelectionPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanSelectMultipleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsSelectionRequiredProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SelectionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionPatternIdentifiersStatics<R, F: FnOnce(&ISelectionPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SelectionPatternIdentifiers, ISelectionPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SelectionPatternIdentifiers;{4aa66fb0-e3f7-475f-b78d-f8a83bb730c4})");
}
unsafe impl ::windows::core::Interface for SelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aa66fb0_e3f7_475f_b78d_f8a83bb730c4);
}
impl ::windows::core::RuntimeName for SelectionPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SelectionPatternIdentifiers";
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SelectionPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpreadsheetItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl SpreadsheetItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FormulaProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISpreadsheetItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISpreadsheetItemPatternIdentifiersStatics<R, F: FnOnce(&ISpreadsheetItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpreadsheetItemPatternIdentifiers, ISpreadsheetItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpreadsheetItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers;{84347e19-ca4b-46a2-a794-c87928a3b1ab})");
}
unsafe impl ::windows::core::Interface for SpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84347e19_ca4b_46a2_a794_c87928a3b1ab);
}
impl ::windows::core::RuntimeName for SpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpreadsheetItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SpreadsheetItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StylesPatternIdentifiers(pub ::windows::core::IInspectable);
impl StylesPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ExtendedPropertiesProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FillColorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FillPatternColorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FillPatternStyleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ShapeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn StyleIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn StyleNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IStylesPatternIdentifiersStatics<R, F: FnOnce(&IStylesPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StylesPatternIdentifiers, IStylesPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StylesPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.StylesPatternIdentifiers;{b0e4e201-e89d-436b-8287-4f7903466879})");
}
unsafe impl ::windows::core::Interface for StylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0e4e201_e89d_436b_8287_4f7903466879);
}
impl ::windows::core::RuntimeName for StylesPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.StylesPatternIdentifiers";
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: StylesPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: StylesPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StylesPatternIdentifiers {}
unsafe impl ::core::marker::Sync for StylesPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: SupportedTextSelection = SupportedTextSelection(0i32);
    pub const Single: SupportedTextSelection = SupportedTextSelection(1i32);
    pub const Multiple: SupportedTextSelection = SupportedTextSelection(2i32);
}
impl ::core::convert::From<i32> for SupportedTextSelection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SupportedTextSelection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SupportedTextSelection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.SupportedTextSelection;i4)");
}
impl ::windows::core::DefaultType for SupportedTextSelection {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: SynchronizedInputType = SynchronizedInputType(1i32);
    pub const KeyDown: SynchronizedInputType = SynchronizedInputType(2i32);
    pub const LeftMouseUp: SynchronizedInputType = SynchronizedInputType(4i32);
    pub const LeftMouseDown: SynchronizedInputType = SynchronizedInputType(8i32);
    pub const RightMouseUp: SynchronizedInputType = SynchronizedInputType(16i32);
    pub const RightMouseDown: SynchronizedInputType = SynchronizedInputType(32i32);
}
impl ::core::convert::From<i32> for SynchronizedInputType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SynchronizedInputType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SynchronizedInputType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.SynchronizedInputType;i4)");
}
impl ::windows::core::DefaultType for SynchronizedInputType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TableItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl TableItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnHeaderItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowHeaderItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITableItemPatternIdentifiersStatics<R, F: FnOnce(&ITableItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TableItemPatternIdentifiers, ITableItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TableItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TableItemPatternIdentifiers;{c326e5ad-8077-4c64-98e4-e83bcf1b4389})");
}
unsafe impl ::windows::core::Interface for TableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc326e5ad_8077_4c64_98e4_e83bcf1b4389);
}
impl ::windows::core::RuntimeName for TableItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TableItemPatternIdentifiers";
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TableItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TableItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TablePatternIdentifiers(pub ::windows::core::IInspectable);
impl TablePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnHeadersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowHeadersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowOrColumnMajorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITablePatternIdentifiersStatics<R, F: FnOnce(&ITablePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TablePatternIdentifiers, ITablePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TablePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TablePatternIdentifiers;{38d104fe-0d0c-412a-bf8d-51ede683baf5})");
}
unsafe impl ::windows::core::Interface for TablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38d104fe_0d0c_412a_bf8d_51ede683baf5);
}
impl ::windows::core::RuntimeName for TablePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TablePatternIdentifiers";
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TablePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TablePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TablePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TablePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TablePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TablePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TogglePatternIdentifiers(pub ::windows::core::IInspectable);
impl TogglePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ToggleStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITogglePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITogglePatternIdentifiersStatics<R, F: FnOnce(&ITogglePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TogglePatternIdentifiers, ITogglePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TogglePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TogglePatternIdentifiers;{7e191f6b-34d4-4ae7-83ac-29f88882d985})");
}
unsafe impl ::windows::core::Interface for TogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e191f6b_34d4_4ae7_83ac_29f88882d985);
}
impl ::windows::core::RuntimeName for TogglePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TogglePatternIdentifiers";
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TogglePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TogglePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TogglePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TogglePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: ToggleState = ToggleState(0i32);
    pub const On: ToggleState = ToggleState(1i32);
    pub const Indeterminate: ToggleState = ToggleState(2i32);
}
impl ::core::convert::From<i32> for ToggleState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ToggleState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ToggleState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ToggleState;i4)");
}
impl ::windows::core::DefaultType for ToggleState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TransformPattern2Identifiers(pub ::windows::core::IInspectable);
impl TransformPattern2Identifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ZoomLevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MaxZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MinZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPattern2IdentifiersStatics<R, F: FnOnce(&ITransformPattern2IdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransformPattern2Identifiers, ITransformPattern2IdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TransformPattern2Identifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TransformPattern2Identifiers;{08aaa03d-dea7-402f-8097-9a2783d60e5d})");
}
unsafe impl ::windows::core::Interface for TransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08aaa03d_dea7_402f_8097_9a2783d60e5d);
}
impl ::windows::core::RuntimeName for TransformPattern2Identifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TransformPattern2Identifiers";
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows::core::IUnknown {
    fn from(value: TransformPattern2Identifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows::core::IUnknown {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows::core::IInspectable {
    fn from(value: TransformPattern2Identifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows::core::IInspectable {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TransformPattern2Identifiers {}
unsafe impl ::core::marker::Sync for TransformPattern2Identifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TransformPatternIdentifiers(pub ::windows::core::IInspectable);
impl TransformPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanMoveProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanResizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanRotateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPatternIdentifiersStatics<R, F: FnOnce(&ITransformPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransformPatternIdentifiers, ITransformPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TransformPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TransformPatternIdentifiers;{e4115b8c-c3c8-4a37-b994-2709a7811665})");
}
unsafe impl ::windows::core::Interface for TransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4115b8c_c3c8_4a37_b994_2709a7811665);
}
impl ::windows::core::RuntimeName for TransformPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TransformPatternIdentifiers";
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TransformPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TransformPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TransformPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TransformPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ValuePatternIdentifiers(pub ::windows::core::IInspectable);
impl ValuePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsReadOnlyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ValueProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IValuePatternIdentifiersStatics<R, F: FnOnce(&IValuePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ValuePatternIdentifiers, IValuePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ValuePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ValuePatternIdentifiers;{425bf64c-5333-4e41-b470-2bad14ecd085})");
}
unsafe impl ::windows::core::Interface for ValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x425bf64c_5333_4e41_b470_2bad14ecd085);
}
impl ::windows::core::RuntimeName for ValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ValuePatternIdentifiers";
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ValuePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ValuePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ValuePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: WindowInteractionState = WindowInteractionState(0i32);
    pub const Closing: WindowInteractionState = WindowInteractionState(1i32);
    pub const ReadyForUserInteraction: WindowInteractionState = WindowInteractionState(2i32);
    pub const BlockedByModalWindow: WindowInteractionState = WindowInteractionState(3i32);
    pub const NotResponding: WindowInteractionState = WindowInteractionState(4i32);
}
impl ::core::convert::From<i32> for WindowInteractionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WindowInteractionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WindowInteractionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.WindowInteractionState;i4)");
}
impl ::windows::core::DefaultType for WindowInteractionState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WindowPatternIdentifiers(pub ::windows::core::IInspectable);
impl WindowPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanMaximizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanMinimizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsModalProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsTopmostProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn WindowInteractionStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn WindowVisualStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IWindowPatternIdentifiersStatics<R, F: FnOnce(&IWindowPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowPatternIdentifiers, IWindowPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WindowPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.WindowPatternIdentifiers;{39f78bb4-7032-41e2-b79e-27b74a8628de})");
}
unsafe impl ::windows::core::Interface for WindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f78bb4_7032_41e2_b79e_27b74a8628de);
}
impl ::windows::core::RuntimeName for WindowPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.WindowPatternIdentifiers";
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: WindowPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: WindowPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WindowPatternIdentifiers {}
unsafe impl ::core::marker::Sync for WindowPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: WindowVisualState = WindowVisualState(0i32);
    pub const Maximized: WindowVisualState = WindowVisualState(1i32);
    pub const Minimized: WindowVisualState = WindowVisualState(2i32);
}
impl ::core::convert::From<i32> for WindowVisualState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WindowVisualState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WindowVisualState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.WindowVisualState;i4)");
}
impl ::windows::core::DefaultType for WindowVisualState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: ZoomUnit = ZoomUnit(0i32);
    pub const LargeDecrement: ZoomUnit = ZoomUnit(1i32);
    pub const SmallDecrement: ZoomUnit = ZoomUnit(2i32);
    pub const LargeIncrement: ZoomUnit = ZoomUnit(3i32);
    pub const SmallIncrement: ZoomUnit = ZoomUnit(4i32);
}
impl ::core::convert::From<i32> for ZoomUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ZoomUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ZoomUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ZoomUnit;i4)");
}
impl ::windows::core::DefaultType for ZoomUnit {
    type DefaultType = Self;
}
