#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompositionConditionalValue(pub ::windows::runtime::IInspectable);
impl CompositionConditionalValue {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<CompositionConditionalValue> {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<CompositionConditionalValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn ICompositionConditionalValueStatics<R, F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CompositionConditionalValue, ICompositionConditionalValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionConditionalValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionConditionalValue;{43250538-eb73-4561-a71d-1a43eaeb7a9b})");
}
unsafe impl ::windows::runtime::Interface for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x43250538_eb73_4561_a71d_1a43eaeb7a9b);
}
impl ::windows::runtime::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionConditionalValue";
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows::runtime::IUnknown {
    fn from(value: CompositionConditionalValue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows::runtime::IUnknown {
    fn from(value: &CompositionConditionalValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows::runtime::IInspectable {
    fn from(value: CompositionConditionalValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows::runtime::IInspectable {
    fn from(value: &CompositionConditionalValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<CompositionConditionalValue> for super::CompositionObject {
    fn from(value: CompositionConditionalValue) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for super::CompositionObject {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CompositionConditionalValue {}
unsafe impl ::core::marker::Sync for CompositionConditionalValue {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompositionInteractionSourceCollection(pub ::windows::runtime::IInspectable);
impl CompositionInteractionSourceCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<ICompositionInteractionSource>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ICompositionInteractionSource>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Count(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RemoveAll(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection;{1b468e4b-a5bf-47d8-a547-3894155a158c})");
}
unsafe impl ::windows::runtime::Interface for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b468e4b_a5bf_47d8_a547_3894155a158c);
}
impl ::windows::runtime::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for ::windows::runtime::IInspectable {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows::runtime::IInspectable {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::core::marker::Sync for CompositionInteractionSourceCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionConditionalValue(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x43250538_eb73_4561_a71d_1a43eaeb7a9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x090c4b72_8467_4d0a_9065_ac46b80a5522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Composition_Interactions`*"]
pub struct ICompositionInteractionSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x043b2431_06e3_495a_ba54_409f0017fac0);
}
impl ICompositionInteractionSource {}
unsafe impl ::windows::runtime::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{043b2431-06e3-495a-ba54-409f0017fac0}");
}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionInteractionSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionInteractionSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositionInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositionInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: ICompositionInteractionSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: &ICompositionInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ICompositionInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ICompositionInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b468e4b_a5bf_47d8_a547_3894155a158c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa78347e5_a9d1_4d02_985e_b930cd0b9da4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a8e8cb1_1000_4416_8363_cc27fb877308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, adjustment: f32, positionthreshold: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, adjustment: f32, positionthreshold: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modifiers: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modifiers: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modifiers: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, animation: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, animation: ::windows::runtime::RawPtr, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x25769a3e_ce6d_448c_8386_92620d240756);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conditionalvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conditionalvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe6c5d7a2_5c4b_42c6_84b7_f69441b18091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modifiers: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xebd222bc_04af_4ac7_847d_06ea36e80a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd3ef5da2_a254_40e4_88d5_44e4e16b5809);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8d1c8cf1_d7b0_434c_a5d2_2d7611864834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47d579b7_0985_5e99_b024_2f32c380c1a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x50012faa_1510_4142_a1a5_019b09f8857b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf2e771ed_b803_5137_9435_1c96e48721e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa0e2c920_26b4_4da2_8b61_5e683979bbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x993818fe_c94e_4b86_87f3_922665ba46b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x04922fdc_f154_4cb8_bf33_cc1ba611e6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8cc83dd6_ba7b_431a_844b_6eac9130f99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70acdaae_27dc_48ed_a3c3_6d61c9a029d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcfda55b0_5e3e_4289_932d_ee5f50e74283);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x86f7ec09_5096_4170_9cc8_df2fe101bb93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18ed4699_0745_4096_bcab_3a4e99569bcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87108cf2_e7ff_4f7d_9ffd_d72f1e409b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb1eb32f6_c26c_41f6_a189_fabc22b323cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x48ac1c2f_47bd_59af_a58c_79bd2eb9ef71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa7263939_a17b_4011_99fd_b5c24f143748);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x509652d6_d488_59cd_819f_f52310295b11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Composition_Interactions`*"]
pub struct IInteractionTrackerOwner(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdb2e8af3_4deb_4e53_b29c_b06c9f96d651);
}
impl IInteractionTrackerOwner {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn CustomAnimationStateEntered<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerCustomAnimationStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IdleStateEntered<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerIdleStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn InertiaStateEntered<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn InteractingStateEntered<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerInteractingStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestIgnored<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerRequestIgnoredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ValuesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerValuesChangedArgs>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{db2e8af3-4deb-4e53-b29c-b06c9f96d651}");
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: IInteractionTrackerOwner) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows::runtime::IInspectable {
    fn from(value: IInteractionTrackerOwner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows::runtime::IInspectable {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80dd82f1_ce25_488f_91dd_cb6455ccff2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbba5d7b7_6590_4498_8d6c_eb62b514c92a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, owner: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35e53720_46b7_5cb0_b505_f3d6884a6163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boundtracker1: ::windows::runtime::RawPtr, boundtracker2: ::windows::runtime::RawPtr, axismode: InteractionBindingAxisModes) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boundtracker1: ::windows::runtime::RawPtr, boundtracker2: ::windows::runtime::RawPtr, result__: *mut InteractionBindingAxisModes) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcf1578ef_d3df_4501_b9e6_f02fb22f73d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87e08ab0_3086_4853_a4b7_77882ad5d7e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7401d6c4_6c6d_48df_bc3e_171e227e7d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f17695c_162d_4c07_9400_c282b28276ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x82001a48_09c0_434f_8189_141c66df362f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca0e8a86_d8d6_4111_b088_70347bd2b0ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: VisualInteractionSourceRedirectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionChainingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionChainingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionSourceMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionSourceMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionChainingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionChainingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionSourceMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionSourceMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionChainingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionChainingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InteractionSourceMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InteractionSourceMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerpoint: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa914893_a73c_414d_80d0_249bad2fbd93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conditionalvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conditionalvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conditionalvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conditionalvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conditionalvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd941ef2a_0d5c_4057_92d7_c9711533204f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb2ca917c_e98a_41f2_b3c9_891c9266c8f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x369965e1_8645_4f75_ba00_6479cd10c8e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa979c032_5764_55e0_bc1f_0778786dcfde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: InteractionBindingAxisModes = InteractionBindingAxisModes(0u32);
    pub const PositionX: InteractionBindingAxisModes = InteractionBindingAxisModes(1u32);
    pub const PositionY: InteractionBindingAxisModes = InteractionBindingAxisModes(2u32);
    pub const Scale: InteractionBindingAxisModes = InteractionBindingAxisModes(4u32);
}
impl ::core::convert::From<u32> for InteractionBindingAxisModes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionBindingAxisModes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionBindingAxisModes;u4)");
}
impl ::windows::runtime::DefaultType for InteractionBindingAxisModes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: InteractionChainingMode = InteractionChainingMode(0i32);
    pub const Always: InteractionChainingMode = InteractionChainingMode(1i32);
    pub const Never: InteractionChainingMode = InteractionChainingMode(2i32);
}
impl ::core::convert::From<i32> for InteractionChainingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionChainingMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionChainingMode;i4)");
}
impl ::windows::runtime::DefaultType for InteractionChainingMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionSourceConfiguration(pub ::windows::runtime::IInspectable);
impl InteractionSourceConfiguration {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionXSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionYSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetScaleSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionSourceConfiguration;{a78347e5-a9d1-4d02-985e-b930cd0b9da4})");
}
unsafe impl ::windows::runtime::Interface for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa78347e5_a9d1_4d02_985e_b930cd0b9da4);
}
impl ::windows::runtime::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionSourceConfiguration";
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: InteractionSourceConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: InteractionSourceConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: InteractionSourceConfiguration) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::core::marker::Sync for InteractionSourceConfiguration {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: InteractionSourceMode = InteractionSourceMode(0i32);
    pub const EnabledWithInertia: InteractionSourceMode = InteractionSourceMode(1i32);
    pub const EnabledWithoutInertia: InteractionSourceMode = InteractionSourceMode(2i32);
}
impl ::core::convert::From<i32> for InteractionSourceMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionSourceMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceMode;i4)");
}
impl ::windows::runtime::DefaultType for InteractionSourceMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(0i32);
    pub const Enabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(1i32);
}
impl ::core::convert::From<i32> for InteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionSourceRedirectionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)");
}
impl ::windows::runtime::DefaultType for InteractionSourceRedirectionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTracker(pub ::windows::runtime::IInspectable);
impl InteractionTracker {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn InteractionSources(&self) -> ::windows::runtime::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CompositionInteractionSourceCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsPositionRoundingSuggested(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn MaxPosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn SetMaxPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn MaxScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMaxScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn MinPosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn SetMinPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn MinScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMinScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn NaturalRestingPosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalRestingScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Owner(&self) -> ::windows::runtime::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInteractionTrackerOwner>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`, `Foundation_Numerics`*"]
    pub fn PositionInertiaDecayRate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`, `Foundation_Numerics`*"]
    pub fn SetPositionInertiaDecayRate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn ScaleInertiaDecayRate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn SetScaleInertiaDecayRate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<f32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn AdjustPositionXIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), adjustment, positionthreshold).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn AdjustPositionYIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), adjustment, positionthreshold).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigurePositionXInertiaModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigurePositionYInertiaModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureScaleInertiaModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdatePosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdatePositionBy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, amount: Param0) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), amount.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePositionWithAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, animation: Param0) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), animation.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdatePositionWithAdditionalVelocity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, velocityinpixelspersecond: Param0) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), velocityinpixelspersecond.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdateScale<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: f32, centerpoint: Param1) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value, centerpoint.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdateScaleWithAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, animation: Param0, centerpoint: Param1) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), animation.into_param().abi(), centerpoint.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdateScaleWithAdditionalVelocity<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, velocityinpercentpersecond: f32, centerpoint: Param1) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), velocityinpercentpersecond, centerpoint.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTracker>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn CreateWithOwner<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>, Param1: ::windows::runtime::IntoParam<'a, IInteractionTrackerOwner>>(compositor: Param0, owner: Param1) -> ::windows::runtime::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), compositor.into_param().abi(), owner.into_param().abi(), &mut result__).from_abi::<InteractionTracker>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureCenterPointXInertiaModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureCenterPointYInertiaModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureVector2PositionInertiaModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>>(&self, modifiers: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdatePositionWithOption<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0, option: InteractionTrackerClampingOption) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), option, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdatePositionByWithOption<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, amount: Param0, option: InteractionTrackerClampingOption) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), amount.into_param().abi(), option, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsInertiaFromImpulse(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetBindingMode<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTracker>>(boundtracker1: Param0, boundtracker2: Param1, axismode: InteractionBindingAxisModes) -> ::windows::runtime::Result<()> {
        Self::IInteractionTrackerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), axismode).ok() })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn GetBindingMode<'a, Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>, Param1: ::windows::runtime::IntoParam<'a, InteractionTracker>>(boundtracker1: Param0, boundtracker2: Param1) -> ::windows::runtime::Result<InteractionBindingAxisModes> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__: InteractionBindingAxisModes = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), &mut result__).from_abi::<InteractionBindingAxisModes>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn TryUpdatePositionWithOption2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), option, posupdateoption, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn IInteractionTrackerStatics<R, F: FnOnce(&IInteractionTrackerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InteractionTracker, IInteractionTrackerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInteractionTrackerStatics2<R, F: FnOnce(&IInteractionTrackerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InteractionTracker, IInteractionTrackerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTracker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTracker;{2a8e8cb1-1000-4416-8363-cc27fb877308})");
}
unsafe impl ::windows::runtime::Interface for InteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a8e8cb1_1000_4416_8363_cc27fb877308);
}
impl ::windows::runtime::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTracker";
}
impl ::core::convert::From<InteractionTracker> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTracker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTracker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTracker> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionTracker> for super::CompositionObject {
    fn from(value: InteractionTracker) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTracker> for super::CompositionObject {
    fn from(value: &InteractionTracker) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionTracker {}
unsafe impl ::core::marker::Sync for InteractionTracker {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: InteractionTrackerClampingOption = InteractionTrackerClampingOption(0i32);
    pub const Disabled: InteractionTrackerClampingOption = InteractionTrackerClampingOption(1i32);
}
impl ::core::convert::From<i32> for InteractionTrackerClampingOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionTrackerClampingOption {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)");
}
impl ::windows::runtime::DefaultType for InteractionTrackerClampingOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTrackerCustomAnimationStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{8d1c8cf1-d7b0-434c-a5d2-2d7611864834})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8d1c8cf1_d7b0_434c_a5d2_2d7611864834);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerIdleStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerIdleStateEnteredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{50012faa-1510-4142-a1a5-019b09f8857b})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x50012faa_1510_4142_a1a5_019b09f8857b);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerInertiaModifier(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaModifier {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{a0e2c920-26b4-4da2-8b61-5e683979bbe2})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa0e2c920_26b4_4da2_8b61_5e683979bbe2);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaModifier {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerInertiaMotion(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaMotion {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Motion(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMotion<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<InteractionTrackerInertiaMotion> {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerInertiaMotion>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn IInteractionTrackerInertiaMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InteractionTrackerInertiaMotion, IInteractionTrackerInertiaMotionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{04922fdc-f154-4cb8-bf33-cc1ba611e6db})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x04922fdc_f154_4cb8_bf33_cc1ba611e6db);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaMotion {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerInertiaNaturalMotion(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaNaturalMotion {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalMotion(&self) -> ::windows::runtime::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ScalarNaturalMotionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetNaturalMotion<'a, Param0: ::windows::runtime::IntoParam<'a, super::ScalarNaturalMotionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<InteractionTrackerInertiaNaturalMotion> {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerInertiaNaturalMotion>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InteractionTrackerInertiaNaturalMotion, IInteractionTrackerInertiaNaturalMotionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{70acdaae-27dc-48ed-a3c3-6d61c9a029d2})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70acdaae_27dc_48ed_a3c3_6d61c9a029d2);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerInertiaRestingValue(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaRestingValue {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RestingValue(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetRestingValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<InteractionTrackerInertiaRestingValue> {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerInertiaRestingValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn IInteractionTrackerInertiaRestingValueStatics<R, F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InteractionTrackerInertiaRestingValue, IInteractionTrackerInertiaRestingValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{86f7ec09-5096-4170-9cc8-df2fe101bb93})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x86f7ec09_5096_4170_9cc8_df2fe101bb93);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerInertiaStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaStateEnteredArgs {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`, `Foundation_Numerics`*"]
    pub fn ModifiedRestingPosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn ModifiedRestingScale(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn NaturalRestingPosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalRestingScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsInertiaFromImpulse(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{87108cf2-e7ff-4f7d-9ffd-d72f1e409b63})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87108cf2_e7ff_4f7d_9ffd_d72f1e409b63);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerInteractingStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInteractingStateEnteredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTrackerInteractingStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{a7263939-a17b-4011-99fd-b5c24f143748})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa7263939_a17b_4011_99fd_b5c24f143748);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: InteractionTrackerPositionUpdateOption = InteractionTrackerPositionUpdateOption(0i32);
    pub const AllowActiveCustomScaleAnimation: InteractionTrackerPositionUpdateOption = InteractionTrackerPositionUpdateOption(1i32);
}
impl ::core::convert::From<i32> for InteractionTrackerPositionUpdateOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionTrackerPositionUpdateOption {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)");
}
impl ::windows::runtime::DefaultType for InteractionTrackerPositionUpdateOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerRequestIgnoredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerRequestIgnoredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{80dd82f1-ce25-488f-91dd-cb6455ccff2e})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80dd82f1_ce25_488f_91dd_cb6455ccff2e);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerValuesChangedArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerValuesChangedArgs {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{cf1578ef-d3df-4501-b9e6-f02fb22f73d0})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcf1578ef_d3df_4501_b9e6_f02fb22f73d0);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerVector2InertiaModifier(pub ::windows::runtime::IInspectable);
impl InteractionTrackerVector2InertiaModifier {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{87e08ab0-3086-4853-a4b7-77882ad5d7e3})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87e08ab0_3086_4853_a4b7_77882ad5d7e3);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(pub ::windows::runtime::IInspectable);
impl InteractionTrackerVector2InertiaNaturalMotion {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalMotion(&self) -> ::windows::runtime::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Vector2NaturalMotionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetNaturalMotion<'a, Param0: ::windows::runtime::IntoParam<'a, super::Vector2NaturalMotionAnimation>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<InteractionTrackerVector2InertiaNaturalMotion> {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerVector2InertiaNaturalMotion>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerVector2InertiaNaturalMotionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InteractionTrackerVector2InertiaNaturalMotion, IInteractionTrackerVector2InertiaNaturalMotionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{5f17695c-162d-4c07-9400-c282b28276ca})");
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f17695c_162d_4c07_9400_c282b28276ca);
}
impl ::windows::runtime::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerVector2InertiaModifier> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerVector2InertiaModifier> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VisualInteractionSource(pub ::windows::runtime::IInspectable);
impl VisualInteractionSource {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsPositionXRailsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsPositionYRailsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ManipulationRedirectionMode(&self) -> ::windows::runtime::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: VisualInteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VisualInteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetManipulationRedirectionMode(&self, value: VisualInteractionSourceRedirectionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionXChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionXChainingMode(&self, value: InteractionChainingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionXSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionYChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionYChainingMode(&self, value: InteractionChainingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionYSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetScaleChainingMode(&self, value: InteractionChainingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Visual>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Input`*"]
    pub fn TryRedirectForManipulation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Input::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(source: Param0) -> ::windows::runtime::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn DeltaPosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn DeltaScale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Numerics`*"]
    pub fn PositionVelocity(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleVelocity(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureCenterPointXModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureCenterPointYModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureDeltaPositionXModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureDeltaPositionYModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation_Collections`*"]
    pub fn ConfigureDeltaScaleModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PointerWheelConfig(&self) -> ::windows::runtime::Result<InteractionSourceConfiguration> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn CreateFromIVisualElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::IVisualElement>>(source: Param0) -> ::windows::runtime::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn IVisualInteractionSourceStatics<R, F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualInteractionSourceStatics2<R, F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VisualInteractionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.VisualInteractionSource;{ca0e8a86-d8d6-4111-b088-70347bd2b0ed})");
}
unsafe impl ::windows::runtime::Interface for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca0e8a86_d8d6_4111_b088_70347bd2b0ed);
}
impl ::windows::runtime::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.VisualInteractionSource";
}
impl ::core::convert::From<VisualInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: VisualInteractionSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &VisualInteractionSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VisualInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: VisualInteractionSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: &VisualInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionInteractionSource> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionInteractionSource> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionInteractionSource> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionInteractionSource> {
        ::core::convert::TryInto::<ICompositionInteractionSource>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<VisualInteractionSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<VisualInteractionSource> for super::CompositionObject {
    fn from(value: VisualInteractionSource) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&VisualInteractionSource> for super::CompositionObject {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for VisualInteractionSource {}
unsafe impl ::core::marker::Sync for VisualInteractionSource {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: VisualInteractionSourceRedirectionMode = VisualInteractionSourceRedirectionMode(0i32);
    pub const CapableTouchpadOnly: VisualInteractionSourceRedirectionMode = VisualInteractionSourceRedirectionMode(1i32);
    pub const PointerWheelOnly: VisualInteractionSourceRedirectionMode = VisualInteractionSourceRedirectionMode(2i32);
    pub const CapableTouchpadAndPointerWheel: VisualInteractionSourceRedirectionMode = VisualInteractionSourceRedirectionMode(3i32);
}
impl ::core::convert::From<i32> for VisualInteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisualInteractionSourceRedirectionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)");
}
impl ::windows::runtime::DefaultType for VisualInteractionSourceRedirectionMode {
    type DefaultType = Self;
}
