#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub mod Peers;
#[cfg(feature = "UI_Xaml_Automation_Provider")]
pub mod Provider;
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub mod Text;
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AnnotationPatternIdentifiers(::windows::core::IUnknown);
impl AnnotationPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AnnotationTypeIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AnnotationTypeNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AuthorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DateTimeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn TargetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAnnotationPatternIdentifiersStatics<R, F: FnOnce(&IAnnotationPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AnnotationPatternIdentifiers, IAnnotationPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AnnotationPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnnotationPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnnotationPatternIdentifiers {}
impl ::core::fmt::Debug for AnnotationPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnnotationPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnnotationPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers;{d475a0c1-48b2-4e40-a6cf-3dc4b638c0de})");
}
unsafe impl ::windows::core::Interface for AnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd475a0c1_48b2_4e40_a6cf_3dc4b638c0de);
}
impl ::windows::core::RuntimeName for AnnotationPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers";
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AnnotationPatternIdentifiers {}
unsafe impl ::core::marker::Sync for AnnotationPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: Self = Self(60000i32);
    pub const SpellingError: Self = Self(60001i32);
    pub const GrammarError: Self = Self(60002i32);
    pub const Comment: Self = Self(60003i32);
    pub const FormulaError: Self = Self(60004i32);
    pub const TrackChanges: Self = Self(60005i32);
    pub const Header: Self = Self(60006i32);
    pub const Footer: Self = Self(60007i32);
    pub const Highlighted: Self = Self(60008i32);
    pub const Endnote: Self = Self(60009i32);
    pub const Footnote: Self = Self(60010i32);
    pub const InsertionChange: Self = Self(60011i32);
    pub const DeletionChange: Self = Self(60012i32);
    pub const MoveChange: Self = Self(60013i32);
    pub const FormatChange: Self = Self(60014i32);
    pub const UnsyncedChange: Self = Self(60015i32);
    pub const EditingLockedChange: Self = Self(60016i32);
    pub const ExternalChange: Self = Self(60017i32);
    pub const ConflictingChange: Self = Self(60018i32);
    pub const Author: Self = Self(60019i32);
    pub const AdvancedProofingIssue: Self = Self(60020i32);
    pub const DataValidationError: Self = Self(60021i32);
    pub const CircularReferenceError: Self = Self(60022i32);
}
impl ::core::marker::Copy for AnnotationType {}
impl ::core::clone::Clone for AnnotationType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AnnotationType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AnnotationType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnnotationType {}
impl ::core::fmt::Debug for AnnotationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnnotationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnnotationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AnnotationType;i4)");
}
impl ::windows::core::DefaultType for AnnotationType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationActiveEnd {}
impl ::core::clone::Clone for AutomationActiveEnd {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationActiveEnd {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationActiveEnd {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationActiveEnd {}
impl ::core::fmt::Debug for AutomationActiveEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationActiveEnd").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationActiveEnd {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationActiveEnd;i4)");
}
impl ::windows::core::DefaultType for AutomationActiveEnd {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: Self = Self(0i32);
    pub const LasVegasLights: Self = Self(1i32);
    pub const BlinkingBackground: Self = Self(2i32);
    pub const SparkleText: Self = Self(3i32);
    pub const MarchingBlackAnts: Self = Self(4i32);
    pub const MarchingRedAnts: Self = Self(5i32);
    pub const Shimmer: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for AutomationAnimationStyle {}
impl ::core::clone::Clone for AutomationAnimationStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationAnimationStyle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationAnimationStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationAnimationStyle {}
impl ::core::fmt::Debug for AutomationAnimationStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationAnimationStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationAnimationStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationAnimationStyle;i4)");
}
impl ::windows::core::DefaultType for AutomationAnimationStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationAnnotation(::windows::core::IUnknown);
impl AutomationAnnotation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationAnnotation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn Type(&self) -> ::windows::core::Result<AnnotationType> {
        let this = self;
        unsafe {
            let mut result__: AnnotationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AnnotationType>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetType(&self, value: AnnotationType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn Element(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetElement<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CreateInstance(r#type: AnnotationType) -> ::windows::core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CreateWithElementParameter<'a, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(r#type: AnnotationType, element: Param1) -> ::windows::core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), r#type, element.into_param().abi(), &mut result__).from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn TypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ElementProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAutomationAnnotationFactory<R, F: FnOnce(&IAutomationAnnotationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationAnnotation, IAutomationAnnotationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationAnnotationStatics<R, F: FnOnce(&IAutomationAnnotationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationAnnotation, IAutomationAnnotationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AutomationAnnotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationAnnotation {}
impl ::core::fmt::Debug for AutomationAnnotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationAnnotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationAnnotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationAnnotation;{fb3c30ca-03d8-4618-91bf-e4d84f4af318})");
}
unsafe impl ::windows::core::Interface for AutomationAnnotation {
    type Vtable = IAutomationAnnotationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb3c30ca_03d8_4618_91bf_e4d84f4af318);
}
impl ::windows::core::RuntimeName for AutomationAnnotation {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationAnnotation";
}
impl ::core::convert::From<AutomationAnnotation> for ::windows::core::IUnknown {
    fn from(value: AutomationAnnotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows::core::IUnknown {
    fn from(value: &AutomationAnnotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationAnnotation> for ::windows::core::IInspectable {
    fn from(value: AutomationAnnotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows::core::IInspectable {
    fn from(value: &AutomationAnnotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationAnnotation> for super::DependencyObject {
    fn from(value: AutomationAnnotation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AutomationAnnotation> for super::DependencyObject {
    fn from(value: &AutomationAnnotation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AutomationAnnotation {}
unsafe impl ::core::marker::Sync for AutomationAnnotation {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: Self = Self(0i32);
    pub const HollowRoundBullet: Self = Self(1i32);
    pub const FilledRoundBullet: Self = Self(2i32);
    pub const HollowSquareBullet: Self = Self(3i32);
    pub const FilledSquareBullet: Self = Self(4i32);
    pub const DashBullet: Self = Self(5i32);
    pub const Other: Self = Self(6i32);
}
impl ::core::marker::Copy for AutomationBulletStyle {}
impl ::core::clone::Clone for AutomationBulletStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationBulletStyle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationBulletStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationBulletStyle {}
impl ::core::fmt::Debug for AutomationBulletStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationBulletStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationBulletStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationBulletStyle;i4)");
}
impl ::windows::core::DefaultType for AutomationBulletStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: Self = Self(0i32);
    pub const RTL: Self = Self(1i32);
}
impl ::core::marker::Copy for AutomationCaretBidiMode {}
impl ::core::clone::Clone for AutomationCaretBidiMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationCaretBidiMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationCaretBidiMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationCaretBidiMode {}
impl ::core::fmt::Debug for AutomationCaretBidiMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationCaretBidiMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationCaretBidiMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationCaretBidiMode;i4)");
}
impl ::windows::core::DefaultType for AutomationCaretBidiMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: Self = Self(0i32);
    pub const EndOfLine: Self = Self(1i32);
    pub const BeginningOfLine: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationCaretPosition {}
impl ::core::clone::Clone for AutomationCaretPosition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationCaretPosition {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationCaretPosition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationCaretPosition {}
impl ::core::fmt::Debug for AutomationCaretPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationCaretPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationCaretPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationCaretPosition;i4)");
}
impl ::windows::core::DefaultType for AutomationCaretPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationElementIdentifiers(::windows::core::IUnknown);
impl AutomationElementIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AcceleratorKeyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AutomationIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn BoundingRectangleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ClassNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ClickablePointProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ControlTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HasKeyboardFocusProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HelpTextProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsContentElementProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsControlElementProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsEnabledProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsKeyboardFocusableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsOffscreenProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsPasswordProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsRequiredForFormProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ItemStatusProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ItemTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LabeledByProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LocalizedControlTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn NameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn OrientationProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LiveSettingProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ControlledPeersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn PositionInSetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SizeOfSetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AnnotationsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LandmarkTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsPeripheralProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsDataValidForFormProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FullDescriptionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DescribedByProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FlowsToProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FlowsFromProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CultureProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics6(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HeadingLevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics7(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsDialogProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics<R, F: FnOnce(&IAutomationElementIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics2<R, F: FnOnce(&IAutomationElementIdentifiersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics3<R, F: FnOnce(&IAutomationElementIdentifiersStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics4<R, F: FnOnce(&IAutomationElementIdentifiersStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics5<R, F: FnOnce(&IAutomationElementIdentifiersStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics6<R, F: FnOnce(&IAutomationElementIdentifiersStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics7<R, F: FnOnce(&IAutomationElementIdentifiersStatics7) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics7> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics8<R, F: FnOnce(&IAutomationElementIdentifiersStatics8) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics8> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AutomationElementIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationElementIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationElementIdentifiers {}
impl ::core::fmt::Debug for AutomationElementIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationElementIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationElementIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationElementIdentifiers;{e68a63cf-4345-4e2d-8a6a-49cce1fa2dcc})");
}
unsafe impl ::windows::core::Interface for AutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe68a63cf_4345_4e2d_8a6a_49cce1fa2dcc);
}
impl ::windows::core::RuntimeName for AutomationElementIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationElementIdentifiers";
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows::core::IUnknown {
    fn from(value: AutomationElementIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows::core::IInspectable {
    fn from(value: AutomationElementIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationElementIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationElementIdentifiers {}
unsafe impl ::core::marker::Sync for AutomationElementIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
    pub const BottomToTop: Self = Self(2i32);
    pub const Vertical: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationFlowDirections {}
impl ::core::clone::Clone for AutomationFlowDirections {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationFlowDirections {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationFlowDirections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationFlowDirections {}
impl ::core::fmt::Debug for AutomationFlowDirections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationFlowDirections").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationFlowDirections {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationFlowDirections;i4)");
}
impl ::windows::core::DefaultType for AutomationFlowDirections {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: Self = Self(0i32);
    pub const Outline: Self = Self(1i32);
    pub const Shadow: Self = Self(2i32);
    pub const Engraved: Self = Self(3i32);
    pub const Embossed: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationOutlineStyles {}
impl ::core::clone::Clone for AutomationOutlineStyles {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationOutlineStyles {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationOutlineStyles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationOutlineStyles {}
impl ::core::fmt::Debug for AutomationOutlineStyles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationOutlineStyles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationOutlineStyles {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationOutlineStyles;i4)");
}
impl ::windows::core::DefaultType for AutomationOutlineStyles {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationProperties(::windows::core::IUnknown);
impl AutomationProperties {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AcceleratorKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetAcceleratorKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetAcceleratorKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AutomationIdProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetAutomationId<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetAutomationId<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HelpTextProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetHelpText<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetHelpText<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsRequiredForFormProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetIsRequiredForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetIsRequiredForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ItemStatusProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetItemStatus<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetItemStatus<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ItemTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetItemType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetItemType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LabeledByProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetLabeledBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::UIElement> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetLabeledBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn NameProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetName<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LiveSettingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetLiveSetting<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AutomationLiveSetting> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationLiveSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationLiveSetting>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetLiveSetting<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationLiveSetting) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AccessibilityViewProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetAccessibilityView<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AccessibilityView> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: Peers::AccessibilityView = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AccessibilityView>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetAccessibilityView<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AccessibilityView) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ControlledPeersProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControlledPeers<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::UIElement>> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::UIElement>>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn PositionInSetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetPositionInSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetPositionInSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SizeOfSetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetSizeOfSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetSizeOfSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LevelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AnnotationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAnnotations<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LandmarkTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AutomationLandmarkType> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: Peers::AutomationLandmarkType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationLandmarkType>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationLandmarkType) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetLocalizedLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetLocalizedLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsPeripheralProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetIsPeripheral<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetIsPeripheral<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsDataValidForFormProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetIsDataValidForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetIsDataValidForForm<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FullDescriptionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetFullDescription<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetFullDescription<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LocalizedControlTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetLocalizedControlType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetLocalizedControlType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DescribedByProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDescribedBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FlowsToProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFlowsTo<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FlowsFromProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFlowsFrom<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CultureProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics6(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetCulture<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics6(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetCulture<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HeadingLevelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics7(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetHeadingLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<Peers::AutomationHeadingLevel> {
        Self::IAutomationPropertiesStatics7(|this| unsafe {
            let mut result__: Peers::AutomationHeadingLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationHeadingLevel>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetHeadingLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationHeadingLevel) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics7(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsDialogProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GetIsDialog<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics8(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SetIsDialog<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics8(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn AutomationControlTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics9(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetAutomationControlType<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows::core::Result<Peers::AutomationControlType> {
        Self::IAutomationPropertiesStatics9(|this| unsafe {
            let mut result__: Peers::AutomationControlType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<Peers::AutomationControlType>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation', 'UI_Xaml_Automation_Peers'*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetAutomationControlType<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(element: Param0, value: Peers::AutomationControlType) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics9(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics<R, F: FnOnce(&IAutomationPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics2<R, F: FnOnce(&IAutomationPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics3<R, F: FnOnce(&IAutomationPropertiesStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics4<R, F: FnOnce(&IAutomationPropertiesStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics5<R, F: FnOnce(&IAutomationPropertiesStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics6<R, F: FnOnce(&IAutomationPropertiesStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics7<R, F: FnOnce(&IAutomationPropertiesStatics7) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics7> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics8<R, F: FnOnce(&IAutomationPropertiesStatics8) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics8> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics9<R, F: FnOnce(&IAutomationPropertiesStatics9) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics9> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AutomationProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationProperties {}
impl ::core::fmt::Debug for AutomationProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationProperties;{68d7232c-e622-48e9-af0b-1ffa33cc5cba})");
}
unsafe impl ::windows::core::Interface for AutomationProperties {
    type Vtable = IAutomationPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68d7232c_e622_48e9_af0b_1ffa33cc5cba);
}
impl ::windows::core::RuntimeName for AutomationProperties {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationProperties";
}
impl ::core::convert::From<AutomationProperties> for ::windows::core::IUnknown {
    fn from(value: AutomationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows::core::IUnknown {
    fn from(value: &AutomationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationProperties> for ::windows::core::IInspectable {
    fn from(value: AutomationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows::core::IInspectable {
    fn from(value: &AutomationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationProperties {}
unsafe impl ::core::marker::Sync for AutomationProperties {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationProperty(::windows::core::IUnknown);
impl AutomationProperty {}
impl ::core::clone::Clone for AutomationProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationProperty {}
impl ::core::fmt::Debug for AutomationProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationProperty;{b627195b-3227-4e16-9534-ddece30ddb46})");
}
unsafe impl ::windows::core::Interface for AutomationProperty {
    type Vtable = IAutomationPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb627195b_3227_4e16_9534_ddece30ddb46);
}
impl ::windows::core::RuntimeName for AutomationProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationProperty";
}
impl ::core::convert::From<AutomationProperty> for ::windows::core::IUnknown {
    fn from(value: AutomationProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows::core::IUnknown {
    fn from(value: &AutomationProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationProperty> for ::windows::core::IInspectable {
    fn from(value: AutomationProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows::core::IInspectable {
    fn from(value: &AutomationProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationProperty {}
unsafe impl ::core::marker::Sync for AutomationProperty {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: Self = Self(70001i32);
    pub const Heading2: Self = Self(70002i32);
    pub const Heading3: Self = Self(70003i32);
    pub const Heading4: Self = Self(70004i32);
    pub const Heading5: Self = Self(70005i32);
    pub const Heading6: Self = Self(70006i32);
    pub const Heading7: Self = Self(70007i32);
    pub const Heading8: Self = Self(70008i32);
    pub const Heading9: Self = Self(70009i32);
    pub const Title: Self = Self(70010i32);
    pub const Subtitle: Self = Self(70011i32);
    pub const Normal: Self = Self(70012i32);
    pub const Emphasis: Self = Self(70013i32);
    pub const Quote: Self = Self(70014i32);
    pub const BulletedList: Self = Self(70015i32);
}
impl ::core::marker::Copy for AutomationStyleId {}
impl ::core::clone::Clone for AutomationStyleId {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationStyleId {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationStyleId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationStyleId {}
impl ::core::fmt::Debug for AutomationStyleId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationStyleId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationStyleId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationStyleId;i4)");
}
impl ::windows::core::DefaultType for AutomationStyleId {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const WordsOnly: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const Dot: Self = Self(4i32);
    pub const Dash: Self = Self(5i32);
    pub const DashDot: Self = Self(6i32);
    pub const DashDotDot: Self = Self(7i32);
    pub const Wavy: Self = Self(8i32);
    pub const ThickSingle: Self = Self(9i32);
    pub const DoubleWavy: Self = Self(10i32);
    pub const ThickWavy: Self = Self(11i32);
    pub const LongDash: Self = Self(12i32);
    pub const ThickDash: Self = Self(13i32);
    pub const ThickDashDot: Self = Self(14i32);
    pub const ThickDashDotDot: Self = Self(15i32);
    pub const ThickDot: Self = Self(16i32);
    pub const ThickLongDash: Self = Self(17i32);
    pub const Other: Self = Self(18i32);
}
impl ::core::marker::Copy for AutomationTextDecorationLineStyle {}
impl ::core::clone::Clone for AutomationTextDecorationLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationTextDecorationLineStyle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationTextDecorationLineStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationTextDecorationLineStyle {}
impl ::core::fmt::Debug for AutomationTextDecorationLineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextDecorationLineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationTextDecorationLineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationTextDecorationLineStyle;i4)");
}
impl ::windows::core::DefaultType for AutomationTextDecorationLineStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: Self = Self(0i32);
    pub const AutoCorrect: Self = Self(1i32);
    pub const Composition: Self = Self(2i32);
    pub const CompositionFinalized: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationTextEditChangeType {}
impl ::core::clone::Clone for AutomationTextEditChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationTextEditChangeType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutomationTextEditChangeType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationTextEditChangeType {}
impl ::core::fmt::Debug for AutomationTextEditChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextEditChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationTextEditChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationTextEditChangeType;i4)");
}
impl ::windows::core::DefaultType for AutomationTextEditChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct DockPatternIdentifiers(::windows::core::IUnknown);
impl DockPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DockPositionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDockPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDockPatternIdentifiersStatics<R, F: FnOnce(&IDockPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DockPatternIdentifiers, IDockPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DockPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DockPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DockPatternIdentifiers {}
impl ::core::fmt::Debug for DockPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DockPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DockPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DockPatternIdentifiers;{ccd7f4e6-e4f9-47ff-bde7-378b11f78e09})");
}
unsafe impl ::windows::core::Interface for DockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccd7f4e6_e4f9_47ff_bde7_378b11f78e09);
}
impl ::windows::core::RuntimeName for DockPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DockPatternIdentifiers";
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DockPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DockPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DockPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DockPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DockPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DockPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Fill: Self = Self(4i32);
    pub const None: Self = Self(5i32);
}
impl ::core::marker::Copy for DockPosition {}
impl ::core::clone::Clone for DockPosition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DockPosition {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DockPosition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DockPosition {}
impl ::core::fmt::Debug for DockPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DockPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DockPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.DockPosition;i4)");
}
impl ::windows::core::DefaultType for DockPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct DragPatternIdentifiers(::windows::core::IUnknown);
impl DragPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DropEffectProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DropEffectsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn GrabbedItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsGrabbedProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDragPatternIdentifiersStatics<R, F: FnOnce(&IDragPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DragPatternIdentifiers, IDragPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragPatternIdentifiers {}
impl ::core::fmt::Debug for DragPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DragPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DragPatternIdentifiers;{6266e985-4d07-4e80-82eb-8f96690a1a0c})");
}
unsafe impl ::windows::core::Interface for DragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6266e985_4d07_4e80_82eb_8f96690a1a0c);
}
impl ::windows::core::RuntimeName for DragPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DragPatternIdentifiers";
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DragPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DragPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DragPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DragPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DragPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DragPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct DropTargetPatternIdentifiers(::windows::core::IUnknown);
impl DropTargetPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DropTargetEffectProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn DropTargetEffectsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDropTargetPatternIdentifiersStatics<R, F: FnOnce(&IDropTargetPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DropTargetPatternIdentifiers, IDropTargetPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DropTargetPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DropTargetPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DropTargetPatternIdentifiers {}
impl ::core::fmt::Debug for DropTargetPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DropTargetPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DropTargetPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers;{11865133-a6fe-4634-bd18-0ef612b7b208})");
}
unsafe impl ::windows::core::Interface for DropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11865133_a6fe_4634_bd18_0ef612b7b208);
}
impl ::windows::core::RuntimeName for DropTargetPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers";
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DropTargetPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DropTargetPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct ExpandCollapsePatternIdentifiers(::windows::core::IUnknown);
impl ExpandCollapsePatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ExpandCollapseStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IExpandCollapsePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IExpandCollapsePatternIdentifiersStatics<R, F: FnOnce(&IExpandCollapsePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExpandCollapsePatternIdentifiers, IExpandCollapsePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExpandCollapsePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpandCollapsePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpandCollapsePatternIdentifiers {}
impl ::core::fmt::Debug for ExpandCollapsePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpandCollapsePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpandCollapsePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers;{b006bac0-751b-4d55-92cb-613ec1bdf5d0})");
}
unsafe impl ::windows::core::Interface for ExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb006bac0_751b_4d55_92cb_613ec1bdf5d0);
}
impl ::windows::core::RuntimeName for ExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpandCollapsePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ExpandCollapsePatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: Self = Self(0i32);
    pub const Expanded: Self = Self(1i32);
    pub const PartiallyExpanded: Self = Self(2i32);
    pub const LeafNode: Self = Self(3i32);
}
impl ::core::marker::Copy for ExpandCollapseState {}
impl ::core::clone::Clone for ExpandCollapseState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ExpandCollapseState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ExpandCollapseState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpandCollapseState {}
impl ::core::fmt::Debug for ExpandCollapseState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpandCollapseState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExpandCollapseState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ExpandCollapseState;i4)");
}
impl ::windows::core::DefaultType for ExpandCollapseState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct GridItemPatternIdentifiers(::windows::core::IUnknown);
impl GridItemPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ColumnProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ColumnSpanProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ContainingGridProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn RowProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn RowSpanProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGridItemPatternIdentifiersStatics<R, F: FnOnce(&IGridItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GridItemPatternIdentifiers, IGridItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GridItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GridItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GridItemPatternIdentifiers {}
impl ::core::fmt::Debug for GridItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GridItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GridItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.GridItemPatternIdentifiers;{757744f1-3285-4fb1-803b-2545bd431599})");
}
unsafe impl ::windows::core::Interface for GridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757744f1_3285_4fb1_803b_2545bd431599);
}
impl ::windows::core::RuntimeName for GridItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.GridItemPatternIdentifiers";
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GridItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridItemPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct GridPatternIdentifiers(::windows::core::IUnknown);
impl GridPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ColumnCountProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn RowCountProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGridPatternIdentifiersStatics<R, F: FnOnce(&IGridPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GridPatternIdentifiers, IGridPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GridPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GridPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GridPatternIdentifiers {}
impl ::core::fmt::Debug for GridPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GridPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GridPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.GridPatternIdentifiers;{c902980f-96c5-450c-9044-7e52c24f9e94})");
}
unsafe impl ::windows::core::Interface for GridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc902980f_96c5_450c_9044_7e52c24f9e94);
}
impl ::windows::core::RuntimeName for GridPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.GridPatternIdentifiers";
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: GridPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &GridPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: GridPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &GridPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GridPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridPatternIdentifiers {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnnotationPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd475a0c1_48b2_4e40_a6cf_3dc4b638c0de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnnotationPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAnnotationPatternIdentifiersStatics {
    type Vtable = IAnnotationPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0e3a35d_d167_46dc_95ab_330af61aebb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersStaticsVtbl(
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
pub struct IAutomationAnnotation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationAnnotation {
    type Vtable = IAutomationAnnotationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb3c30ca_03d8_4618_91bf_e4d84f4af318);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AnnotationType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AnnotationType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationAnnotationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationAnnotationFactory {
    type Vtable = IAutomationAnnotationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4906fa52_ddc0_4e69_b76b_019d928d822f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AnnotationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AnnotationType, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationAnnotationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationAnnotationStatics {
    type Vtable = IAutomationAnnotationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe503eab7_4ee5_48cb_b5b8_bbcd46c9d1da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationStaticsVtbl(
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
pub struct IAutomationElementIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe68a63cf_4345_4e2d_8a6a_49cce1fa2dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics {
    type Vtable = IAutomationElementIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4549399f_8340_4d67_b9bf_8c2ac6a0773a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStaticsVtbl(
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
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics2 {
    type Vtable = IAutomationElementIdentifiersStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5cbb1e2_d55f_46a9_9eda_1a4742515dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics2Vtbl(
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
pub struct IAutomationElementIdentifiersStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics3 {
    type Vtable = IAutomationElementIdentifiersStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f5cbebd_b3eb_4083_adc7_0c2f39bb3543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics3Vtbl(
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
pub struct IAutomationElementIdentifiersStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics4 {
    type Vtable = IAutomationElementIdentifiersStatics4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5af51f75_5913_4d78_b330_a6f50b73ed9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics4Vtbl(
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
pub struct IAutomationElementIdentifiersStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics5 {
    type Vtable = IAutomationElementIdentifiersStatics5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986a8206_de59_42f9_a1e7_62b8af9e756d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics5Vtbl(
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
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics6 {
    type Vtable = IAutomationElementIdentifiersStatics6Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde52b00d_8328_4eae_8035_f8db99c8bac4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics6Vtbl(
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
pub struct IAutomationElementIdentifiersStatics7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics7 {
    type Vtable = IAutomationElementIdentifiersStatics7Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f1abb2_742c_446a_a8f6_1672b10d2874);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics7Vtbl(
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
pub struct IAutomationElementIdentifiersStatics8(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics8 {
    type Vtable = IAutomationElementIdentifiersStatics8Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8517b060_806c_5dc5_bc41_891bb5a47adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics8Vtbl(
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
pub struct IAutomationProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationProperties {
    type Vtable = IAutomationPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68d7232c_e622_48e9_af0b_1ffa33cc5cba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics {
    type Vtable = IAutomationPropertiesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb618fd7b_32d0_4970_9c42_7c039ac7be78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLiveSetting) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationLiveSetting) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics2 {
    type Vtable = IAutomationPropertiesStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3976547f_7089_4801_8f1d_aab78090d1a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AccessibilityView) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AccessibilityView) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics3 {
    type Vtable = IAutomationPropertiesStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b75d735_5cb1_42ad_9b57_5faba8c1867f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics4 {
    type Vtable = IAutomationPropertiesStatics4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7d62655_311a_4b7c_a131_524e89cd3cf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationLandmarkType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationLandmarkType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics5 {
    type Vtable = IAutomationPropertiesStatics5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be35b26_c8f9_41a2_b4db_e6a7a32b0c34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics6 {
    type Vtable = IAutomationPropertiesStatics6Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61e030f_eb49_4e5d_b012_4c1c96c3901b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics6Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics7 {
    type Vtable = IAutomationPropertiesStatics7Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7e98bf3_8f91_4068_a4ad_b7b402d10a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics7Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationHeadingLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics8(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics8 {
    type Vtable = IAutomationPropertiesStatics8Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x432eca20_171a_560d_8524_3e651d3ad6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics8Vtbl(
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
pub struct IAutomationPropertiesStatics9(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics9 {
    type Vtable = IAutomationPropertiesStatics9Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f20b1d1_87b2_5562_8077_da593edafd2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics9Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut Peers::AutomationControlType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: Peers::AutomationControlType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationProperty {
    type Vtable = IAutomationPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb627195b_3227_4e16_9534_ddece30ddb46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDockPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccd7f4e6_e4f9_47ff_bde7_378b11f78e09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDockPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDockPatternIdentifiersStatics {
    type Vtable = IDockPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b87245c_ed80_4fe5_8eb4_708a39c841e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersStaticsVtbl(
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
pub struct IDragPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6266e985_4d07_4e80_82eb_8f96690a1a0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDragPatternIdentifiersStatics {
    type Vtable = IDragPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a05379d_1755_4082_9d90_46f1411d7986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersStaticsVtbl(
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
pub struct IDropTargetPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11865133_a6fe_4634_bd18_0ef612b7b208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDropTargetPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDropTargetPatternIdentifiersStatics {
    type Vtable = IDropTargetPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b693304_89fb_4b0a_9452_ca2c66aaf9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersStaticsVtbl(
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
pub struct IExpandCollapsePatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb006bac0_751b_4d55_92cb_613ec1bdf5d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpandCollapsePatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExpandCollapsePatternIdentifiersStatics {
    type Vtable = IExpandCollapsePatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7816fd4_6ee0_4f38_8e14_56ef21adacfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersStaticsVtbl(
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
pub struct IGridItemPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757744f1_3285_4fb1_803b_2545bd431599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridItemPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGridItemPatternIdentifiersStatics {
    type Vtable = IGridItemPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x217d2402_5e46_4d61_8794_b8ee8e774714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersStaticsVtbl(
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
pub struct IGridPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc902980f_96c5_450c_9044_7e52c24f9e94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGridPatternIdentifiersStatics {
    type Vtable = IGridPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bc452f3_a181_4137_8de9_1f9b1a8320ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersStaticsVtbl(
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
pub struct IMultipleViewPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d5cd3b8_1e12_488b_b0ea_5e6cb89816e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultipleViewPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMultipleViewPatternIdentifiersStatics {
    type Vtable = IMultipleViewPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9cfa66f_6b84_4d71_9e48_d764d3bcda8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersStaticsVtbl(
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
pub struct IRangeValuePatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8760f45_33c9_467d_bc9e_d1515263ace1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeValuePatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRangeValuePatternIdentifiersStatics {
    type Vtable = IRangeValuePatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce23450f_1c27_457f_b815_7a5e46863dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersStaticsVtbl(
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
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IScrollPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x366b1003_425c_4951_ae83_d521e73bc696);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IScrollPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IScrollPatternIdentifiersStatics {
    type Vtable = IScrollPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bf8e0a1_fb7f_4fa4_83b3_cfaeb103a685);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectionItemPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dafa41a_3ef8_4bb5_a02b_3ee1b2274740);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectionItemPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectionItemPatternIdentifiersStatics {
    type Vtable = ISelectionItemPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa918d163_487e_4e3e_9f86_7b44acbe27ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersStaticsVtbl(
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
pub struct ISelectionPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aa66fb0_e3f7_475f_b78d_f8a83bb730c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectionPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectionPatternIdentifiersStatics {
    type Vtable = ISelectionPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93035b4c_6b50_40a1_b23f_5c78ddbd479a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersStaticsVtbl(
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
pub struct ISpreadsheetItemPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84347e19_ca4b_46a2_a794_c87928a3b1ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpreadsheetItemPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpreadsheetItemPatternIdentifiersStatics {
    type Vtable = ISpreadsheetItemPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43658779_5380_4f12_b468_b4f368ad4499);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersStaticsVtbl(
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
pub struct IStylesPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0e4e201_e89d_436b_8287_4f7903466879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStylesPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStylesPatternIdentifiersStatics {
    type Vtable = IStylesPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x528a457a_bc3c_4d48_94af_1f68703ca296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersStaticsVtbl(
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
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITableItemPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc326e5ad_8077_4c64_98e4_e83bcf1b4389);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITableItemPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITableItemPatternIdentifiersStatics {
    type Vtable = ITableItemPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c4b923_e9a2_4de9_b2a4_a8b22d0be362);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersStaticsVtbl(
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
pub struct ITablePatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38d104fe_0d0c_412a_bf8d_51ede683baf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITablePatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITablePatternIdentifiersStatics {
    type Vtable = ITablePatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75073d25_32c9_4903_aecf_dc3504cbd244);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersStaticsVtbl(
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
pub struct ITogglePatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e191f6b_34d4_4ae7_83ac_29f88882d985);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITogglePatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITogglePatternIdentifiersStatics {
    type Vtable = ITogglePatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7f75544_14a5_4f2f_92fc_760524de06ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersStaticsVtbl(
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
pub struct ITransformPattern2Identifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransformPattern2Identifiers {
    type Vtable = ITransformPattern2IdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08aaa03d_dea7_402f_8097_9a2783d60e5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformPattern2IdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransformPattern2IdentifiersStatics {
    type Vtable = ITransformPattern2IdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78963644_11f0_467c_a72b_5dac41c1f6fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersStaticsVtbl(
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
pub struct ITransformPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4115b8c_c3c8_4a37_b994_2709a7811665);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITransformPatternIdentifiersStatics {
    type Vtable = ITransformPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4570edab_d705_40c4_a1dc_e9acfcef85f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersStaticsVtbl(
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
pub struct IValuePatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x425bf64c_5333_4e41_b470_2bad14ecd085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IValuePatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IValuePatternIdentifiersStatics {
    type Vtable = IValuePatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc247e8f7_adcc_440f_b123_33788a40525a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersStaticsVtbl(
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
pub struct IWindowPatternIdentifiers(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f78bb4_7032_41e2_b79e_27b74a8628de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowPatternIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowPatternIdentifiersStatics {
    type Vtable = IWindowPatternIdentifiersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07d0ad06_6302_4d29_878b_19da03fc228d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersStaticsVtbl(
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
);
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct MultipleViewPatternIdentifiers(::windows::core::IUnknown);
impl MultipleViewPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CurrentViewProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SupportedViewsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMultipleViewPatternIdentifiersStatics<R, F: FnOnce(&IMultipleViewPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MultipleViewPatternIdentifiers, IMultipleViewPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MultipleViewPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultipleViewPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultipleViewPatternIdentifiers {}
impl ::core::fmt::Debug for MultipleViewPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MultipleViewPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MultipleViewPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers;{5d5cd3b8-1e12-488b-b0ea-5e6cb89816e1})");
}
unsafe impl ::windows::core::Interface for MultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d5cd3b8_1e12_488b_b0ea_5e6cb89816e1);
}
impl ::windows::core::RuntimeName for MultipleViewPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MultipleViewPatternIdentifiers {}
unsafe impl ::core::marker::Sync for MultipleViewPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct RangeValuePatternIdentifiers(::windows::core::IUnknown);
impl RangeValuePatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsReadOnlyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn LargeChangeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn MaximumProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn MinimumProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SmallChangeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ValueProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRangeValuePatternIdentifiersStatics<R, F: FnOnce(&IRangeValuePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RangeValuePatternIdentifiers, IRangeValuePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RangeValuePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RangeValuePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RangeValuePatternIdentifiers {}
impl ::core::fmt::Debug for RangeValuePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RangeValuePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RangeValuePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers;{f8760f45-33c9-467d-bc9e-d1515263ace1})");
}
unsafe impl ::windows::core::Interface for RangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8760f45_33c9_467d_bc9e_d1515263ace1);
}
impl ::windows::core::RuntimeName for RangeValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers";
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RangeValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for RangeValuePatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: Self = Self(0i32);
    pub const ColumnMajor: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for RowOrColumnMajor {}
impl ::core::clone::Clone for RowOrColumnMajor {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RowOrColumnMajor {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RowOrColumnMajor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RowOrColumnMajor {}
impl ::core::fmt::Debug for RowOrColumnMajor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RowOrColumnMajor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RowOrColumnMajor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.RowOrColumnMajor;i4)");
}
impl ::windows::core::DefaultType for RowOrColumnMajor {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: Self = Self(0i32);
    pub const SmallDecrement: Self = Self(1i32);
    pub const NoAmount: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ScrollAmount {}
impl ::core::clone::Clone for ScrollAmount {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ScrollAmount {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ScrollAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScrollAmount {}
impl ::core::fmt::Debug for ScrollAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollAmount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScrollAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ScrollAmount;i4)");
}
impl ::windows::core::DefaultType for ScrollAmount {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct ScrollPatternIdentifiers(::windows::core::IUnknown);
impl ScrollPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HorizontallyScrollableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HorizontalScrollPercentProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn HorizontalViewSizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn NoScroll() -> ::windows::core::Result<f64> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn VerticallyScrollableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn VerticalScrollPercentProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn VerticalViewSizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IScrollPatternIdentifiersStatics<R, F: FnOnce(&IScrollPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScrollPatternIdentifiers, IScrollPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScrollPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScrollPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScrollPatternIdentifiers {}
impl ::core::fmt::Debug for ScrollPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScrollPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ScrollPatternIdentifiers;{366b1003-425c-4951-ae83-d521e73bc696})");
}
unsafe impl ::windows::core::Interface for ScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x366b1003_425c_4951_ae83_d521e73bc696);
}
impl ::windows::core::RuntimeName for ScrollPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ScrollPatternIdentifiers";
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScrollPatternIdentifiers {}
unsafe impl ::core::marker::Sync for ScrollPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct SelectionItemPatternIdentifiers(::windows::core::IUnknown);
impl SelectionItemPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsSelectedProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SelectionContainerProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISelectionItemPatternIdentifiersStatics<R, F: FnOnce(&ISelectionItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SelectionItemPatternIdentifiers, ISelectionItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SelectionItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SelectionItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectionItemPatternIdentifiers {}
impl ::core::fmt::Debug for SelectionItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers;{2dafa41a-3ef8-4bb5-a02b-3ee1b2274740})");
}
unsafe impl ::windows::core::Interface for SelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dafa41a_3ef8_4bb5_a02b_3ee1b2274740);
}
impl ::windows::core::RuntimeName for SelectionItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SelectionItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionItemPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct SelectionPatternIdentifiers(::windows::core::IUnknown);
impl SelectionPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CanSelectMultipleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsSelectionRequiredProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn SelectionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISelectionPatternIdentifiersStatics<R, F: FnOnce(&ISelectionPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SelectionPatternIdentifiers, ISelectionPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SelectionPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SelectionPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectionPatternIdentifiers {}
impl ::core::fmt::Debug for SelectionPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SelectionPatternIdentifiers;{4aa66fb0-e3f7-475f-b78d-f8a83bb730c4})");
}
unsafe impl ::windows::core::Interface for SelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aa66fb0_e3f7_475f_b78d_f8a83bb730c4);
}
impl ::windows::core::RuntimeName for SelectionPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SelectionPatternIdentifiers";
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SelectionPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct SpreadsheetItemPatternIdentifiers(::windows::core::IUnknown);
impl SpreadsheetItemPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FormulaProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISpreadsheetItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpreadsheetItemPatternIdentifiersStatics<R, F: FnOnce(&ISpreadsheetItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpreadsheetItemPatternIdentifiers, ISpreadsheetItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpreadsheetItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpreadsheetItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpreadsheetItemPatternIdentifiers {}
impl ::core::fmt::Debug for SpreadsheetItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpreadsheetItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpreadsheetItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers;{84347e19-ca4b-46a2-a794-c87928a3b1ab})");
}
unsafe impl ::windows::core::Interface for SpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84347e19_ca4b_46a2_a794_c87928a3b1ab);
}
impl ::windows::core::RuntimeName for SpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpreadsheetItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SpreadsheetItemPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct StylesPatternIdentifiers(::windows::core::IUnknown);
impl StylesPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ExtendedPropertiesProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FillColorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FillPatternColorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn FillPatternStyleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ShapeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn StyleIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn StyleNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStylesPatternIdentifiersStatics<R, F: FnOnce(&IStylesPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StylesPatternIdentifiers, IStylesPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StylesPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StylesPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StylesPatternIdentifiers {}
impl ::core::fmt::Debug for StylesPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StylesPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StylesPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.StylesPatternIdentifiers;{b0e4e201-e89d-436b-8287-4f7903466879})");
}
unsafe impl ::windows::core::Interface for StylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0e4e201_e89d_436b_8287_4f7903466879);
}
impl ::windows::core::RuntimeName for StylesPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.StylesPatternIdentifiers";
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: StylesPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: StylesPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StylesPatternIdentifiers {}
unsafe impl ::core::marker::Sync for StylesPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for SupportedTextSelection {}
impl ::core::clone::Clone for SupportedTextSelection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SupportedTextSelection {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SupportedTextSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SupportedTextSelection {}
impl ::core::fmt::Debug for SupportedTextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SupportedTextSelection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SupportedTextSelection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.SupportedTextSelection;i4)");
}
impl ::windows::core::DefaultType for SupportedTextSelection {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: Self = Self(1i32);
    pub const KeyDown: Self = Self(2i32);
    pub const LeftMouseUp: Self = Self(4i32);
    pub const LeftMouseDown: Self = Self(8i32);
    pub const RightMouseUp: Self = Self(16i32);
    pub const RightMouseDown: Self = Self(32i32);
}
impl ::core::marker::Copy for SynchronizedInputType {}
impl ::core::clone::Clone for SynchronizedInputType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SynchronizedInputType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SynchronizedInputType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SynchronizedInputType {}
impl ::core::fmt::Debug for SynchronizedInputType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SynchronizedInputType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SynchronizedInputType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.SynchronizedInputType;i4)");
}
impl ::windows::core::DefaultType for SynchronizedInputType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct TableItemPatternIdentifiers(::windows::core::IUnknown);
impl TableItemPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ColumnHeaderItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn RowHeaderItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITableItemPatternIdentifiersStatics<R, F: FnOnce(&ITableItemPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TableItemPatternIdentifiers, ITableItemPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TableItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TableItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TableItemPatternIdentifiers {}
impl ::core::fmt::Debug for TableItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TableItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TableItemPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TableItemPatternIdentifiers;{c326e5ad-8077-4c64-98e4-e83bcf1b4389})");
}
unsafe impl ::windows::core::Interface for TableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc326e5ad_8077_4c64_98e4_e83bcf1b4389);
}
impl ::windows::core::RuntimeName for TableItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TableItemPatternIdentifiers";
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TableItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TableItemPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct TablePatternIdentifiers(::windows::core::IUnknown);
impl TablePatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ColumnHeadersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn RowHeadersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn RowOrColumnMajorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITablePatternIdentifiersStatics<R, F: FnOnce(&ITablePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TablePatternIdentifiers, ITablePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TablePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TablePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TablePatternIdentifiers {}
impl ::core::fmt::Debug for TablePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TablePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TablePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TablePatternIdentifiers;{38d104fe-0d0c-412a-bf8d-51ede683baf5})");
}
unsafe impl ::windows::core::Interface for TablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38d104fe_0d0c_412a_bf8d_51ede683baf5);
}
impl ::windows::core::RuntimeName for TablePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TablePatternIdentifiers";
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TablePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TablePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TablePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TablePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TablePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TablePatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct TogglePatternIdentifiers(::windows::core::IUnknown);
impl TogglePatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ToggleStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITogglePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITogglePatternIdentifiersStatics<R, F: FnOnce(&ITogglePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TogglePatternIdentifiers, ITogglePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TogglePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TogglePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TogglePatternIdentifiers {}
impl ::core::fmt::Debug for TogglePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TogglePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TogglePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TogglePatternIdentifiers;{7e191f6b-34d4-4ae7-83ac-29f88882d985})");
}
unsafe impl ::windows::core::Interface for TogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e191f6b_34d4_4ae7_83ac_29f88882d985);
}
impl ::windows::core::RuntimeName for TogglePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TogglePatternIdentifiers";
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TogglePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TogglePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TogglePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TogglePatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for ToggleState {}
impl ::core::clone::Clone for ToggleState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ToggleState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ToggleState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToggleState {}
impl ::core::fmt::Debug for ToggleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToggleState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToggleState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ToggleState;i4)");
}
impl ::windows::core::DefaultType for ToggleState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct TransformPattern2Identifiers(::windows::core::IUnknown);
impl TransformPattern2Identifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CanZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ZoomLevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn MaxZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn MinZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITransformPattern2IdentifiersStatics<R, F: FnOnce(&ITransformPattern2IdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransformPattern2Identifiers, ITransformPattern2IdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TransformPattern2Identifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TransformPattern2Identifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TransformPattern2Identifiers {}
impl ::core::fmt::Debug for TransformPattern2Identifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformPattern2Identifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TransformPattern2Identifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TransformPattern2Identifiers;{08aaa03d-dea7-402f-8097-9a2783d60e5d})");
}
unsafe impl ::windows::core::Interface for TransformPattern2Identifiers {
    type Vtable = ITransformPattern2IdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08aaa03d_dea7_402f_8097_9a2783d60e5d);
}
impl ::windows::core::RuntimeName for TransformPattern2Identifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TransformPattern2Identifiers";
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows::core::IUnknown {
    fn from(value: TransformPattern2Identifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows::core::IUnknown {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows::core::IInspectable {
    fn from(value: TransformPattern2Identifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows::core::IInspectable {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TransformPattern2Identifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TransformPattern2Identifiers {}
unsafe impl ::core::marker::Sync for TransformPattern2Identifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct TransformPatternIdentifiers(::windows::core::IUnknown);
impl TransformPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CanMoveProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CanResizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CanRotateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITransformPatternIdentifiersStatics<R, F: FnOnce(&ITransformPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TransformPatternIdentifiers, ITransformPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TransformPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TransformPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TransformPatternIdentifiers {}
impl ::core::fmt::Debug for TransformPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TransformPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TransformPatternIdentifiers;{e4115b8c-c3c8-4a37-b994-2709a7811665})");
}
unsafe impl ::windows::core::Interface for TransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4115b8c_c3c8_4a37_b994_2709a7811665);
}
impl ::windows::core::RuntimeName for TransformPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TransformPatternIdentifiers";
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TransformPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TransformPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TransformPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TransformPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct ValuePatternIdentifiers(::windows::core::IUnknown);
impl ValuePatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsReadOnlyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn ValueProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IValuePatternIdentifiersStatics<R, F: FnOnce(&IValuePatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ValuePatternIdentifiers, IValuePatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ValuePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ValuePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ValuePatternIdentifiers {}
impl ::core::fmt::Debug for ValuePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValuePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ValuePatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ValuePatternIdentifiers;{425bf64c-5333-4e41-b470-2bad14ecd085})");
}
unsafe impl ::windows::core::Interface for ValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x425bf64c_5333_4e41_b470_2bad14ecd085);
}
impl ::windows::core::RuntimeName for ValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ValuePatternIdentifiers";
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ValuePatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: Self = Self(0i32);
    pub const Closing: Self = Self(1i32);
    pub const ReadyForUserInteraction: Self = Self(2i32);
    pub const BlockedByModalWindow: Self = Self(3i32);
    pub const NotResponding: Self = Self(4i32);
}
impl ::core::marker::Copy for WindowInteractionState {}
impl ::core::clone::Clone for WindowInteractionState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WindowInteractionState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WindowInteractionState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowInteractionState {}
impl ::core::fmt::Debug for WindowInteractionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowInteractionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowInteractionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.WindowInteractionState;i4)");
}
impl ::windows::core::DefaultType for WindowInteractionState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct WindowPatternIdentifiers(::windows::core::IUnknown);
impl WindowPatternIdentifiers {
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CanMaximizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn CanMinimizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsModalProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn IsTopmostProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn WindowInteractionStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Automation'*"]
    pub fn WindowVisualStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowPatternIdentifiersStatics<R, F: FnOnce(&IWindowPatternIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowPatternIdentifiers, IWindowPatternIdentifiersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowPatternIdentifiers {}
impl ::core::fmt::Debug for WindowPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowPatternIdentifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.WindowPatternIdentifiers;{39f78bb4-7032-41e2-b79e-27b74a8628de})");
}
unsafe impl ::windows::core::Interface for WindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f78bb4_7032_41e2_b79e_27b74a8628de);
}
impl ::windows::core::RuntimeName for WindowPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.WindowPatternIdentifiers";
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: WindowPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: WindowPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowPatternIdentifiers {}
unsafe impl ::core::marker::Sync for WindowPatternIdentifiers {}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: Self = Self(0i32);
    pub const Maximized: Self = Self(1i32);
    pub const Minimized: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowVisualState {}
impl ::core::clone::Clone for WindowVisualState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WindowVisualState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WindowVisualState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowVisualState {}
impl ::core::fmt::Debug for WindowVisualState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowVisualState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowVisualState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.WindowVisualState;i4)");
}
impl ::windows::core::DefaultType for WindowVisualState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation'*"]
#[repr(transparent)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: Self = Self(0i32);
    pub const LargeDecrement: Self = Self(1i32);
    pub const SmallDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ZoomUnit {}
impl ::core::clone::Clone for ZoomUnit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ZoomUnit {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ZoomUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ZoomUnit {}
impl ::core::fmt::Debug for ZoomUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ZoomUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ZoomUnit;i4)");
}
impl ::windows::core::DefaultType for ZoomUnit {
    type DefaultType = Self;
}
